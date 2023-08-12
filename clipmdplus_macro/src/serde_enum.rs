/// stdmd types serde_enum.rs (missing crate)
/// Derives:
/// Serialize_enum
/// Deserialize_enum
/// ToString
/// NamingStyle
///     SnakeCase
///     CamelCase
///     ScreamingSnakeCase
///
/// The clipboard processes new clips and saves to history.
/// You can add a clip to a category or folder. (All standard)
/// Pasting and saving features differentiates this crate.
/// Ideally, all of the crates reformatting tricks should be exported.
/// The serde_enum crate already contained good case conversion functionality.
/// This is a learning process for me (DaveGH).
/// I might be doing it wrong. Hmmmm.
///
/// Functions:
/// enum NamingStyle {
/// static ref NAME_MAP: HashMap<NamingStyle, fn(&str) -> String> = {
/// pub fn to_string_enum(item: TokenStream) -> TokenStream {
/// pub fn deserialize_enum(item: TokenStream) -> TokenStream {
/// pub fn serialize_enum(item: TokenStream) -> TokenStream {
/// pub fn get_naming_style(item: TokenStream) -> TokenStream {
/// pub fn to_snake_case(v: &str) -> String {
/// pub fn to_camel_case(v: &str) -> String {
/// pub fn to_screaming_snake_case(v: &str) -> String {
///
///
/// fn format_variant(v: &Variant, parent_style: NamingStyle) -> String {
/// NamingStyle
/// SnakeCase
/// fn to_snake_case_impl(v: &str) -> String {
/// CamelCase
/// fn to_camel_case_impl(v: &str) -> String {
/// ScreamingSnakeCase
/// fn to_screaming_snake_case_impl(v: &str) -> String {
/// None
/// fn get_variant_alias(v: &Variant) -> Option<String> {
/// fn get_variant_alias(v: &Variant) -> Option<String> {
/// fn get_enum_from_input(target: &DeriveInput) -> DataEnum {
///
/// fn create_ser_arms(target: &DataEnum, n: NamingStyle) -> impl Iterator<Item = TokenStream2> {
/// fn create_to_str_arms(target: &DataEnum, n: NamingStyle) -> impl Iterator<Item = TokenStream2> {
/// fn create_de_arms(target: &DataEnum, n: NamingStyle) -> impl Iterator<Item = TokenStream2> {
///

// ! Crates -------------------------------------------------------
// NamingStyle and NAME_MAP
// serde_enum
// use std::collections::HashMap;
// use syn::*;
// use syn::{Attribute, DataEnum, DeriveInput, Expr, ExprLit, ExprParen, Fields, Lit, parse2, parse_macro_input, Variant}; // path, tokens, Token, 
#[derive(Debug, derive_more::Display, Clone, Copy, PartialEq, Eq, Hash)]
enum NamingStyle {
    SnakeCase,
    CamelCase,
    ScreamingSnakeCase,
    None,
}
// ! NAME_MAP -------------------------------------------------------
lazy_static! {
    /// doc todo create_ser_arms
static ref NAME_MAP: HashMap<NamingStyle, fn(&str) -> String> = {
        let mut m = HashMap::new();

        // I have no actual idea why this is working like that, and why I need this cast
        m.insert(NamingStyle::SnakeCase, to_snake_case_impl as fn(&str) -> String);
        m.insert(NamingStyle::CamelCase, to_camel_case_impl);
        m.insert(NamingStyle::ScreamingSnakeCase, to_screaming_snake_case_impl);
        m
    };
}
// ! Serialization -------------------------------------------------------
// You need to mark your macro with #[macro_export] and then mark the module with #[macro_use]:
// #[proc_macro]. 
// This doesn't apply to proc_macro_derive. #[allow(unused_macros)] needed for libraries.
// ! (serde) Serialization -------------------------------------------------------
/// doc todo create_ser_arms
#[allow(unused_macros)]
// #[macro_export]
#[proc_macro_derive(Deserialize_enum, attributes(serde))]
pub fn deserialize_enum(item: TokenStream) -> TokenStream {
    let target = parse_macro_input!(item as DeriveInput);
    let data = get_enum_from_input(&target);

    let style = get_naming_style_impl(target.attrs.iter());

    let target_ident = &target.ident;
    let de_arms = create_de_arms(&data, style);
    let out = quote! {
        impl<'de> serde::Deserialize<'de> for #target_ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>
            {
                Ok(
                    match <&str>::deserialize(deserializer)? {
                        #(#de_arms),*,
                        _ => { unimplemented!() }
                    }
                )
            }
        }
    };
    out.into()
}
/// doc todo create_ser_arms
#[allow(unused_macros)]
// #[macro_export]
#[proc_macro_derive(Serialize_enum, attributes(serde))]
pub fn serialize_enum(item: TokenStream) -> TokenStream {
    let target = parse_macro_input!(item as DeriveInput);
    let data = get_enum_from_input(&target);

    let style = get_naming_style_impl(target.attrs.iter());

    let target_ident = &target.ident;
    let ser_arms = create_ser_arms(&data, style);
    let out = quote! {
        impl serde::Serialize for #target_ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer
            {
                match self {
                    #(#ser_arms),*
                }
            }
        }
    };
    out.into()
}
// ! Naming Conversion macros -------------------------------------------------------
// todo move Naming Conversion macros to separate crate.
/// doc todo ToString to_string_enum
#[allow(unused_macros)]
// #[macro_export]
#[proc_macro_derive(ToString)]
pub fn to_string_enum(item: TokenStream) -> TokenStream {
    let target = parse_macro_input!(item as DeriveInput);
    let data = get_enum_from_input(&target); // validates

    let ident = &target.ident;

    let style = get_naming_style_impl(target.attrs.iter());

    let to_str_arms = create_to_str_arms(&data, style);

    let out = quote! {
        impl std::convert::From<&#ident> for &'static str {
            fn from(v: &#ident) -> &'static str {
                match v {
                    #(#ident::#to_str_arms),*
                }
            }
        }

        impl std::string::ToString for #ident {
            fn to_string(&self) -> String {
                <&#ident as std::convert::Into<&'static str>>::into(self).to_string()
            }
        }
    };
    out.into()
}
/// doc todo NamingStyle get_naming_style
#[allow(unused_macros)]
#[proc_macro_derive(NamingStyle)]
pub fn get_naming_style(item: TokenStream) -> TokenStream {
    let target = parse_macro_input!(item as DeriveInput);
    let naming_style = get_naming_style_impl(target.attrs.iter());
    // out.into()
    // out.to_string();
    let s = naming_style.to_string();
    let stream: proc_macro::TokenStream = s.parse().unwrap();
    stream
    // .parse().expect("valid style")
    // let out = quote! {
    //     impl<'de> serde::Deserialize<'de> for #target_ident {
    //         fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    //         where
    //             D: serde::Deserializer<'de>
    //         {
    //             Ok(
    //                 match <&str>::deserialize(deserializer)? {
    //                     #(#de_arms),*,
    //                     _ => { unimplemented!() }
    //                 }
    //             )
    //         }
    //     }
    // };
    // out.into()
}
/// doc todo SnakeCase 
// #[proc_macro_derive(SnakeCase)]
// pub fn to_snake_case(v: &str) -> String {
#[allow(unused_macros)]
macro_rules! SnakeCase {
    ($value:ident) => {
        #[doc = "SnakeCase macro"]
        to_snake_case_impl($value)
    };
}
/// doc todo CamelCase to_camel_case_impl
// #[proc_macro_derive(CamelCase)]
// pub fn to_camel_case(v: &str) -> String {
#[allow(unused_macros)]
macro_rules! CamelCase {
    ($value:expr) => {
        #[doc = "SnakeCase macro"]
        to_camel_case_impl($value)
    };
}
/// doc todo ScreamingSnakeCase to_screaming_snake_case_impl
// #[proc_macro_derive(ScreamingSnakeCase)]
// pub fn to_screaming_snake_case(v: &str) -> String {
#[allow(unused_macros)]
macro_rules! ScreamingSnakeCase {
    ($value:expr) => {
        #[doc = "ScreamingSnakeCase macro"]
        to_screaming_snake_case_impl($value)
    };
}
// ! Naming Style functions -------------------------------------------------------
// ! Format functions -------------------------------------------------------
/// Attribute to NamingStyle conversion
/// doc todo get_naming_style_impl
fn get_naming_style_impl<'a>(target: impl Iterator<Item = &'a Attribute>) -> NamingStyle {
    // a is an Attribute
    for a in target {
        // A a.path at which a named item is exported (e.g. std::collections::HashMap).
        // Attribute.meta is Syn:Meta and also has a path
        // Attribute.path() exists. ( was a.path.get_ident() )
        // get_ident() - A word of Rust code, which may be a keyword or legal variable name.
        if let Some(i) = a.path().get_ident() {
            // So serde only
            if i == "serde" {
                // if let Ok(ExprParen { expr, .. }) = parse2::<ExprParen>(a.tokens.clone()) {
                // parse2 - syn
                // pub fn parse2<T>(tokens: proc_macro2::TokenStream) -> Result<T>
                // where
                //     T: parse::Parse,
                // I'm assuming it wants the bracket_token here - []
                // maypbe pound_token instead
                // if let Ok(ExprParen { expr, .. }) = parse2::<ExprParen>(Token![a.bracket_token] ) {
                // TokenStream::new(Vec<a>)
                if let Ok(ExprParen { expr, .. }) = parse2::<ExprParen>(a.parse_args().unwrap()) {
                    // ea Expression Assign
                    if let Expr::Assign(ea) = expr.as_ref() {
                        // ep - unknown meaning here:
                        // two functions are introduced here:
                        // .left.as_ref()
                        // .right.as_ref()
                        if let Expr::Path(ep) = ea.left.as_ref() {
                            // ?
                            if let Some(i) = ep.path.get_ident() {
                                // Attributes:
                                if i == "rename" || i == "rename_all" {
                                    // capture litteral into s as a str?
                                    if let Expr::Lit(ExprLit {
                                        lit: Lit::Str(s), ..
                                    }) = ea.right.as_ref()
                                    {
                                        return match s.value().as_str() {
                                            "snake_case" => NamingStyle::SnakeCase,
                                            "camelCase" => NamingStyle::CamelCase,
                                            "SCREAMING_SNAKE_CASE" => {
                                                NamingStyle::ScreamingSnakeCase
                                            }
                                            _ => {
                                                panic!(
                                                    "Unsupported style. \
                                                    Available: `snake_case`, `camelCase`"
                                                )
                                            }
                                        };
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    NamingStyle::None
}
/// Variant to String conversion
/// doc todo format_variant
fn format_variant(v: &Variant, parent_style: NamingStyle) -> String {
    if let Some(s) = get_variant_alias(v) {
        return s;
    }

    let own_style = get_naming_style_impl(v.attrs.iter());

    match own_style {
        NamingStyle::None => match parent_style {
            NamingStyle::None => v.ident.to_string(),
            ps => NAME_MAP.get(&ps).unwrap()(&v.ident.to_string()),
        },
        os => NAME_MAP.get(&os).unwrap()(&v.ident.to_string()),
    }
}
/// SnakeCase conversion
/// doc todo to_snake_case_impl
fn to_snake_case_impl(v: &str) -> String {
    let mut out = String::with_capacity(v.len());
    if v.is_empty() {
        out.push(v.chars().next().unwrap().to_ascii_lowercase());
    }

    for c in v.chars().skip(1) {
        if c.is_uppercase() {
            out.push('_');
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }

    out
}
/// CamelCase conversion
fn to_camel_case_impl(v: &str) -> String {
    v.to_string()
        .char_indices()
        .map(|(i, c)| if i == 0 { c.to_ascii_lowercase() } else { c })
        .collect()
}
/// SCREAMING_SNAKE_CASE conversion
/// doc todo to_screaming_snake_case_impl
fn to_screaming_snake_case_impl(v: &str) -> String {
    v.char_indices()
        .fold(String::with_capacity(v.len()), |mut s, (i, c)| {
            if c.is_uppercase() && i != 0 {
                s.push('_');
            }
            s.push(c.to_ascii_uppercase());
            s
        })
}
// ! Variants -------------------------------------------------------
/// Takes a Variant, checks for the _ident, if "serde", if the path _ident is "name" return the s Expr::Lit
/// Variant Attributes ident("serde"), ExprParen, parse_args, ExprAssign, ExprPath, ident, ExprLit
fn get_variant_alias(v: &Variant) -> Option<String> {
    for a in v.attrs.iter() {
        if let Some(i) = a.path().get_ident() {
            if i == "serde" {
                if let Ok(ExprParen { expr, .. }) = parse2::<ExprParen>(a.parse_args().unwrap()) {
                    // if let Ok(ExprParen { expr, .. }) = parse2::<ExprParen>(a.tokens.clone()) {
                    if let Expr::Assign(ea) = expr.as_ref() {
                        if let Expr::Path(ep) = ea.left.as_ref() {
                            if let Some(i) = ep.path.get_ident() {
                                if i == "name" {
                                    if let Expr::Lit(ExprLit {
                                        lit: Lit::Str(s), ..
                                    }) = ea.right.as_ref()
                                    {
                                        return Some(s.value());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}
/// Checks that the passed input's .data is an Enum and clones the ref
fn get_enum_from_input(target: &DeriveInput) -> DataEnum {
    if !target.generics.params.is_empty() {
        panic!("`Serialize_enum` target cannot have any generics parameters!");
    }

    if let Data::Enum(ref e) = target.data {
        e.clone()
    } else {
        panic!("`Serialize_enum` can only be applied to enums!");
    }
}
// ! Create arms -------------------------------------------------------
/// doc todo create_ser_arms
fn create_ser_arms(target: &DataEnum, n: NamingStyle) -> impl Iterator<Item = TokenStream2> {
    target.variants.clone().into_iter().map(move |v| {
        assert!(matches!(v.fields, Fields::Unit));
        let ident = &v.ident;
        let value = format_variant(&v, n);

        quote! {
            Self::#ident => { serializer.serialize_str(#value) }
        }
    })
}
/// doc todo create_to_str_arms
fn create_to_str_arms(target: &DataEnum, n: NamingStyle) -> impl Iterator<Item = TokenStream2> {
    target.variants.clone().into_iter().map(move |v| {
        let ident = &v.ident;
        let value = format_variant(&v, n);

        quote! {
            #ident => #value
        }
    })
}
/// doc todo create_de_arms
fn create_de_arms(target: &DataEnum, n: NamingStyle) -> impl Iterator<Item = TokenStream2> {
    target.variants.clone().into_iter().map(move |v| {
        assert!(matches!(v.fields, Fields::Unit));

        let ident = &v.ident;
        let value = format_variant(&v, n);

        quote! {
            #value => Self::#ident
        }
    })
}
