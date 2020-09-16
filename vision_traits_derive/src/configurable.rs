use crate::field_name;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, DeriveInput, Error, Fields};

pub fn configurable_impl(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let ident = &ast.ident;

    match ast.data {
        Data::Struct(ref struct_data) => {
            let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

            match struct_data.fields {
                Fields::Named(ref fields) => {
                    if !fields.named.is_empty() {
                        let name_map = fields
                            .named
                            .iter()
                            .zip(fields.named.iter().map(&field_name))
                            .collect::<Vec<_>>();

                        let schema = name_map.iter().map(|(f, name)| {
                            let ty = &f.ty;
                            quote_spanned! {f.ident.span() =>
                                map.insert(#name.to_owned(), <#ty as ::vision_traits::editable::Editable>::schema());
                            }
                        });

                        let deserialize = name_map.iter().map(|(f, name)| {
                            let ident = f.ident.as_ref();
                            let ty = &f.ty;
                            quote_spanned! {f.ident.span() =>
                                #ident: <#ty as ::vision_traits::editable::Editable>::deserialize(
                                    map.get(#name).ok_or_else(|| ::vision_traits::DeserializationError::MissingField(#name.to_owned()))?
                                )
                                .map_err(|x| ::vision_traits::DeserializationError::FieldDeserializationError(#name.to_owned(), x))?
                            }
                        });

                        quote! {
                            impl #impl_generics Configurable for #ident #ty_generics #where_clause {
                                fn schema() -> ::std::collections::HashMap<::std::string::String, ::vision_traits::schema::SettingType> {
                                    let mut map = ::std::collections::HashMap::new();
                                    #(#schema)*
                                    map
                                }

                                fn deserialize(input: &str) -> ::std::result::Result<Self, ::vision_traits::DeserializationError> {
                                    let json = ::vision_traits::json::parse(input).map_err(|_| ::vision_traits::DeserializationError::NotObject)?;
                                    if let ::vision_traits::json::JsonValue::Object(ref map) = json {
                                        Ok(Self { #(#deserialize),* })
                                    } else {
                                        Err(::vision_traits::DeserializationError::NotObject)
                                    }
                                }
                            }
                        }
                    } else {
                        quote! {
                            impl #impl_generics Configurable for #ident #ty_generics #where_clause {
                                fn schema() -> ::std::collections::HashMap<::std::string::String, ::vision_traits::schema::SettingType> {
                                    ::std::collections::HashMap::new()
                                }
                                fn deserialize(input: &str) -> ::std::result::Result<Self, ::vision_traits::DeserializationError> {
                                    Ok(Self{})
                                }
                            }
                        }
                    }
                }

                Fields::Unit => {
                    quote! {
                        impl #impl_generics Configurable for #ident #ty_generics #where_clause {
                            fn schema() -> ::std::collections::HashMap<::std::string::String, ::vision_traits::schema::SettingType> {
                                ::std::collections::HashMap::new()
                            }
                            fn deserialize(input: &str) -> ::std::result::Result<Self, ::vision_traits::DeserializationError> {
                                Ok(Self)
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
