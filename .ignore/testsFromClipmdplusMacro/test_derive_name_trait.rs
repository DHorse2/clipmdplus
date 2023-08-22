#[cfg(test)]
#[allow(dead_code, unused_variables, unused_imports)] 
mod as_function {
    use super::*;
    use super::Name;
    use super::VariantName;

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
#[allow(unused_variables, dead_code, unused_imports)] 
mod name {
    // use super::NamedVariant;
    // use crate as derive_name;
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
