use super::{Error, Validation};
use std::collections::{BTreeMap, HashMap};
use std::marker::PhantomData;

pub struct Required<V>(PhantomData<V>);

pub fn required<V>() -> Required<Option<V>> {
    Required(PhantomData)
}

pub struct All;

impl<V> Validation<V> for All {
    fn validate(&self, _value: &V) -> Result<(), Error> {
        Ok(())
    }
}

macro_rules! required_impl {
    ($ident: ty) => {
        impl Validation<$ident> for Required<$ident> {
            fn validate(&self, value: &$ident) -> Result<(), Error> {
                if value.is_empty() {
                    Err(Error::Required)
                } else {
                    Ok(())
                }
            }
        }
    };
    ($ident: ty, $life: lifetime) => {
        impl<$life> Validation<&$life $ident> for Required<&$life $ident> {
            fn validate(&self, value: &&$life $ident) -> Result<(), Error> {
                if value.is_empty() {
                    Err(Error::Required)
                } else {
                    Ok(())
                }
            }
        }
    };
    ($ident: ident, $($ty: ident)*) => {
        impl<$($ty),*> Validation< $ident<$($ty),*> > for Required<$ident<$($ty),*>> {
            fn validate(&self, value: &$ident<$($ty),*>) -> Result<(), Error> {
                if value.is_empty() {
                    Err(Error::Required)
                } else {
                    Ok(())
                }
            }
        }
    };
}

required_impl!(String);
required_impl!(str, 'a);
required_impl!([u8], 'a);
required_impl!(Vec, V);
required_impl!(HashMap, K V);
required_impl!(BTreeMap, K V);

impl<V> Validation<Option<V>> for Required<Option<V>> {
    fn validate(&self, value: &Option<V>) -> Result<(), Error> {
        if value.is_none() {
            Err(Error::Required)
        } else {
            Ok(())
        }
    }
}

// macro_rules! option_impl {
//     ($ident: path, $ty: ty) => {
//         impl Validation<Option<$ty>> for $ident {
//             fn validate(&self, value: &Option<$ty>) -> Result<(), Error> {
//                 if let Some(s) = value {
//                     <Self as Validation<$ty>>::validate(self, s)
//                 } else {
//                     Ok(())
//                 }
//             }
//         }
//     };
// }

pub struct MinLen(pub usize);

pub struct MaxLen(pub usize);

macro_rules! len_impl {
    ($valid: ident, $ident: ty, $op: tt) => {
        impl Validation<$ident> for $valid {
            fn validate(&self, value: &$ident) -> Result<(), Error> {
                if !(value.len() $op self.0) {
                    Err(Error::$valid(self.0))
                } else {
                    Ok(())
                }
            }
        }
    };
    ($valid: ident, $ident: ty, $op: tt, $life: lifetime) => {
        impl<$life> Validation<&$life $ident> for $valid {
            fn validate(&self, value: &&$life $ident) -> Result<(), Error> {
                if !((*value).len() $op self.0) {
                    Err(Error::$valid(self.0))
                } else {
                    Ok(())
                }
            }
        }

    };
    ($valid: ident, $ident: ident, $op: tt, $($ty: ident)*) => {
        impl<$($ty),*> Validation< $ident<$($ty),*> > for $valid {
            fn validate(&self, value: &$ident<$($ty),*>) -> Result<(), Error> {
                if !(value.len() $op self.0) {
                    Err(Error::$valid(self.0))
                } else {
                    Ok(())
                }
            }
        }
    };
}

macro_rules! minlen_impl {
    ($ident: ty) => {
        len_impl!(MinLen, $ident, >=);
    };
    ($ident: ty, $life: lifetime) => {
        len_impl!(MinLen, $ident, >=, $life);
    };
    ($ident: ident, $($ty: ident)*) => {
        len_impl!(MinLen, $ident, >=, $($ty)*);
    };
}

minlen_impl!(String);
minlen_impl!(str,'a);
minlen_impl!([u8], 'a);
minlen_impl!(Vec, V);
minlen_impl!(HashMap, K V);
minlen_impl!(BTreeMap, K V);

macro_rules! maxlen_impl {
    ($ident: ty) => {
        len_impl!(MaxLen, $ident, <=);
    };
    ($ident: ty, $life: lifetime) => {
        len_impl!(MaxLen, $ident, <=, $life);
    };
    ($ident: ident, $($ty: ident)*) => {
        len_impl!(MaxLen, $ident, <=, $($ty)*);
    };
}

maxlen_impl!(String);
maxlen_impl!(str,'a);
maxlen_impl!([u8], 'a);
maxlen_impl!(Vec, V);
maxlen_impl!(HashMap, K V);
maxlen_impl!(BTreeMap, K V);

pub struct Min<I>(pub I);
pub struct Max<I>(pub I);

macro_rules! minmax_impl {
    ($ty: ty) => {
        impl Validation<$ty> for Min<$ty> {
            fn validate(&self, value: &$ty) -> Result<(), Error> {
                if !(value >= &self.0) {
                    Err(Error::MinLen(self.0 as usize))
                } else {
                    Ok(())
                }
            }
        }

        impl Validation<$ty> for Max<$ty> {
            fn validate(&self, value: &$ty) -> Result<(), Error> {
                if !(value <= &self.0) {
                    Err(Error::MaxLen(self.0 as usize))
                } else {
                    Ok(())
                }
            }
        }
    };
}

minmax_impl!(i8);
minmax_impl!(u8);
minmax_impl!(i16);
minmax_impl!(u16);
minmax_impl!(i32);
minmax_impl!(u32);
minmax_impl!(i64);
minmax_impl!(u64);
minmax_impl!(f32);
minmax_impl!(f64);
minmax_impl!(usize);
minmax_impl!(isize);

#[cfg(regexp)]
pub struct Regexp<'a>(pub &'a regexp::regex::Regex);

#[cfg(regexp)]
impl<'a> Validation<String> for Regexp<'a> {
    fn validate(&self, value: &String) -> Result<(), Error> {
        if self.0.is_match(value) {
            Ok(())
        } else {
            Err(Error::Required)
        }
    }
}

#[cfg(regexp)]
impl<'a, 'b> Validation<&'b str> for Regexp<'a> {
    fn validate(&self, value: &&'a str) -> Result<(), Error> {
        if self.0.is_match(value) {
            Ok(())
        } else {
            Err(Error::Required)
        }
    }
}

pub struct Parse<P>(std::marker::PhantomData<P>);

impl<P> Parse<P> {
    pub fn new() -> Parse<P> {
        Parse(std::marker::PhantomData)
    }
}

impl<P: std::str::FromStr> Validation<String> for Parse<P>
where
    P::Err: std::error::Error,
{
    fn validate(&self, value: &String) -> Result<(), Error> {
        match P::from_str(value) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Custom(e.to_string())),
        }
    }
}
