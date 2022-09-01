use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, Lit, Meta, NestedMeta, PathSegment, Type,
    TypePath, Variant,
};

fn get_macro_attributes(variant: &Variant) -> (bool, String) {
    let mut dont_match = false;
    let mut regex = format!(r"^(?i){}$", variant.ident.to_string());

    if let Some(i) = variant
        .attrs
        .iter()
        .position(|attr| attr.path.is_ident("matches"))
    {
        if let Meta::List(list) = variant.attrs[i].parse_meta().unwrap() {
            for arg in list.nested {
                if let NestedMeta::Meta(Meta::Path(path)) = arg {
                    let PathSegment { ident, .. } = &path.segments[0];
                    if ident.to_string() == "nothing" {
                        dont_match = true;
                    }
                } else if let NestedMeta::Meta(Meta::NameValue(namevalue)) = arg {
                    let PathSegment { ident, .. } = &namevalue.path.segments[0];
                    if ident.to_string() == "regex" {
                        if let Lit::Str(literal) = namevalue.lit {
                            regex = literal.value()
                        }
                    }
                }
            }
        }
    }
    (dont_match, regex)
}

#[proc_macro_derive(Matches, attributes(matches))]
pub fn derive_match(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = input.ident;

    let mut variant_arms = TokenStream2::new();
    let mut variant_regexs = TokenStream2::new();
    let mut variant_checks = TokenStream2::new();

    match input.data {
        Data::Enum(data_enum) => {
            for variant in &data_enum.variants {
                let (dont_match, regex) = get_macro_attributes(variant);
                if dont_match {
                    continue;
                }

                let ref variant_name = variant.ident;

                match &variant.fields {
                    Fields::Unit => {
                        let regex_name =
                            format_ident!("{}", variant_name.to_string().to_uppercase());

                        variant_regexs.extend(quote_spanned! {variant.span() =>
                            static ref #regex_name: regex::Regex = regex::Regex::new(#regex).unwrap();
                        });

                        if enum_name.to_string() == "Token" {
                            variant_arms.extend(quote_spanned! {variant.span() =>
                                s if #regex_name .is_match(s) => Some(#enum_name::#variant_name),
                            });
                        } else {
                            let token_type =
                                format_ident!("{}", enum_name.to_string().chars().nth(0).unwrap());

                            variant_arms.extend(quote_spanned! {variant.span() =>
                                s if #regex_name .is_match(s) => Some(Token::#token_type(#enum_name::#variant_name)),
                            });
                        }
                    }
                    Fields::Unnamed(token_subtype) => {
                        if enum_name.to_string() == "Token" {
                            if let Type::Path(TypePath { path, .. }) = &token_subtype.unnamed[0].ty
                            {
                                let PathSegment { ident, .. } = &path.segments[0];
                                variant_checks.extend(quote_spanned! {variant.span() =>
                                    if let Some(token) = #ident ::matches(s) { return Some(token); }
                                });
                            }
                        }
                    }
                    _ => continue,
                }
            }
        }
        _ => panic!("Matches is only for enums"),
    };

    let expanded = quote! {
        impl #enum_name {
            pub fn matches(s: &str) -> Option<Token> {
                lazy_static::lazy_static! {
                    #variant_regexs
                }
                #variant_checks
                match s {
                    #variant_arms
                    _ => None
                }
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}
