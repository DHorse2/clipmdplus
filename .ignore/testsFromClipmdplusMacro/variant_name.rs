#![allow(dead_code)]
#[macro_use]
extern crate clipmdplus_macro;
use clipmdplus_macro::VariantName;

#[derive(VariantName)]
struct MyInts(i32, i32);

#[derive(VariantName)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(VariantName)]
enum MixedInts {
    SmallInt(i32),
    BigInt(i64),
    TwoSmallInts(i32, i32),
    NamedSmallInts { x: i32, y: i32 },
    UnsignedOne(u32),
    UnsignedTwo(u32),
    Unit,
}
