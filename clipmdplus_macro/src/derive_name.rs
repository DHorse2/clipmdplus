/// derive_name.rs
/// ClipMdPlus Macro Library - Struct & Enum Name, Variants
///
/// Provides simple syntax to the item and variant names.
/// 
/// proc_macro_derive:
///     use super::super::Names;
///     use super::super::VariantNames;
///     use clipmdplus_macro::VariantNames;
/// traits:
///     Name
///     Named for T
///     VariantName
///     

// derive_name crates
// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{self, parse_quote, Arm, Data};

/// Names
#[allow(unused_macros)]
#[proc_macro_derive(Names)]
// #[macro_export]
pub fn names(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse_macro_input!(input);
    let ident = &ast.ident;
    let gen = quote! {
        impl clipmdplus_macro::Name for #ident {
            fn name() -> &'static str {
                stringify!(#ident)
            }
        }
    };
    gen.into()
}
/// VariantNames
#[allow(unused_macros)]
#[proc_macro_derive(VariantNames)]
// #[macro_export]
pub fn variant(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse_macro_input!(input);

    if let Data::Enum(r#enum) = &ast.data {
        let ident = &ast.ident;
        let mut match_arms = Vec::<Arm>::with_capacity(r#enum.variants.len());

        for variant in r#enum.variants.iter() {
            let variant_ident = &variant.ident;
            let match_pattern = match &variant.fields {
                syn::Fields::Named(_) => {
                    quote!( Self::#variant_ident {..} )
                }
                syn::Fields::Unnamed(_) => {
                    quote!( Self::#variant_ident (..) )
                }
                syn::Fields::Unit => quote!( Self::#variant_ident ),
            };

            match_arms.push(parse_quote! {
                #match_pattern => stringify!(#variant_ident)
            });
        }
        let gen = quote! {
            // use clipmdplus_macro::Name;
            impl clipmdplus_macro::VariantName for #ident {
                fn variant_name(&self) -> &'static str {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
        };
        gen.into()
    } else {
        quote!(
            compile_error!("Can only implement 'VariantName' on a enum");
        )
        .into()
    }
}