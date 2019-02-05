extern crate proc_macro;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use std::collections::HashSet;

#[proc_macro_derive(Routes, attributes(route))]
pub fn plaster_router(input: TokenStream) -> TokenStream {
    match syn::parse2::<syn::Item>(input.into()) {
        Ok(item) => match item {
            syn::Item::Enum(item_enum) => parse_enum(item_enum).into(),
            _ => panic!("plaster_router must be used on an enum"),
        },
        Err(e) => {
            panic!("parse error: {}", e);
        }
    }
}

fn parse_enum(item: syn::ItemEnum) -> proc_macro2::TokenStream {
    let ident = item.ident;
    let routes = item.variants.into_iter().map(|variant| {
        if let Some(path) = parse_route_attr(&variant.attrs) {
            let mut route = path.as_str();
            if route.len() != 0 && route.as_bytes()[0] == b'/' {
                route = &route[1..];
            }

            let route_literal = syn::LitStr::new(route, proc_macro2::Span::call_site());
            let variant_ident = variant.ident;
            let mut params = HashSet::new();

            for segment in route.split('/') {
                if segment.len() > 0 && segment.as_bytes()[0] == b':' {
                    params.insert(segment[1..].to_string());
                } else if segment.len() > 0 && segment.as_bytes()[0] == b'*' {
                    params.insert(segment[1..].to_string());
                }
            }

            if params.len() > 0 {
                if let syn::Fields::Named(fields) = variant.fields {
                    let field_names: HashSet<String> = fields
                        .named
                        .iter()
                        .map(|field| field.ident.as_ref().unwrap().to_string())
                        .collect();

                    if params.len() != field_names.len()
                        || params.difference(&field_names).count() > 0
                    {
                        panic!("all params must have a field in the variant");
                    }

                    let field_idents: Vec<_> = fields
                        .named
                        .into_iter()
                        .map(|field| field.ident.unwrap())
                        .collect();
                    let params_literal: Vec<syn::LitStr> = params
                        .iter()
                        .map(|param| syn::LitStr::new(param, proc_macro2::Span::call_site()))
                        .collect();

                    quote! {
                        router.add_route(#route_literal, |params| {
                            #ident::#variant_ident {
                                #(
                                    #field_idents: params.find(#params_literal).unwrap().to_string()
                                ),*
                            }
                        });
                    }
                } else {
                    panic!("all variants with params must have named fields");
                }
            } else {
                quote! {
                    router.add_route(#route_literal, |_| #ident::#variant_ident);
                }
            }
        } else {
            panic!("all variants of the enum must have a route attribute");
        }
    });

    quote! {
        impl plaster_router::Routes<#ident> for #ident {
            fn router(callback: plaster::callback::Callback<()>) -> plaster_router::Router<#ident> {
                let mut router = plaster_router::Router::new(callback);
                #(#routes)*
                router
            }
        }
    }
}

fn parse_route_attr(attrs: &[syn::Attribute]) -> Option<String> {
    attrs.iter().find_map(|attr| {
        let meta = attr
            .parse_meta()
            .expect("could not parse meta for attribute");
        match meta {
            syn::Meta::List(list) => {
                if list.ident == "route" {
                    if let Some(route) = list.nested.first() {
                        if let syn::NestedMeta::Literal(syn::Lit::Str(route)) = route.value() {
                            Some(route.value())
                        } else {
                            panic!("route spec in route attribute must be a string in quotes");
                        }
                    } else {
                        panic!("must specify a route spec in route attribute");
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    })
}
