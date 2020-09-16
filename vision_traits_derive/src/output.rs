use crate::field_name;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, DeriveInput, Error, Fields};

pub fn output_impl(ast: &DeriveInput) -> proc_macro2::TokenStream {
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

                        let to_any_map = name_map.iter().map(|(f, name)| {
                            let ident = f.ident.as_ref();
                            quote_spanned! {f.ident.span() =>
                                map.insert(#name.to_owned(), ::std::boxed::Box::new(self.#ident) as ::std::boxed::Box<dyn ::std::any::Any>);
                            }
                        });

                        let schema = name_map.iter().map(|(f, name)| {
                            let ty = &f.ty;

                            quote_spanned! {f.ident.span() =>
                                map.insert(#name.to_owned(), ::vision_traits::Type { name: ::std::any::type_name::<#ty>().to_owned() });
                            }
                        });

                        quote! {
                            impl #impl_generics Output for #ident #ty_generics #where_clause {
                                fn to_any_map(self) -> ::std::collections::HashMap<::std::string::String, ::std::boxed::Box<dyn ::std::any::Any>> {
                                    let mut map = ::std::collections::HashMap::new();
                                    #(#to_any_map)*
                                    map
                                }

                                // fn from_any_map(any_map: &::std::collections::HashMap<::std::string::String, ::std::rc::Rc<dyn ::std::any::Any>>) -> Option<Self> {
                                //     Self { #(#from_any_map),* }.into()
                                // }

                                fn schema() -> ::std::collections::HashMap<::std::string::String, ::vision_traits::Type> {
                                    let mut map = ::std::collections::HashMap::new();
                                    #(#schema)*
                                    map
                                }
                            }
                        }
                    } else {
                        quote! {
                            impl #impl_generics Output for #ident #ty_generics #where_clause {
                                fn to_any_map(self) -> ::std::collections::HashMap<::std::string::String, ::std::boxed::Box<dyn ::std::any::Any>> {
                                    ::std::collections::HashMap::new()
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
                        impl #impl_generics Output for #ident #ty_generics #where_clause {
                            // fn from_any_map(any_map: &::std::collections::HashMap<::std::string::String, ::std::rc::Rc<dyn ::std::any::Any>>) -> Option<Self> {
                            //     Self.into()
                            // }

                            fn to_any_map(self) -> ::std::collections::HashMap<::std::string::String, ::std::boxed::Box<dyn ::std::any::Any>> {
                                ::std::collections::HashMap::new()
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
