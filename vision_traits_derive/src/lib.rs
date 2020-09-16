mod configurable;
mod input;
mod output;

use configurable::configurable_impl;
use input::input_impl;
use output::output_impl;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Attribute, DeriveInput, Error, Field, Lit, Meta};

#[proc_macro_derive(Configurable, attributes(name))]
pub fn configurable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    proc_macro::TokenStream::from(configurable_impl(&ast))
}

#[proc_macro_derive(Input, attributes(name, input_lifetime))]
pub fn input(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    proc_macro::TokenStream::from(input_impl(&ast))
}

#[proc_macro_derive(Output, attributes(name))]
pub fn output(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    proc_macro::TokenStream::from(output_impl(&ast))
}

fn field_name(field: &Field) -> proc_macro2::TokenStream {
    let name_attrs = field
        .attrs
        .iter()
        .filter(|x| x.path.is_ident("name"))
        .collect::<Vec<&Attribute>>();

    if name_attrs.is_empty() {
        let name_string = field.ident.as_ref().unwrap().to_string();
        quote! { #name_string }
    } else if name_attrs.len() == 1 {
        let name_attr = name_attrs.first().unwrap();

        if let Ok(ref meta) = name_attr.parse_meta() {
            if let Meta::NameValue(ref value) = meta {
                if let Lit::Str(ref lit_str) = value.lit {
                    let name_string = lit_str.value();
                    return quote! { #name_string };
                }
            }
        }

        Error::new(
            name_attrs.first().span(),
            "Invalid name attribute format, expected: #[name(\"foo\")]",
        )
        .to_compile_error()
    } else {
        Error::new(
            name_attrs.first().span(),
            "Multiple name attributes on one field",
        )
        .to_compile_error()
    }
}
