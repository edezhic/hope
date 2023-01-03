use std::vec::IntoIter;

use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};

use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, Lit, Meta, NestedMeta, PathSegment, Variant,
};

fn get_macro_attributes(variant: &Variant) -> (bool, IntoIter<Ident>, bool) {
    let mut expects_input = true;
    let mut expected_args = vec![];
    let mut returns_value = false;
    if let Some(i) = variant
        .attrs
        .iter()
        .position(|attr| attr.path.is_ident("syntax"))
    {
        if let Meta::List(list) = variant.attrs[i].parse_meta().unwrap() {
            for arg in list.nested {
                if let NestedMeta::Meta(Meta::Path(path)) = arg {
                    let PathSegment { ident, .. } = &path.segments[0];
                    if ident.to_string() == "returns" {
                        returns_value = true;
                    } else if ident.to_string() == "no_input" {
                        expects_input = false;
                    }
                } else if let NestedMeta::Meta(Meta::NameValue(namevalue)) = arg {
                    let PathSegment { ident, .. } = &namevalue.path.segments[0];
                    if ident.to_string() == "args" {
                        if let Lit::Str(literal) = namevalue.lit {
                            expected_args = literal
                                .value()
                                .replace(" ", "")
                                .split(",")
                                .map(str::to_string)
                                .map(|s| format_ident!("{}", s))
                                .collect();
                        }
                    }
                }
            }
        }
    }
    (expects_input, expected_args.into_iter(), returns_value)
}

#[proc_macro_derive(CommandSyntax, attributes(syntax))]
pub fn derive_command_syntax(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = input.ident;

    let mut command_syntax_arm = TokenStream2::new();

    match input.data {
        Data::Enum(data_enum) => {
            for variant in &data_enum.variants {
                let ref variant_name = variant.ident;

                match &variant.fields {
                    Fields::Unit => {
                        let (expects_input, expected_args, returns) = get_macro_attributes(variant);
                        command_syntax_arm.extend(quote_spanned! {variant.span() =>
                            Command::#variant_name => Syntax { expects_input: #expects_input, expected_args: vec![#(#expected_args),*], returns: #returns },
                        });
                    }
                    _ => continue,
                }
            }
        }
        _ => panic!("Syntax is only for Commands"),
    };

    let expanded = quote! {
        impl #enum_name {
            pub fn syntax(&self) -> Syntax {
                match self {
                    #command_syntax_arm
                }
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}
