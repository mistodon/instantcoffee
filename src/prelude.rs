#![allow(dead_code)]

pub use std::borrow::Cow;
pub use std::iter::Iterator;

#[allow(non_camel_case_types)]
pub type sstr = &'static str;

#[allow(non_camel_case_types)]
pub type cowstr = Cow<'static, str>;

#[allow(non_camel_case_types)]
pub type CowVec<T> = Cow<'static, [T]>;
