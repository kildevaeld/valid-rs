use super::{Error, Validation};
use serde_json::{value::Map, Value};
use std::marker::PhantomData;

#[derive(Default)]
pub struct Object {
    validators: Vec<(String, Box<dyn Validation<Value>>)>,
}

impl Object {
    pub fn add<C, V>(mut self, key: impl ToString, value: C) -> Self
    where
        C: 'static + Validation<Option<V>>,
        V: Default + 'static + for<'de> serde::de::Deserialize<'de>,
    {
        self.validators.push((
            key.to_string(),
            Box::new(ValueValidator {
                v: value,
                _s: PhantomData,
            }),
        ));
        self
    }
}

impl Validation<Map<String, Value>> for Object {
    fn validate(&self, value: &Map<String, Value>) -> Result<(), Error> {
        for v in &self.validators {
            let val = value.get(&v.0).unwrap_or(&Value::Null);
            v.1.validate(val)?;
        }

        Ok(())
    }
}

impl Validation<Value> for Object {
    fn validate(&self, value: &Value) -> Result<(), Error> {
        if let Value::Object(map) = value {
            <Self as Validation<Map<String, Value>>>::validate(self, map)
        } else {
            Err(Error::Required)
        }
    }
}

struct ValueValidator<V, S> {
    v: V,
    _s: PhantomData<S>,
}

impl<V, S> Validation<Value> for ValueValidator<V, S>
where
    V: Validation<S>,
    S: Default + for<'de> serde::de::Deserialize<'de>,
{
    fn validate(&self, val: &Value) -> Result<(), Error> {
        let val = serde_json::from_value(val.clone()).unwrap_or_default();
        self.v.validate(&val)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::*;

    #[test]
    fn test() {
        let o = Object::default()
            .add("name", Valid::<_, String>::new(MaxLen(256)).required())
            .add("age", Valid::new(Max(100).and(Min(18))));

        let value = serde_json::json! ({
            "name": "Hello, World",
            "age": 36
        });

        assert!(o.validate(&value).is_ok());
    }
}
