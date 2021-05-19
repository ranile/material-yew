use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

use pulldown_cmark::Event;
use pulldown_cmark::Options;
use pulldown_cmark::Parser;
use pulldown_cmark::Tag;
use pulldown_cmark::{html, CodeBlockKind, CowStr};

fn main() {
    println!("cargo:rerun-if-changed=../README.md");

    let markdown_input = include_str!("../README.md");
    let markdown_input = markdown_input.replace("# Material Yew", "");

    // Set up options and parser. Strikethroughs are not part of the CommonMark
    // standard and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&markdown_input, options);

    let mut new_parser = Vec::new();
    let mut to_highlight = String::new();
    let mut in_code_block = false;

    let mut lang = "".to_string();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(c)) => {
                if let CodeBlockKind::Fenced(block_lang) = c {
                    let block_lang = block_lang.to_string();
                    lang = if block_lang == *"rust" {
                        "rs".to_string()
                    } else {
                        block_lang
                    }
                };
                // In actual use you'd probably want to keep track of what language this code is
                in_code_block = true;
            }
            Event::End(Tag::CodeBlock(_)) => {
                if in_code_block {
                    // Format the whole multi-line code block as HTML all at once
                    let ext = lang.trim();
                    let syntax_set = match ext {
                        "html" => SyntaxSet::load_defaults_newlines(),
                        "rs" | "toml" => {
                            syntect::dumps::from_binary(include_bytes!(
                                "syntect-dumps/syntax-set.syntax"
                            ))
                        }
                        _ => panic!("language other than html, toml, rust, used"),
                    };

                    let syntax = syntax_set
                        .find_syntax_by_extension(ext)
                        .unwrap_or_else(|| panic!("{}", ext));
                    let theme = syntect::dumps::from_binary(include_bytes!(
                        "syntect-dumps/Material-Theme-Lighter.theme"
                    ));
                    let html =
                        highlighted_html_for_string(&to_highlight, &syntax_set, &syntax, &theme);
                    // And put it into the vector
                    new_parser.push(Event::Html(CowStr::from(html)));
                    to_highlight = String::new();
                    in_code_block = false;
                }
            }
            Event::Text(t) => {
                if in_code_block {
                    // If we're in a code block, build up the string of text
                    to_highlight.push_str(&t);
                } else {
                    new_parser.push(Event::Text(t))
                }
            }
            e => {
                new_parser.push(e);
            }
        }
    }

    let mut html_output = String::with_capacity(markdown_input.len() * 3 / 2);
    html::push_html(&mut html_output, new_parser.into_iter());

    std::fs::write(
        "readme.html",
        format!(r#"<section class="home-content">{}</section>"#, html_output),
    )
    .unwrap();
}
