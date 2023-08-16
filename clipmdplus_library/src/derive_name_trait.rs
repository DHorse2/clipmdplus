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

#[cfg(test)]
mod as_function {
    use super::*;
    // use super::NamedStruct;
    // use crate as derive_name;
    use crate::{self as clipmdplus_library};
    extern crate clipmdplus_macro;
    use clipmdplus_macro::Name;

    #[derive(Name)]
    struct Struct;

    #[derive(Name)]
    enum Enum {
        One,
        Two,
        Three
    }

    #[test]
    fn test() {
        assert_eq!(Struct::name(), "Struct");
        assert_eq!(Enum::name(), "Enum");
    }
}

#[cfg(test)]
mod as_method {
    // use super::NamedStruct;
    // use crate as derive_name;
    use crate as clipmdplus_library;
    extern crate clipmdplus_macro;
    use clipmdplus_macro::Name;
    use clipmdplus_macro::VariantName;
    use clipmdplus_library::NamedStruct;
    use clipmdplus_library::NamedVariant;

    
    #[derive(Name)]
    struct Struct;

    #[derive(VariantName)]
    enum Enum {
        A,
    }

    #[test]
    fn test() {
        assert_eq!(Struct::name(), "Struct");
        assert_eq!(Enum::name(), "Struct");
        assert_eq!(Enum::A.name(), "A");
    }
}

#[cfg(test)]
mod name {
    // use super::NamedVariant;
    // use crate as derive_name;
    use clipmdplus_macro::Name;
    use clipmdplus_macro::VariantName;
    use clipmdplus_library::NamedStruct;
    use clipmdplus_library::NamedVariant;

    #[derive(VariantName)]
    enum Enum {
        Alice,
        Bob(i32),
        Claire { i: i32 },
    }

    #[test]
    fn test() {
        assert_eq!(Enum::Alice.name(), "Alice");
        assert_eq!(Enum::Bob(1).name(), "Bob");
        assert_eq!(Enum::Claire { i: 1 }.name(), "Claire");
    }
}
