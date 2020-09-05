use super::error::Error;
use std::collections::{BTreeMap, HashMap};
use std::marker::PhantomData;

pub trait Validation<V> {
    fn validate(&self, value: &V) -> Result<(), Error>;
}

impl<V> Validation<V> for Box<dyn Validation<V>> {
    fn validate(&self, value: &V) -> Result<(), Error> {
        self.as_ref().validate(value)
    }
}

pub trait ValidationExt<V>: Validation<V> + Sized {
    fn boxed(self) -> Box<dyn Validation<V>>
    where
        Self: 'static,
    {
        Box::new(self)
    }

    fn inner<C>(self, c: C) -> ContainerValidator<Self, C, V>
    where
        V: Container,
        C: Validation<V::Value>,
    {
        ContainerValidator::new(self, c)
    }

    fn and<C>(self, c: C) -> And<Self, C, V>
    where
        C: Validation<V>,
    {
        And {
            v1: self,
            v2: c,
            v: std::marker::PhantomData,
        }
    }
}

impl<V, T> ValidationExt<V> for T where T: Validation<V> {}

pub struct ValidationFn<F>(F);

impl<F, T> Validation<T> for ValidationFn<F>
where
    F: 'static + Fn(&T) -> Result<(), Error>,
{
    fn validate(&self, value: &T) -> Result<(), Error> {
        (self.0)(value)
    }
}

pub fn validation<F, S>(cb: F) -> impl Validation<S>
where
    F: 'static + Fn(&S) -> Result<(), Error>,
{
    ValidationFn(cb)
}

#[derive(Default)]
pub struct ValidationList<V>(Vec<Box<dyn Validation<V>>>);

impl<V> ValidationList<V> {
    pub fn push<VAL: Validation<V> + 'static>(mut self, v: VAL) -> ValidationList<V> {
        self.0.push(Box::new(v));
        self
    }
}

impl<V: 'static> Validation<V> for ValidationList<V> {
    fn validate(&self, value: &V) -> Result<(), Error> {
        let mut errors = Vec::new();
        for vali in &self.0 {
            if let Err(err) = vali.validate(value) {
                errors.push(err);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Multi(errors))
        }
    }
}

pub struct And<V1, V2, V> {
    v1: V1,
    v2: V2,
    v: std::marker::PhantomData<V>,
}

impl<V1, V2, V: 'static> Validation<V> for And<V1, V2, V>
where
    V1: Validation<V>,
    V2: Validation<V>,
{
    fn validate(&self, value: &V) -> Result<(), Error> {
        match self.v1.validate(value) {
            Ok(_) => {}
            Err(err) => return Err(err),
        }

        self.v2.validate(value)
    }
}

pub trait Container {
    type Value;
    fn validate_inner<V: Validation<Self::Value>>(&self, validator: &V) -> Result<(), Error>;
}

impl<K, V> Container for HashMap<K, V> {
    type Value = V;
    fn validate_inner<VAL: Validation<Self::Value>>(&self, validator: &VAL) -> Result<(), Error> {
        let mut errors = Vec::new();
        for (_, v) in self.iter() {
            match validator.validate(v) {
                Ok(_) => continue,
                Err(err) => {
                    errors.push(err);
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Multi(errors))
        }
    }
}

impl<K, V> Container for BTreeMap<K, V> {
    type Value = V;
    fn validate_inner<VAL: Validation<Self::Value>>(&self, validator: &VAL) -> Result<(), Error> {
        let mut errors = Vec::new();
        for (_, v) in self.iter() {
            match validator.validate(v) {
                Ok(_) => continue,
                Err(err) => {
                    errors.push(err);
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Multi(errors))
        }
    }
}

impl<V> Container for Vec<V> {
    type Value = V;
    fn validate_inner<VAL: Validation<Self::Value>>(&self, validator: &VAL) -> Result<(), Error> {
        let mut errors = Vec::new();
        for v in self.iter() {
            match validator.validate(v) {
                Ok(_) => continue,
                Err(err) => {
                    errors.push(err);
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Multi(errors))
        }
    }
}

// impl<V, T> Container for T
// where
//     T: Validation<V>,
// {
//     type Value = Option<V>;
//     fn validate_inner<VAL: Validation<Self::Value>>(&self, validator: &VAL) -> Result<(), Error> {
//         let mut errors = Vec::new();
//         for v in self.iter() {
//             match validator.validate(v) {
//                 Ok(_) => continue,
//                 Err(err) => {
//                     errors.push(err);
//                 }
//             }
//         }
//         if errors.is_empty() {
//             Ok(())
//         } else {
//             Err(Error::Multi(errors))
//         }
//     }
// }

// impl<V> Container for Option<V> {
//     type Value = V;
//     fn validate_inner<VAL: Validation<Self::Value>>(&self, validator: &VAL) -> Result<(), Error> {
//         if let Some(o) = self {
//             validator.validate(o)
//         } else {
//             Ok(())
//         }
//     }
// }

pub struct ContainerValidator<V1, V2, C> {
    v1: V1,
    v2: V2,
    _c: PhantomData<C>,
}

impl<V1, V2, C> ContainerValidator<V1, V2, C> {
    pub fn new(v1: V1, v2: V2) -> ContainerValidator<V1, V2, C> {
        ContainerValidator {
            v1,
            v2,
            _c: PhantomData,
        }
    }
}

impl<V1, V2, C> Validation<C> for ContainerValidator<V1, V2, C>
where
    V1: Validation<C>,
    V2: Validation<C::Value>,
    C: Container,
{
    fn validate(&self, value: &C) -> Result<(), Error> {
        self.v1.validate(value)?;
        value.validate_inner(&self.v2)?;
        Ok(())
    }
}

pub struct Valid<V, S> {
    validator: V,
    _s: PhantomData<S>,
    required: bool,
}

impl<V, S> Valid<V, S> {
    pub fn new(validator: V) -> Valid<V, S>
    where
        V: Validation<S>,
    {
        Valid {
            validator,
            _s: PhantomData,
            required: false,
        }
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
}

impl<V, S> Valid<V, S>
where
    V: Validation<S>,
{
    pub fn validate<'a>(&'a self, value: impl Into<Option<&'a S>>) -> Result<(), Error> {
        let value = value.into();
        match value {
            Some(s) => self.validator.validate(s),
            None => {
                if self.required {
                    Err(Error::Required)
                } else {
                    Ok(())
                }
            }
        }
    }

    pub fn and<O: Validation<S>>(self, other: O) -> Valid<And<V, O, S>, S> {
        Valid {
            required: self.required,
            validator: self.validator.and(other),
            _s: PhantomData,
        }
    }
}

impl<V, S> Validation<Option<S>> for Valid<V, S>
where
    V: Validation<S>,
{
    fn validate(&self, value: &Option<S>) -> Result<(), Error> {
        self.validate(value)
    }
}
