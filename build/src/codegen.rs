use crate::ty::Type;
use crate::{utils, Component, Property};
use convert_case::{Case, Casing};
use eyre::ensure;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn gen_component(component_name: &str, mut variants: Vec<Component>) -> String {
    if variants.len() == 1 {
        let mut component = variants.pop().unwrap();
        assert_eq!(format!("md-{component_name}"), component.name);
        let mut properties = &mut component.properties;
        if properties.len() == 1 && properties[0].ty.0.ends_with("[]") {
            properties.clear();
        }
        properties.retain(|it| {
            // MenuItem
            match &it.ty.0.strip_suffix("[]") {
                Some("MenuItem") => false,
                Some(_) => true,
                None => true,
            }
        });
        // println!("{:?}", properties);
        let props = gen_properties(&properties, true).to_string();
        let to_html = gen_to_html(&component);

        let module_dir = format!("/md-web/{}.js", component_name);
        let component_name = component_name.to_case(Case::Pascal);
        format!(
            r#"
use yew::prelude::*;
{props}

#[function_component]
pub fn {component_name}(props: &Props) -> Html {{
    use_effect_with((), |_| crate::import_material_web_module!("{module_dir}"));
    {to_html}
}}
        "#
        )
    } else {
        let all_variants_same = |variants: &[Component]| {
            let first = &variants[0];
            variants
                .iter()
                .all(|item| *item.properties == *first.properties)
        };
        let same = all_variants_same(&variants);
        if !same {
            let skip = match component_name {
                "chip" => 1,
                _ => todo!(
                    "{} has multiple variants, but they are not all the same",
                    component_name
                ),
            };
            variants = variants.into_iter().skip(skip).collect();
        }

        // all this point, all the properties are safe to be joined together
        let mut properties = Vec::new();
        for variant in &variants {
            for prop in &variant.properties {
                if !properties.contains(prop) {
                    properties.push(prop.clone());
                }
            }
        }

        let html = gen_dynamic_tagged_html(variants[0].properties.as_slice());
        let (variants_enum, variants_property) = gen_variants_enum(component_name, &mut variants);

        let component = variants.first_mut().unwrap();
        component.properties.push(variants_property);
        let props =
            utils::format_tokens(gen_properties(&component.properties, true).to_string()).unwrap();

        let module_dir = format!("/md-web/{}.js", component_name);
        let component_name = component_name.to_case(Case::Pascal);

        format!(
            r#"
use yew::prelude::*;

{variants_enum}
{props}

#[function_component]
pub fn {component_name}(props: &Props) -> Html {{
    use_effect_with((), |_| crate::import_material_web_module!("{module_dir}"));
    {html}
}}
        "#
        )
    }
}

fn gen_dynamic_tagged_html(properties: &[Property]) -> String {
    let mut output = format!("html! {{ <@{{props.variant.as_tag_name()}} \n");
    for property in properties {
        gen_prop_assignment(&mut output, property);
    }

    output.push_str("> ");
    output.push_str("{props.children.clone()}");
    output.push_str(" </@> }");
    output
}

fn gen_variants_enum(component_name: &str, variants: &Vec<Component>) -> (String, Property) {
    let ty = Type(format!("{}Variants", component_name.to_case(Case::Pascal)));
    let mut variants_enum = format!("#[derive(PartialEq)] \n pub enum {} {{ \n", ty.0);

    let enum_variants = variants
        .iter()
        .map(|variant| {
            let variant_name = variant.name.strip_prefix("md-").unwrap();
            let variant_name = variant_name
                .strip_suffix(&format!("-{component_name}"))
                .unwrap_or(variant_name);
            let variant_name = if variant_name == component_name {
                "Standard"
            } else {
                variant_name
            }
            .to_case(Case::Pascal);
            (variant_name, variant.name.clone())
        })
        .collect::<Vec<_>>();

    for (variant_name, _) in &enum_variants {
        variants_enum.push_str(&variant_name);
        variants_enum.push(',');
        variants_enum.push('\n');
    }
    variants_enum.push('}');

    variants_enum.push_str("\n\nimpl ");
    variants_enum.push_str(&ty.0);
    variants_enum.push_str("{\n");
    variants_enum.push_str("fn as_tag_name(&self) -> &'static str {\n");
    variants_enum.push_str("match self {\n");
    for (variant_name, tag_name) in &enum_variants {
        variants_enum.push_str("            ");
        variants_enum.push_str(&ty.0);
        variants_enum.push_str("::");
        variants_enum.push_str(&variant_name);
        variants_enum.push_str(" => \"");
        variants_enum.push_str(&tag_name);
        variants_enum.push_str("\",\n");
    }
    variants_enum.push_str("}\n");
    variants_enum.push_str("}\n");
    variants_enum.push_str("}\n");

    let variants_enum = utils::format_tokens(variants_enum).unwrap();

    (
        variants_enum,
        Property {
            property: "variant".to_string(),
            attribute: None,
            ty,
            default: None,
            description: Some("The variant to use.".to_string()),
        },
    )
}

fn gen_properties(properties: &[Property], add_children: bool) -> TokenStream {
    let mut tokens = TokenStream::new();
    for property in properties {
        tokens.extend(property.to_tokens());
    }
    if add_children {
        tokens.extend(quote! {
            pub children: Html,
        });
    }
    quote! {
        #[derive(Properties, PartialEq)]
        pub struct Props {
            #tokens
        }
    }
}

fn gen_to_html(component: &Component) -> String {
    let name = &component.name;
    let mut output = format!("html! {{ <{} \n", name);
    component.properties.iter().for_each(|p| {
        gen_prop_assignment(&mut output, &p);
    });
    output.push_str(" /> }");
    output
}

fn gen_prop_assignment(output: &mut String, p: &Property) {
    output.push_str("\t");
    if !(p.property == "disabled" || p.property == "required" || p.property == "value") {
        output.push('~')
    }
    output.push_str(&p.property);
    output.push_str("={");
    let value = p.property.to_case(Case::Snake).replace("ty", "type");
    if p.property == "disabled" || p.property == "required" {
        output.push_str("props.");
        output.push_str(&value);
        output.push_str(".unwrap_or_default()");
    } else if p.property == "value" {
        output.push_str("props.");
        output.push_str(&value);
        output.push_str(".clone().unwrap_or_default()");
    } else if p.ty.is_string() {
        output.push_str("crate::js_value_from_string_or_null(props.");
        output.push_str(&value);
        output.push_str(".as_ref())");
    } else {
        output.push_str("crate::js_value_or_null(props.");
        output.push_str(&value);
        output.push_str(".clone())");
    };
    output.push_str("}\n");
}

impl Property {
    fn to_tokens(&self) -> TokenStream {
        let name = Ident::new(
            &self.property.to_case(Case::Snake).replace("ty", "type"),
            Span::call_site(),
        );
        let description = self
            .description
            .as_ref()
            .map(|d| quote! { #[doc = #d] })
            .unwrap_or_default();
        let inner_ty = self.ty.as_ident();
        let (ty, attr) = if self.default.is_some() {
            (
                quote! { Option<#inner_ty> },
                quote! {
                    #[prop_or_default]
                },
            )
        } else {
            (quote! { #inner_ty }, quote!())
        };
        quote! {
            #description
            #attr
            pub #name: #ty,
        }
    }
}
