use std::fmt::{Display, Formatter, Result};
use Solution::*;

pub enum Solution {
    I32(i32),
    I64(i64),
    I128(i128),
    U32(u32),
    U64(u64),
    U128(u128),
    Str(String),
    USize(usize),
    ISize(isize)
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            I32(x) => x.fmt(f),
            I64(x) => x.fmt(f),
            I128(x) => x.fmt(f),
            U32(x) => x.fmt(f),
            U64(x) => x.fmt(f),
            U128(x) => x.fmt(f),
            Str(x) => x.fmt(f),
            USize(x) => x.fmt(f),
            ISize(x) => x.fmt(f)
        }
    }
}