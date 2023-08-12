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
/// #[derive(Names)]
/// struct Alice;
///
/// #[derive(Names)]
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
/// assert_eq!(Alice::Bob.variant_name(), "Bob");
/// ```
// !------------------------------------------------------------
// pub use derive_name::{Name, VariantName};
extern crate clipmdplus_macro;
pub use clipmdplus_macro::{Names, VariantNames};

pub trait Name {
    fn name() -> &'static str;
}

pub trait Named {
    fn name(&self) -> &'static str;
}

impl<T: Name> Named for T {
    fn name(&self) -> &'static str {
        T::name()
    }
}

pub trait VariantName {
    fn variant_name(&self) -> &'static str;
}

#[cfg(test)]
mod as_function {
    // use super::Name;
    // use crate as derive_name;

    #[derive(Names)]
    struct Struct;

    #[derive(Names)]
    enum Enum {}

    #[test]
    fn test() {
        assert_eq!(Struct::name(), "Struct");
        assert_eq!(Enum::name(), "Enum");
    }
}

#[cfg(test)]
mod as_method {
    // use super::Name;
    // use crate as derive_name;

    #[derive(clipmdplus_macro::Name)]
    struct Struct;

    #[derive(clipmdplus_macro::Name)]
    enum Enum {
        A,
    }

    #[test]
    fn test() {
        assert_eq!(Struct.name(), "Struct");
        assert_eq!(Enum::A.name(), "Enum");
    }
}

#[cfg(test)]
mod variant_name {
    // use super::VariantName;
    // use crate as derive_name;

    #[derive(clipmdplus_macro::VariantNames)]
    enum Enum {
        Alice,
        Bob(i32),
        Claire { i: i32 },
    }

    #[test]
    fn test() {
        assert_eq!(Enum::Alice.variant_name(), "Alice");
        assert_eq!(Enum::Bob(1).variant_name(), "Bob");
        assert_eq!(Enum::Claire { i: 1 }.variant_name(), "Claire");
    }
}
