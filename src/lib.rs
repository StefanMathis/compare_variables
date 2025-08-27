#[cfg(feature = "proc_macro")]
pub use compare_variables_macro::compare_variables;

// ===============================================================================================

/**
This struct creates an error message for an comparison error of an implementor of the `PartialOrd` trait.
```
use compare_variables::{ComparisonError, ComparisonValueOwned, ComparisonOperator};

let message = ComparisonError::new(
    ComparisonValueOwned::new(1.5, Some("x".to_owned())),
    ComparisonOperator::Equal,
    ComparisonValueOwned::new(1.0, None),
    ComparisonOperator::Equal,
    None,
);
assert_eq!(
    format!("{}", message),
    "constraint `x (value: 1.5) == 1.0` is not fulfilled"
);
```

Usually, this error is not build manually, but created via the `check` method or the corresponding macro. Both check the input
values and only construct the error if the input value is really out of bounds.

# Construction via method
```
use compare_variables::{ComparisonError, ComparisonValue, ComparisonOperator};

let error = ComparisonError::check(
    ComparisonValue::new(0.0, None),
    ComparisonOperator::Lesser,
    ComparisonValue::new(2.0, Some("argument_name")),
    ComparisonOperator::LesserOrEqual,
    Some(ComparisonValue::new(1.0, None)),
);
assert!(error.is_err());
```

# Construction via macro
```
use compare_variables::compare_variables;

let argument = 2.0;
assert!(compare_variables!(argument < 1.5).is_err());
assert!(compare_variables!(argument == argument).is_ok());
```
For more examples, consult the macro documentation.
 */
#[derive(Clone)]
pub struct ComparisonError<T: PartialOrd> {
    first_arg: ComparisonValueOwned<T>,
    relation_first_to_second: ComparisonOperator,
    second_arg: ComparisonValueOwned<T>,
    relation_second_to_third: ComparisonOperator,
    third_arg: Option<ComparisonValueOwned<T>>,
}

impl<T: PartialOrd> ComparisonError<T> {
    pub fn new(
        first_arg: ComparisonValueOwned<T>,
        relation_first_to_second: ComparisonOperator,
        second_arg: ComparisonValueOwned<T>,
        relation_second_to_third: ComparisonOperator,
        third_arg: Option<ComparisonValueOwned<T>>,
    ) -> Self {
        return Self {
            first_arg,
            relation_first_to_second,
            second_arg,
            relation_second_to_third,
            third_arg,
        };
    }

    /**
    Check if the given value is inside the specified bounds. If not, return `Self` as an error.

    The bounds are specified as tuple `(value, include_equality)`. The first tuple element is the actual bound.
    The second element defines whether the input value is still inside the bound if it equals the bound.
     */
    pub fn check(
        first_arg: ComparisonValue<T>,
        relation_first_to_second: ComparisonOperator,
        second_arg: ComparisonValue<T>,
        relation_second_to_third: ComparisonOperator,
        third_arg: Option<ComparisonValue<T>>,
    ) -> Result<(), Self> {
        // Check the relationship between the first and second argument
        if !relation_first_to_second.is_true(&first_arg.value, &second_arg.value) {
            let third_arg = match third_arg {
                Some(arg) => Some(arg.into()),
                None => None,
            };
            return Err(Self {
                first_arg: first_arg.into(),
                relation_first_to_second,
                second_arg: second_arg.into(),
                relation_second_to_third,
                third_arg,
            });
        }

        if let Some(third_arg) = third_arg {
            if !relation_second_to_third.is_true(&second_arg.value, &third_arg.value) {
                return Err(Self {
                    first_arg: first_arg.into(),
                    relation_first_to_second,
                    second_arg: second_arg.into(),
                    relation_second_to_third,
                    third_arg: Some(third_arg.into()),
                });
            }
        };

        return Ok(());
    }
}

impl<T: PartialOrd + std::fmt::Debug> ComparisonError<T> {
    fn format_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn add_arg_to_string<T: PartialOrd + std::fmt::Debug>(
            message: &mut String,
            arg: &ComparisonValueOwned<T>,
        ) {
            let arg_string = format!("{:?}", arg.value);
            if let Some(name) = &arg.variable_name {
                message.push_str(name);
                message.push_str(" (value: ");
                message.push_str(&arg_string);
                message.push_str(")");
            } else {
                message.push_str(&arg_string);
            }
        }

        let mut message = "`".to_owned();

        // Build the message
        add_arg_to_string(&mut message, &self.first_arg);
        message.push_str(" ");
        message.push_str(self.relation_first_to_second.as_str());
        message.push_str(" ");
        add_arg_to_string(&mut message, &self.second_arg);
        if let Some(third_arg) = self.third_arg.as_ref() {
            message.push_str(" ");
            message.push_str(self.relation_second_to_third.as_str());
            message.push_str(" ");
            add_arg_to_string(&mut message, &third_arg);
        }
        message.push_str("`");

        write!(f, "constraint {message} is not fulfilled")
    }
}

impl<T: PartialOrd + std::fmt::Debug> std::error::Error for ComparisonError<T> {}

impl<T: PartialOrd + std::fmt::Debug> std::fmt::Debug for ComparisonError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format_message(f)
    }
}

impl<T: PartialOrd + std::fmt::Debug> std::fmt::Display for ComparisonError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format_message(f)
    }
}

#[derive(Debug, Clone)]
pub struct ComparisonValueOwned<T: PartialOrd> {
    value: T,
    variable_name: Option<String>,
}

impl<T: PartialOrd> ComparisonValueOwned<T> {
    pub fn new(value: T, variable_name: Option<String>) -> Self {
        return Self {
            value,
            variable_name,
        };
    }
}

#[derive(Debug, Clone)]
pub struct ComparisonValue<'a, T: PartialOrd> {
    value: T,
    variable_name: Option<&'a str>,
}

impl<'a, T: PartialOrd> ComparisonValue<'a, T> {
    pub fn new(value: T, variable_name: Option<&'a str>) -> Self {
        return Self {
            value,
            variable_name,
        };
    }
}

impl<T: PartialOrd> From<ComparisonValue<'_, T>> for ComparisonValueOwned<T> {
    fn from(input: ComparisonValue<'_, T>) -> Self {
        let value = input.value;
        let variable_name = match input.variable_name {
            Some(str) => Some(str.to_string()),
            None => None,
        };
        return Self {
            value,
            variable_name,
        };
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ComparisonOperator {
    Lesser,
    LesserOrEqual,
    Equal,
    GreaterOrEqual,
    Greater,
}

impl ComparisonOperator {
    fn as_str(&self) -> &str {
        match self {
            ComparisonOperator::Lesser => "<",
            ComparisonOperator::LesserOrEqual => "<=",
            ComparisonOperator::Equal => "==",
            ComparisonOperator::GreaterOrEqual => ">=",
            ComparisonOperator::Greater => ">",
        }
    }

    /**
    Check whether the relationship between `first_arg` and `second_arg` defined by `self` holds true.
     */
    fn is_true<T: PartialOrd>(&self, first_arg: &T, second_arg: &T) -> bool {
        match self {
            ComparisonOperator::Lesser => return first_arg < second_arg,
            ComparisonOperator::LesserOrEqual => return first_arg <= second_arg,
            ComparisonOperator::Equal => return first_arg == second_arg,
            ComparisonOperator::GreaterOrEqual => return first_arg >= second_arg,
            ComparisonOperator::Greater => return first_arg > second_arg,
        }
    }
}

/// Helper trait which allows to treat ComparisonError's as trait objects.
pub trait ComparisonErrorTrait: std::error::Error + Sync + Send {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: PartialOrd + std::fmt::Debug + Sync + Send + 'static> ComparisonErrorTrait
    for ComparisonError<T>
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_arg_message() {
        let message = ComparisonError::check(
            ComparisonValue::new(1.0, None),
            ComparisonOperator::Equal,
            ComparisonValue::new(2.0, Some("argument")),
            ComparisonOperator::Equal,
            None,
        )
        .unwrap_err();
        assert_eq!(
            format!("{message}"),
            "constraint `1.0 == argument (value: 2.0)` is not fulfilled"
        );

        let message = ComparisonError::check(
            ComparisonValue::new(2.0, None),
            ComparisonOperator::Equal,
            ComparisonValue::new(2.0, Some("argument")),
            ComparisonOperator::Greater,
            Some(ComparisonValue::new(2.0, Some("argument"))),
        )
        .unwrap_err();
        assert_eq!(
            format!("{message}"),
            "constraint `2.0 == argument (value: 2.0) > argument (value: 2.0)` is not fulfilled"
        );

        let message = ComparisonError::check(
            ComparisonValue::new(2.0, None),
            ComparisonOperator::Equal,
            ComparisonValue::new(2.0, Some("argument")),
            ComparisonOperator::Greater,
            Some(ComparisonValue::new(2.0, None)),
        )
        .unwrap_err();
        assert_eq!(
            format!("{message}"),
            "constraint `2.0 == argument (value: 2.0) > 2.0` is not fulfilled"
        );

        let message = ComparisonError::check(
            ComparisonValue::new(2.0, None),
            ComparisonOperator::Lesser,
            ComparisonValue::new(2.0, None),
            ComparisonOperator::Greater,
            Some(ComparisonValue::new(2.0, None)),
        )
        .unwrap_err();
        assert_eq!(
            format!("{message}"),
            "constraint `2.0 < 2.0 > 2.0` is not fulfilled"
        );

        let message = ComparisonError::check(
            ComparisonValue::new(0.0, None),
            ComparisonOperator::Lesser,
            ComparisonValue::new(2.0, None),
            ComparisonOperator::LesserOrEqual,
            Some(ComparisonValue::new(1.0, None)),
        )
        .unwrap_err();
        assert_eq!(
            format!("{message}"),
            "constraint `0.0 < 2.0 <= 1.0` is not fulfilled"
        );

        let _ = ComparisonError::check(
            ComparisonValue::new(2.0, None),
            ComparisonOperator::LesserOrEqual,
            ComparisonValue::new(2.0, None),
            ComparisonOperator::GreaterOrEqual,
            Some(ComparisonValue::new(2.0, None)),
        )
        .is_ok();
    }
}
