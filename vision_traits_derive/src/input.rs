use crate::field_name;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Attribute, Data, DeriveInput, Error, Fields, Lifetime, Type};

pub fn input_impl(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let ident = &ast.ident;

    match ast.data {
        Data::Struct(ref struct_data) => {
            let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

            match struct_data.fields {
                Fields::Named(ref fields) => {
                    if !fields.named.is_empty() {
                        let trait_lifetime = match lifetime_name(ast) {
                            Ok(stream) => stream,
                            Err(err) => return err,
                        };

                        let name_map = fields
                            .named
                            .iter()
                            .zip(fields.named.iter().map(&field_name))
                            .collect::<Vec<_>>();

                        let from_any_map = name_map.iter().map(|(f, name)| {
                            let ident = f.ident.as_ref();
                            let ty = &f.ty;

                            if let Type::Reference(ref ty) = ty {
                                let ty = &ty.elem;
                                quote_spanned! {f.ident.span() =>
                                    #ident: map.get(#name).ok_or_else(|| ::vision_traits::DeserializationError::MissingField(#name.to_owned()))?.downcast_ref::<#ty>().ok_or_else(|| ::vision_traits::DeserializationError::TypeError(#name.to_owned()))?,
                                }
                            } else {
                                quote_spanned! {f.ty.span() =>
                                    #ident: compile_error!("Expected reference type")
                                }
                            }
                        });

                        let schema = name_map.iter().map(|(f, name)| {
                            let ty = &f.ty;

                            if let Type::Reference(ref ty) = ty {
                                let ty = &ty.elem;
                                quote_spanned! {f.ident.span() =>
                                    map.insert(#name.to_owned(), ::vision_traits::Type { name: ::std::any::type_name::<#ty>().to_owned() });
                                }
                            } else {
                                // compile_error already inserted above
                                quote! {
                                }
                            }
                        });

                        quote! {
                            impl #impl_generics Input<#trait_lifetime> for #ident #ty_generics #where_clause {
                                fn from_any_map(map: &#trait_lifetime ::std::collections::HashMap<::std::string::String, &dyn ::std::any::Any>) -> ::std::result::Result<Self, ::vision_traits::DeserializationError>  {
                                    Some(Self {#(#from_any_map),*})
                                }

                                fn schema() -> ::std::collections::HashMap<&::std::string::String, ::vision_traits::Type> {
                                    let mut map = ::std::collections::HashMap::new();
                                    #(#schema)*
                                    map
                                }
                            }
                        }
                    } else {
                        quote! {
                            impl #impl_generics Input<'_> for #ident #ty_generics #where_clause {
                                fn from_any_map(_: &::std::collections::HashMap<::std::string::String, &dyn ::std::any::Any>) -> ::std::result::Result<Self, ::vision_traits::DeserializationError> {
                                    Some(Self{})
                                }

                                fn schema() -> ::std::collections::HashMap<::std::string::String, ::vision_traits::Type> {
                                    ::std::collections::HashMap::new()
                                }
                            }
                        }
                    }
                }

                Fields::Unit => {
                    quote! {
                        impl #impl_generics Input<'_> for #ident #ty_generics #where_clause {
                            fn from_any_map(_: &::std::collections::HashMap<::std::string::String, &dyn ::std::any::Any>) -> ::std::result::Result<Self, ::vision_traits::DeserializationError> {
                                Some(Self)
                            }

                            fn schema() -> ::std::collections::HashMap<::std::string::String, ::vision_traits::Type> {
                                ::std::collections::HashMap::new()
                            }
                        }
                    }
                }

                Fields::Unnamed(_) => {
                    Error::new(ident.span(), "Expected named struct, not tuple struct")
                        .to_compile_error()
                }
            }
        }
        Data::Enum(_) => Error::new(ident.span(), "Expected struct, not enum").to_compile_error(),
        Data::Union(_) => Error::new(ident.span(), "Expected struct, not union").to_compile_error(),
    }
}

fn lifetime_name(ast: &DeriveInput) -> Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
    let name_attrs = ast
        .attrs
        .iter()
        .filter(|x| x.path.is_ident("input_lifetime"))
        .collect::<Vec<&Attribute>>();

    if name_attrs.is_empty() {
        Ok(quote! { 'a })
    } else if name_attrs.len() == 1 {
        let name_attr = name_attrs.first().unwrap();
        let lifetime = name_attr.parse_args::<Lifetime>();

        if let Ok(ref lifetime) = lifetime {
            Ok(quote! { #lifetime })
        } else {
            Err(Error::new(
                name_attrs.first().span(),
                "Invalid input_lifetime attribute format, expected: #[input_lifetime('a)]",
            )
            .to_compile_error())
        }
    } else {
        Err(Error::new(
            name_attrs.first().span(),
            "Multiple input_lifetime attributes on one struct",
        )
        .to_compile_error())
    }
}
