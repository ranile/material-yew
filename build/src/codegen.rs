use crate::{Component, Property};
use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use crate::ty::Type;

fn format_tokens(tokens: String) -> String {
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    thread::spawn(move || {
        stdin
            .write_all(tokens.as_bytes())
            .expect("Failed to write to stdin");
    });
    let output = child.wait_with_output().expect("Failed to read stdout");
    let tokens = String::from_utf8(output.stdout).expect("Failed to convert output to string");

    tokens
}

pub fn gen_component(component_name: &str, mut variants: Vec<Component>) {
    if variants.len() == 1 {
        let component = variants.pop().unwrap();
        assert_eq!(format!("md-{component_name}"), component.name);
        let props = format_tokens(gen_properties(&component, true).to_string());
        let to_html = gen_to_html(&component);

        let component_name = component_name.to_case(Case::Pascal);
        println!(r#"
{props}

#[function_component]
fn {component_name}(props: &Props) -> Html {{
    {to_html}
}}
        "#)
    } else {
        let first = &variants[0];
        assert!(variants;
            .iter()
            .all(|item| *item.properties == *first.properties));

        // all variants are the same, just use first
        let html = {
            let mut output = format!("html! {{ <@{{props.variant.as_tag_name()}} \n");
            for property in &first.properties {
                gen_prop_assignment(&mut output, &property);
            }

            output.push_str("> ");
            output.push_str("{props.children.clone()}");
            output.push_str(" </@> }");
            output
        };

        let (props, variants_enum) = {
            let ty = Type(format!("{}Variants", component_name.to_case(Case::Pascal)));
            let mut variants_enum = format!("#[derive(PartialEq)] \n pub enum {} {{ \n", ty.0);

            let enum_variants = variants.iter().map(|variant| {
                let variant_name = variant
                    .name
                    .strip_prefix("md-")
                    .and_then(|s| s.strip_suffix(&format!("-{component_name}")))
                    .unwrap()
                    .to_case(Case::Pascal);
                (variant_name, variant.name.clone())
            }).collect::<Vec<_>>();

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

            let variants_enum = format_tokens(variants_enum);

            let component = variants.first_mut().unwrap();
            component.properties.push(Property {
                property: "variant".to_string(),
                attribute: None,
                ty,
                default: None,
                description: Some("The variant to use.".to_string()),
            });
            let props = format_tokens(gen_properties(&component, true).to_string());
            (props, variants_enum)
        };

        let component_name = component_name.to_case(Case::Pascal);
        println!(r#"
{variants_enum}
{props}

#[function_component]
pub fn {component_name}(props: &Props) -> Html {{
    {html}
}}
        "#)

    }
}

fn gen_properties(component: &Component, add_children: bool) -> TokenStream {
    let mut tokens = TokenStream::new();
    for property in &component.properties {
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

fn gen_prop_assignment(output: &mut String, p: &&Property) {
    output.push_str("\t");
    if !(p.property == "disabled" || p.property == "required") {
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
        output.push_str("js_value_from_string_or_null(props.");
        output.push_str(&value);
        output.push_str(".as_ref())");
    } else {
        output.push_str("js_value_or_null(props.");
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
