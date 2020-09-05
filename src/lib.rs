mod error;
#[cfg(feature = "json")]
mod object;
mod types;
mod validators;

pub use self::{error::*, types::*, validators::*};

#[cfg(feature = "json")]
pub use object::*;

#[cfg(test)]
mod test {
    pub use super::*;

    #[test]
    fn test() {
        Valid::new(Min(2).and(Max(4)))
            .and(validation(|i| {
                if i % 2 == 0 {
                    Ok(())
                } else {
                    Err(Error::Required)
                }
            }))
            .validate(&2);
        // required()
        //     .inner(MinLen(6).and(MaxLen(10)))
        //     .validate(&Some(vec!["rasmus1", "rasmus2"]))
        //     .expect("required");
        Valid::new(All.inner(MaxLen(6)))
            .validate(&vec!["rasmus", "rasmus"])
            .expect("inner");
        Valid::<_, _>::new(MinLen(10)).validate(&"Hello, World!");
        MinLen(2)
            .inner(validation(|v| {
                if v == &"rasmus" {
                    Ok(())
                } else {
                    Err(Error::Other("Not rasmus".into()))
                }
            }))
            .validate(&vec!["rasmus", "rasmus"])
            .expect("min");
        // ContainerValidator::new(Required, MinLen(6))
        //     .validate(&vec!["rasmus1", "rasmus2"])
        //     .expect("required");
        // assert!(required().validate(&Some("Hello")).is_ok());
        // assert!(required().validate(&Some("")).is_ok());
        // assert!(required().validate(&Option::<i32>::None).is_err());
        // assert!(required().validate(&Option::<i32>::Some(42)).is_ok());

        // assert!(required().validate(&Option::<i32>::Some(42)).is_ok());
        assert!(Min(18).and(Max(36)).validate(&18).is_ok());

        assert!(MinLen(5).validate(&"Hello").is_ok());
    }

    #[test]
    fn and() {
        ValidationList::default()
            .push(MinLen(0))
            .push(MaxLen(10))
            .validate(&"Hello, World!");
        MinLen(2).and(MaxLen(10)).validate(&"Hello, World!");
    }
}
