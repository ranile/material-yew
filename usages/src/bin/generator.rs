use eyre::{eyre, WrapErr};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use std::fmt::Write as _;
use std::fs::File;
use std::io::Write as _;
use syn::{Field, GenericArgument, Item, parse_quote, PathArguments, Type, TypePath};

const INDENT_SIZE: usize= 4;

#[derive(Debug)]
enum SpecialPropTypes {
    Boolean,
    AttrValue,
    Int,
    Float,
    Callback,
}

impl SpecialPropTypes {
    fn from_type(ty: &Type) -> Result<SpecialPropTypes, &TypePath> {
        if let Type::Path(path) = ty {
            let ty = path.path.segments.iter().next().unwrap().ident.to_string();
            match ty.as_str() {
                "bool" => Ok(Self::Boolean),
                "AttrValue" => Ok(Self::AttrValue),
                "i32" | "i64" | "u64" | "u32" => Ok(Self::Int),
                "f32" | "f64" => Ok(Self::Float),
                "Callback" => Ok(Self::Callback),
                _ => Err(path),
            }
        } else {
            panic!("Unsupported type")
        }
    }
}

fn get_example_value_from_prop(field: &Field) -> Result<TokenStream, &TypePath> {
    // let ty = field.ty.to_token_stream().to_string();
    // if ty.contains("JsValue")
    //     || field.attrs.iter().any(|attr| {
    //         attr.path().is_ident("prop_or")
    //             && attr.to_token_stream().to_string().contains("JsValue")
    //     })
    // {
    //     return Ok(quote! { JsValue::default() });
    // }
    let ident = field.ident.as_ref().expect("must have an ident");

    let special_type = match &field.ty {
        Type::Path(path) if path.path.segments.iter().any(|seg| seg.ident == "Callback") => {
            handle_type_path_segment(path).map(|it| (ident, it))
        }
        Type::Path(path) if path.path.segments.iter().any(|seg| seg.ident == "Option") => {
            handle_type_path_segment(path).map(|it| (ident, it))
        }
        _ => SpecialPropTypes::from_type(&field.ty).map(|it| (ident, it)),
    };

    special_type.map(|(ident, special_type)| match special_type {
        SpecialPropTypes::Boolean => {
            quote! { false }
        }
        SpecialPropTypes::AttrValue => {
            let value = format!("<some {ident}>");
            quote! { AttrValue::Static(#value) }
        }
        SpecialPropTypes::Int => {
            quote! { 0 }
        }
        SpecialPropTypes::Float => {
            quote! { 0.0 }
        }
        SpecialPropTypes::Callback => {
            quote! { Callback::noop() }
        }
    })
}

fn handle_type_path_segment(path: &TypePath) -> Result<SpecialPropTypes, &TypePath> {
    assert_eq!(
        path.path.segments.len(),
        1,
        "import the type; paths not supported"
    );
    let ty = &path.path.segments.iter().next().unwrap().arguments;
    let PathArguments::AngleBracketed(args) = ty else {
        panic!("Expected angle bracketed arguments")
    };
    let GenericArgument::Type(arg) = args.args.iter().next().unwrap() else {
        panic!("Expected a type argument");
    };
    SpecialPropTypes::from_type(arg)
}

fn make_example_usage(file: syn::File, file_name: &str) -> Result<(Ident, String), eyre::Error> {
    let mut prop_values = vec![];
    let mut comp_name = None;
    let mut has_children = false;

    for item in file.items.iter() {
        match item {
            Item::Struct(s) if s.ident == "Props" => {
                for field in s.fields.iter() {
                    if field
                        .ident
                        .as_ref()
                        .ok_or_else(|| eyre!("expected children"))?
                        == "children"
                    {
                        has_children = true;
                        continue;
                    }
                    match get_example_value_from_prop(field) {
                        Ok(ty) => {
                            prop_values.push((field.ident.as_ref().unwrap(), ty));
                        }
                        Err(path) => {
                            if path
                                .path
                                .segments
                                .to_token_stream()
                                .to_string()
                                .contains("web_sys")
                            {
                                // web-sys path; don't know how to handle
                                continue;
                            }
                            let path = path.path.get_ident().ok_or_else(|| {
                                eyre::eyre!(
                                    "Expected an ident, found {}",
                                    path.to_token_stream().to_string()
                                )
                            })?;
                            let item = file.items.iter().find(|&item| match item {
                                Item::Enum(e) if e.ident == *path => true,
                                Item::Struct(s) if s.ident == *path => true,
                                _ => false,
                            });
                            match item {
                                Some(Item::Struct(s)) => prop_values
                                    .push((field.ident.as_ref().unwrap(), quote! {#s::default()})),
                                Some(Item::Enum(e)) => {
                                    let ident = &e.ident;
                                    let variant =
                                        e.variants.first().expect("expected enum to have variants");
                                    prop_values.push((
                                        field.ident.as_ref().unwrap(),
                                        quote! {#ident::#variant},
                                    ))
                                }
                                _ => continue,
                            }
                        }
                    }
                }
            }
            Item::Fn(func)
            if func
                .attrs
                .iter()
                .any(|it| it.path().is_ident("function_component")) =>
                {
                    comp_name = Some(func.sig.ident.clone());
                }
            _ => {}
        }
    }

    let comp_name = comp_name.ok_or_else(|| eyre!("component not found in file {file_name}"))?;

    let mut output = String::new();
    write!(output, "<{comp_name}")?;
    for (ident, value) in prop_values {
        let value = format!("{ident}={{{value}}}");
        // fix token stream formatting
        let value = value.replace("AttrValue :: Static (", "AttrValue::Static(");
        write!(output, "\n{}", indent(&value, 1))?;
    }
    writeln!(output, "\n>")?;
    if has_children {
        writeln!(output, "{}", indent("{children}", 1))?;
    }
    write!(output, "</{comp_name}>")?;

    Ok((comp_name, output))
}

fn main() -> eyre::Result<()> {
    let mut output = File::create("usages/src/usage.rs")?;
    writeln!(
        output,
        "// This file is generated by the build script. Do not edit."
    )?;
    writeln!(output, "use yew::{{AttrValue, Callback, html, Html}};")?;
    writeln!(output, "use material_yew::*;")?;
    writeln!(output)?;

    let mut components = Vec::with_capacity(16);

    let dir = std::fs::read_dir("src").wrap_err("read src directory")?;
    for entry in dir {
        let entry = entry.wrap_err("read entry")?;
        let path = entry.path();
        if path.extension().unwrap_or_default() != "rs" {
            continue;
        }
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        if file_name == "main.rs" || file_name == "lib.rs" || file_name == "test.rs" {
            continue;
        }
        let file = std::fs::read_to_string(&path)
            .wrap_err_with(|| format!("Unable to read file: {}", path.to_string_lossy()))?;
        let file = syn::parse_file(&file).wrap_err("Unable to parse file")?;
        let (comp_name, usage) = make_example_usage(file, file_name)?;
        let children = if usage.contains("{children}") {
            "children: Html"
        } else {
            ""
        };
        writeln!(
            output,
            "pub fn test_{}({children}) -> Html {{",
            file_name.strip_suffix(".rs").unwrap_or_default()
        )?;
        let usage = format!("html! {{\n{}\n}}", indent(usage.trim(), 1));
        writeln!(output, "{}", indent(&usage, 1))?;
        writeln!(output, "}}\n\n")?;

        components.push(comp_name);
    }
    drop(output);

    let routes = components.into_iter().map(|comp| {
        let at = format!("/{}", comp.to_string().to_lowercase());
        quote! {
            #[at(#at)]
            #comp
        }
    });
    let routes_file: syn::File = parse_quote! {
        use yew_router::prelude::*;
        #[derive(Routable, PartialEq, Eq, Clone, Debug)]
        pub enum Route {
            #(#routes),*
        }
    };
    std::fs::write("usages/src/route.rs", prettyplease::unparse(&routes_file))?;

    Ok(())
}

fn indent(s: &str, times: usize) -> String {
    textwrap::indent(s, &" ".repeat(INDENT_SIZE * times))
}
