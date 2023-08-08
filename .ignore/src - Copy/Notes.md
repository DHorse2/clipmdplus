# Notes

## Derive_more notes

use derive_more::{Add {...}, AsMut, AsRef, Display, From, FromStr, Into, IntoIterator, TryInto};

use Display-like, contains Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp, Pointer

### Operators

These are traits that can be used for operator overloading.

    Index

    Deref

    Not-like, contains Not and Neg

    Add-like, contains Add, Sub, BitAnd, BitOr, BitXor

    Mul-like, contains Mul, Div, Rem, Shr and Shl

    Sum-like, contains Sum and Product

    IndexMut

    DerefMut

    AddAssign-like, contains AddAssign, SubAssign, BitAndAssign, BitOrAssign and BitXorAssign

    MulAssign-like, contains MulAssign, DivAssign, RemAssign, ShrAssign and ShlAssign

## Crates Used

[macro_use].

    extern crate anyhow;            (no - static types preferred)

    extern crate blob;              (doesn't implement ToSql trait)

    extern crate derive_name;       (not working so far - trying it)

    extern crate derive_more;

    extern crate egui;

    extern crate eframe;            (for egui)

    extern crate serde;

    extern crate serde_derive;

    extern crate serde_enum_str;    (buggy)

    extern crate serde_json;

    extern crate serde-error        (no - uses anyhow::result)

    extern crate thiserror;         (unit errors only)
