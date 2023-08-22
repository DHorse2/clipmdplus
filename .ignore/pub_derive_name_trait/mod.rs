// clipMdPlus_Macro Derive_Name_Trait.rs
/// # Derive Name
///
/// Derive macro to get the name of a struct, enum or enum variant.
///
/// ## Name
///
/// ```
/// use derive_name::Name;
///
/// #[derive(Name)]
/// struct Alice;
///
/// #[derive(Name)]
/// enum Bob {}
///
/// assert_eq!(Alice::name(), "Alice");
/// assert_eq!(Bob::name(), "Bob");
/// ```
///
/// ## Named
///
/// ```
/// use derive_name::Named;
///
/// #[derive(derive_name::Name)]
/// struct Alice;
///
/// #[derive(derive_name::Name)]
/// enum Bob {
///     Variant
/// }
///
/// let her = Alice {};
/// let his = Bob::Variant;
///
/// assert_eq!(her.name(), "Alice");
/// assert_eq!(his.name(), "Bob");
/// ```
///
/// ## VariantName
/// ```
/// use derive_name::VariantName;
///
/// #[derive(VariantName)]
/// enum Alice {
///     Bob
/// }
///
/// assert_eq!(Alice::Bob.name(), "Bob");
/// ```
// !------------------------------------------------------------
// pub use derive_name::{Name, VariantName};
extern crate clipmdplus_macro;
// use clipmdplus_macro::{Name, VariantName};

pub trait NamedStruct {
    fn name() -> &'static str;
}

pub trait Named {
    fn named(&self) -> &'static str;
}

impl<T: NamedStruct> Named for T {
    fn named(&self) -> &'static str {
        T::name()
    }
}

pub trait NamedVariant {
    fn variant_name(&self) -> &'static str;
}
// ! ------------------------------------------------------------
include!(".\\test_derive_name_trait.rs");
// ! ------------------------------------------------------------
