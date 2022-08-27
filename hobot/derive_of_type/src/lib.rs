use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields
};

#[proc_macro_derive(OfType)]
pub fn derive_of_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = input.ident;

    let mut variant_of_type_arms = TokenStream2::new();
    
    match input.data {
        Data::Enum(data_enum) => {
            for variant in &data_enum.variants {
                let ref variant_name = variant.ident;
                let name_literal = format!(r#"{}"#, variant_name);
                
                match &variant.fields {
                    Fields::Unit => {
                        variant_of_type_arms.extend(quote_spanned! {variant.span() =>
                            #name_literal => {
                                if let #enum_name::#variant_name = self {
                                    true
                                } else {
                                    false
                                }
                            }
                        });
                    }
                    Fields::Unnamed(_) => {
                        variant_of_type_arms.extend(quote_spanned! {variant.span() =>
                            #name_literal => {
                                if let #enum_name::#variant_name (_) = self {
                                    true
                                } else {
                                    false
                                }
                            }
                        });
                    }
                    _ => continue,
                }
            }
        }
        _ => panic!("OfType is only for enums"),
    };

    let expanded = quote! {
        impl #enum_name {
            pub fn of_type(&self, s: &str) -> bool {
                match s {
                    #variant_of_type_arms
                    _ => false
                }
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}
