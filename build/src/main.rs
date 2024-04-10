mod codegen;
mod ty;
mod utils;

use std::collections::HashMap;
use std::path::Path;

use color_eyre::Section;
use convert_case::{Case, Casing};
use eyre::{bail, ensure, eyre, ContextCompat, Report};
use pulldown_cmark::{CowStr, Event, HeadingLevel, Options, Parser, Tag};
use serde::Deserialize;

use crate::ty::Type;
use crate::utils::IterExt;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Property {
    pub property: String,
    pub attribute: Option<String>,
    #[serde(alias = "Type")]
    pub ty: Type,
    pub default: Option<String>,
    pub description: Option<String>,
}

impl Property {
    fn from_maps(value: &Vec<HashMap<CowStr, String>>) -> Result<Vec<Property>, Report> {
        let value = serde_json::to_value(&value)?;
        let data = serde_json::from_value::<Vec<Property>>(value)?;
        Ok(data)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExtractedEvent {
    #[serde(rename = "Event")]
    pub name: String,
    pub description: Option<String>,
}

impl ExtractedEvent {
    fn from_maps(value: &Vec<HashMap<CowStr, String>>) -> Result<Vec<ExtractedEvent>, Report> {
        let value = serde_json::to_value(&value)?;
        let data = serde_json::from_value::<Vec<ExtractedEvent>>(value)?;
        Ok(data)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Component {
    pub name: String,
    pub properties: Vec<Property>,
    pub events: Vec<ExtractedEvent>,
}

const MATERIAL_WEB_DIR: &str = "material-web";

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let mut args = std::env::args().into_iter().skip(1);
    let component_name = args
        .next()
        .ok_or_else(|| eyre!("expected component name"))?;
    let handle_each_component_individually =
        args.next().and_then(|it| it.parse().ok()).unwrap_or(false);

    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(MATERIAL_WEB_DIR)
        .join("docs")
        .join("components")
        .join(format!("{}.md", component_name));

    let doc = std::fs::read_to_string(&path)
        .with_note(|| format!("failed to read file {}", path.display()))?;
    let parser = Parser::new_ext(&doc, Options::all());

    let iter = parser
        .skip_while(|event| !matches!(event, Event::Html(s) if s.trim() == "<!-- auto-generated API docs start -->"))
        .skip(4);

    let c = iter
        .chunk_with(|event| matches!(event, Event::Start(Tag::Heading(HeadingLevel::H3, _, _))));
    let components = c
        .into_iter()
        .map(process_variant)
        .collect::<eyre::Result<Vec<Component>>>()?;
    if handle_each_component_individually {
        for component in components {
            let component_name = component.name.strip_prefix("md-").unwrap().to_owned();
            let component = codegen::gen_component(&component_name, vec![component]);
            let component = utils::format_tokens(component)?;
            std::fs::write(
                format!(
                    "{}/../src/{}.rs",
                    env!("CARGO_MANIFEST_DIR"),
                    component_name.to_case(Case::Snake)
                ),
                component,
            )
            .unwrap();
        }
    } else {
        let component = codegen::gen_component(&component_name, components);
        let component = utils::format_tokens(component)?;
        std::fs::write(
            format!(
                "{}/../src/{}.rs",
                env!("CARGO_MANIFEST_DIR"),
                component_name.to_case(Case::Snake)
            ),
            component,
        )
        .unwrap();
    }
    Ok(())
}

fn process_variant(doc: Vec<Event>) -> eyre::Result<Component> {
    let mut iter = doc.into_iter();
    let first = iter.next();
    ensure!(
        matches!(
            first,
            Some(Event::Start(Tag::Heading(HeadingLevel::H3, _, _)))
        ),
        "expected H3 as first element"
    );

    let Event::Text(component_name) = iter
        .by_ref()
        .take_while(|e| !matches!(e, Event::End(Tag::Heading(HeadingLevel::H3, _, _))))
        // this collect is needed because otherwise elements after find will stay in the iterator
        .collect::<Vec<_>>()
        .into_iter()
        .find(|e| matches!(e, Event::Text(t) if t.starts_with("md-")))
        .wrap_err("failed to find extract component name from the heading")?
    else {
        bail!("expected heading to be Text")
    };

    let h4s = iter.chunk_with(|e| matches!(e, Event::Start(Tag::Heading(HeadingLevel::H4, _, _))));

    let properties = extract_data_from_md_table(&mut h4s[0].iter().cloned(), "Properties")?;
    let events = h4s
        .get(2)
        .or_else(|| h4s.get(1))
        .map(|h| extract_data_from_md_table(&mut h.iter().cloned(), "Events"))
        .transpose()?
        .unwrap_or_default();

    let component = Component {
        name: component_name.to_string(),
        properties: Property::from_maps(&properties)?,
        events: ExtractedEvent::from_maps(&events)?,
    };
    Ok(component)
}

fn extract_data_from_md_table<'a>(
    mut iter: impl Iterator<Item = Event<'a>>,
    heading: &str,
) -> eyre::Result<Vec<HashMap<CowStr<'a>, String>>> {
    let mut entries = vec![];
    let mut head = iter
        .by_ref()
        .take_while(|e| !matches!(e, Event::End(Tag::TableHead)))
        .collect::<Vec<_>>();
    if !matches!(head.iter().nth(1), Some(Event::Text(t)) if *t == CowStr::Borrowed(heading)) {
        if heading == "Properties" {
            bail!("expected heading to be {}", heading)
        } else if heading == "Events" {
            // no events, only methods
            return Ok(entries);
        }
    }
    let first_cell = head
        .iter()
        .position(|e| matches!(e, Event::Start(Tag::TableCell)))
        .unwrap();
    // remove "#### Properties"
    head.drain(0..first_cell);
    let head = head
        .into_iter()
        .chunk_with(|e| matches!(e, Event::Start(Tag::TableCell)))
        .into_iter()
        .map(|cell| {
            let [_, Event::Text(text), _] = &cell[..] else {
                bail!("expected 3 elements (Start, Text, End) defining a header table cell")
            };
            Ok(text.clone())
        })
        .collect::<eyre::Result<Vec<_>>>()?;

    let rows = iter.chunk_with(|e| matches!(e, Event::Start(Tag::TableRow)));
    for mut row in rows {
        let pos = row
            .iter()
            .rposition(|e| matches!(e, Event::End(Tag::TableRow)));
        // remove extra markdown
        if let Some(pos) = pos {
            row.truncate(pos)
        }
        row.remove(0);
        let mut properties = HashMap::new();
        for (cell, header) in row
            .iter()
            .chunk_with(|e| matches!(e, Event::Start(Tag::TableCell)))
            .iter()
            .zip(head.iter())
        {
            match &cell[..] {
                [Event::Start(Tag::TableCell), contents @ .., Event::End(Tag::TableCell)] => {
                    let mut buf = String::new();
                    if *header == CowStr::Borrowed("Description") {
                        pulldown_cmark::html::push_html(
                            &mut buf,
                            contents.iter().map(|e| (*e).clone()),
                        );
                    } else {
                        for txt in contents {
                            match txt {
                                Event::Text(t) => buf.push_str(t),
                                Event::Code(t) => buf.push_str(t),
                                actual => bail!(
                                    "expected Text or Code, found {:?} as value for {header}",
                                    actual
                                ),
                            }
                        }
                    }
                    properties.insert(header.clone(), buf);
                }
                c => bail!(
                    "expected at least 2 elements (Start, .., End) defining a table cell; found \
                     {c:?}"
                ),
            }
        }
        entries.push(properties)
    }
    Ok(entries)
}
