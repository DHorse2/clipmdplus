// lib.rs in enum_name

extern crate self as enum_name;

#[macro_use]
mod enum_name_derive;

// pub use types::enum_name_derive::EnumName;

pub trait EnumName {
    fn enum_name(&self) -> &'static str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(EnumName)]
    #[allow(dead_code)]
    enum MyEnum<'a, T> {
        VariantA,
        VariantB(T, i32),
        AnotherOne { x: &'a str },
    }

    #[test]
    fn test_enum_name() {
        assert_eq!("VariantA", MyEnum::VariantA::<u32>.enum_name());
        assert_eq!("VariantB", MyEnum::VariantB(1, 2).enum_name());
        assert_eq!(
            "AnotherOne",
            MyEnum::AnotherOne::<u8> { x: "test" }.enum_name()
        );
    }
}