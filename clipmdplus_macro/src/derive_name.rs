/// derive_name.rs
/// ClipMdPlus Macro Library - Struct & Enum Name, Variants
///
/// Provides simple syntax to the item and variant names.
/// 
/// proc_macro_derive:
///     use super::super::Name;
///     use super::super::VariantName;
///     use clipmdplus_macro::VariantName;
/// traits:
///     Name
///     Named for T
///     VariantName
///     

// derive_name macros
use lazy_static::lazy_static;
// proc_macro
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use quote::TokenStreamExt;

/// Name
#[allow(unused_macros)]
// #[proc_macro_export]
#[proc_macro_derive(Name)]
pub fn names(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse_macro_input!(input);
    let ident = &ast.ident;
    let quoted_ident = format!("{}", ident);
    let gen = quote! {

        impl #ident {
            fn name() -> &'static str {
                #quoted_ident
                // #ident
                // stringify!(#ident)
            }
        }

    };
    // eprintln!("{}", gen);
    gen.into()
}
/// VariantName
#[allow(unused_macros)]
// #[macro_export]
#[proc_macro_derive(VariantName)]
pub fn variant(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse_macro_input!(input);
    let item_ident = &ast.ident;

    // let _quote_mark = '"';
    
    if let Data::Enum(r#enum) = &ast.data {
        let ident = &ast.ident;
        let quoted_ident = format!("{}", ident);
        // let quoted_ident = format!("{}{}{}", quote_mark, ident, quote_mark);
        let mut gen_enum: TokenStream2 = quote! {
            impl #ident {
                fn enum_name() -> &'static str {
                    #quoted_ident
                    // \"#ident\"
                    // stringify!(#ident)
                }
            }

        };
        // eprintln!("{}", gen_enum);

        let mut match_arms = Vec::<Arm>::with_capacity(r#enum.variants.len());

        for variant in r#enum.variants.iter() {
            let variant_ident = &variant.ident;
            let match_pattern = match &variant.fields {
                syn::Fields::Named(_) => {                    quote!( Self::#variant_ident {..} )
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
        let gen_variant = quote! {
            
            impl #ident {
                fn variant_name(&self) -> &'static str {
                    match self {
                        #(#match_arms),*
                    }
                }
            }

        };

        gen_enum.append_all(gen_variant);
        // eprintln!("{}", gen_enum);
        gen_enum.into()
    } else {
        // &ast.ident
        let msg = format!("Can only implement 'VariantName' on a enum. {} is not an enum", item_ident);
        quote!(
            compile_error!(#msg);
        )
        .into()
    }
}
