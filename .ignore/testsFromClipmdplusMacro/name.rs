#[allow(dead_code, unused_variables)] 
#[macro_use]
extern crate clipmdplus_macro;
use crate::clipmdplus_macro::Name;
use crate::clipmdplus_macro::VariantName;

#[derive(Name)]
struct MyInts(i32, i32);

#[derive(Name)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(Name)]
enum MixedInts {
    SmallInt(i32),
    BigInt(i64),
    TwoSmallInts(i32, i32),
    NamedSmallInts { x: i32, y: i32 },
    UnsignedOne(u32),
    UnsignedTwo(u32),
    Unit,
}
