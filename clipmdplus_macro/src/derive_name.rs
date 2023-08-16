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

// derive_name crates
// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{self, parse_quote, Arm, Data};

/// Name
#[allow(unused_macros)]
// #[proc_macro_export]
#[proc_macro_derive(Name)]
pub fn names(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse_macro_input!(input);
    let ident = &ast.ident;
    let gen = quote! {
        impl clipmdplus_library::NamedStruct for #ident {
            fn name() -> &'static str {
                stringify!(#ident)
            }
        }
    };
    eprintln!("{}", gen);
    gen.into()
}
/// VariantName
#[allow(unused_macros)]
// #[macro_export]
#[proc_macro_derive(VariantName)]
pub fn variant(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse_macro_input!(input);
    let item_ident = &ast.ident;
    
    if let Data::Enum(r#enum) = &ast.data {
        let ident = &ast.ident;
        let gen_enum: TokenStream2 = quote! {
            impl clipmdplus_library::NamedStruct for #ident {
                fn name() -> &'static str {
                    stringify!(#ident)
                }
            }
        };
        
        // eprintln!("{}", gen);
        // gen_e.<TokenStream2 as Into<T>>::into();

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
        let mut gen_variant = quote! {
            impl clipmdplus_library::NamedVariant for #ident {
                fn name(&self) -> &'static str {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
        };
        // eprintln!("{}", gen);
        gen_variant.append_all(gen_enum);
        // let gen_out: TokenStream2 = format!("{:?}{:?}", gen_enum, gen_variant);
        // let gen_out: TokenStream2 = syn::parse2(format!("{:?}{:?}", gen_enum, gen_variant));
        // let gen_out: TokenStream2 = syn::parse2(gen_enum, gen_variant);
        // let gen_out = syn::parse_str(format!("{:?}{:?}", gen_enum, gen_variant));
        // eprintln!("{}", gen_out);
        // gen_out.into()
        eprintln!("{}", gen_variant);
        gen_variant.into()
    } else {
        quote!(
            compile_error!("Can only implement 'VariantName' on a enum. {} is not an enum", #item_ident);
        )
        .into()
    }
}