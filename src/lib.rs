#![doc = include_str!("../README.md")]

#[cfg(feature = "proc_macro")]
pub use compare_variables_macro::compare_variables;

// ===============================================================================================

/**
Compare the [partial ordering](https://en.wikipedia.org/wiki/Partially_ordered_set) of two or three values and format the result into a message.

The constructor [`ComparisonError::new`] compares two to three input values with each other using the given [`ComparisonOperator`]s and returns an instance
of this struct as an `Result::Err(ComparisonError)` if the comparison returned "false" (otherwise, [`ComparisonError::new`] returns `Result::Ok(())`).
This is done in order to allow seamless operation with the `?` operator.

# Examples
```
use compare_variables::{ComparisonError, ComparisonValue, ComparisonOperator};

fn my_checked_sub(first: usize, second: usize) -> Result<usize, ComparisonError<usize>> {
    ComparisonError::new(
        ComparisonValue::new(first, None),
        ComparisonOperator::Greater,
        ComparisonValue::new(second, None),
        ComparisonOperator::Equal,
        None,
    )?;
    return Ok(first - second);
}

assert_eq!(my_checked_sub(2, 1).unwrap(), 1);
let err = my_checked_sub(1, 2).unwrap_err();
assert_eq!(err.to_string(), "`1 > 2` is false");
```

## Variable names

It is possible to specify variable names which are then included in the error message string:
```
use compare_variables::{ComparisonError, ComparisonValue, ComparisonOperator};

let err = ComparisonError::new(
    ComparisonValue::new(1, Some("x")),
    ComparisonOperator::Greater,
    ComparisonValue::new(2, None),
    ComparisonOperator::Equal,
    None,
).unwrap_err();
assert_eq!(err.to_string(), "`x (value: 1) > 2` is false");

let err = ComparisonError::new(
    ComparisonValue::new(1, Some("x")),
    ComparisonOperator::Greater,
    ComparisonValue::new(2, Some("y")),
    ComparisonOperator::Equal,
    None,
).unwrap_err();
assert_eq!(err.to_string(), "`x (value: 1) > y (value: 2)` is false");
```

## Construction via macro

It is recommended to use the procedural macro [`compare_variables`] to construct this struct
(available via the feature flag **proc_macro** which is enabled by default). With the macro,
the previous example is simplified to:
```
use compare_variables::compare_variables;

let x = 1;
let y = 2;

let err = compare_variables!(x > 2).unwrap_err();
assert_eq!(err.to_string(), "`x (value: 1) > 2` is false");

let err = compare_variables!(x > y).unwrap_err();
assert_eq!(err.to_string(), "`x (value: 1) > y (value: 2)` is false");
```
For more examples, consult the macro documentation.

## Customize error messages

The error messages are build by concatenating the format strings of the given [`ComparisonValue`]s and [`ComparisonOperator`]s.
These components can be accessed individually in order to build custom error messages:

```
use compare_variables::{ComparisonError, ComparisonValue, ComparisonOperator, ComparisonErrorTrait};

let err = ComparisonError::new(
    ComparisonValue::new(1, None),
    ComparisonOperator::Greater,
    ComparisonValue::new(2, None),
    ComparisonOperator::Equal,
    None,
).unwrap_err();

let my_error_msg = format!("Condition `{} {} {}` is not fulfilled", err.first_val(), err.comp_first_to_second(), err.second_val());
assert_eq!(my_error_msg, "Condition `1 > 2` is not fulfilled");
```
 */
#[derive(Clone)]
pub struct ComparisonError<T: PartialOrd> {
    first_val: ComparisonValue<T>,
    comp_first_to_second: ComparisonOperator,
    second_val: ComparisonValue<T>,
    comp_second_to_third: ComparisonOperator,
    third_val: Option<ComparisonValue<T>>,
}

impl<T: PartialOrd> ComparisonError<T> {
    /**
    Constructs a new instance of [`ComparisonError`] if the comparison defined by the input arguments fails.

    The `first_val` is compared to the `second_val` using the `comp_first_to_second` operator. If a `third_val` is given,
    it is compared to the second argument using the `comp_second_to_third` operator. If no `third_val` is given,
    `comp_second_to_third` is not used internally (and can therefore be chosen arbitrarily). If one of these comparisons
    evaluate to false, an instance of [`ComparisonError`] is returned as an `Result::Err(ComparisonError)`.
    Otherwise, [`ComparisonError::new`] returns `Result::Ok(())`). This is done in order to allow seamless operation with the `?`
    operator.

    For examples, see the docstring of [`ComparisonError`].
     */
    pub fn new(
        first_val: ComparisonValue<T>,
        comp_first_to_second: ComparisonOperator,
        second_val: ComparisonValue<T>,
        comp_second_to_third: ComparisonOperator,
        third_val: Option<ComparisonValue<T>>,
    ) -> Result<(), Self> {
        // Check the relationship between the first and second argument
        if !comp_first_to_second.is_true(&first_val.value, &second_val.value) {
            return Err(Self {
                first_val,
                comp_first_to_second,
                second_val,
                comp_second_to_third,
                third_val,
            });
        }

        if let Some(third_val) = third_val {
            if !comp_second_to_third.is_true(&second_val.value, &third_val.value) {
                return Err(Self {
                    first_val,
                    comp_first_to_second,
                    second_val,
                    comp_second_to_third,
                    third_val: Some(third_val),
                });
            }
        };

        return Ok(());
    }

    /**
    Returns a reference to the first value.
     */
    pub fn first_val(&self) -> &ComparisonValue<T> {
        return &self.first_val;
    }

    /**
    Returns a reference to the second value.
     */
    pub fn second_val(&self) -> &ComparisonValue<T> {
        return &self.second_val;
    }

    /**
    Returns a reference to the third value, if one was given.
     */
    pub fn third_val(&self) -> Option<&ComparisonValue<T>> {
        return self.third_val.as_ref();
    }
}

impl<T: PartialOrd + std::fmt::Debug> std::error::Error for ComparisonError<T> {}

impl<T: PartialOrd + std::fmt::Debug> std::fmt::Debug for ComparisonError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return std::fmt::Display::fmt(self, f);
    }
}

impl<T: PartialOrd + std::fmt::Debug> std::fmt::Display for ComparisonError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "`{} {} {}",
            self.first_val, self.comp_first_to_second, self.second_val
        )?;

        if let Some(third_val) = self.third_val.as_ref() {
            write!(f, " {} {}", self.comp_second_to_third, third_val)?;
        }
        write!(f, "` is false")
    }
}

/**
Wrapper around the value with an additional optional field for the variable name (if comparing variables instead of literal values).

If a variable name is given, it is used in constructing the error message of [`ComparisonError`] in addition to the value itself.
 */
#[derive(Debug, Clone)]
pub struct ComparisonValue<T: PartialOrd> {
    pub value: T,
    pub variable_name: Option<&'static str>,
}

impl<T: PartialOrd> ComparisonValue<T> {
    /**
    Returns a new instance of [`ComparisonValue`].
     */
    pub fn new(value: T, variable_name: Option<&'static str>) -> Self {
        return Self {
            value,
            variable_name,
        };
    }
}

impl<T: PartialOrd + std::fmt::Debug> std::fmt::Display for ComparisonValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = self.variable_name.as_ref() {
            write!(f, "{name} (value: ")?;
            write!(f, "{:?})", self.value)?;
        } else {
            write!(f, "{:?}", self.value)?;
        }
        return Ok(());
    }
}

/**
Defines a comparison between two values.

See the docstring of [`ComparisonOperator::is_true`] for an example on how to apply this enum for a comparison.
 */
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
    /**
    Formats the enum value into a static string slice.
     */
    pub fn as_str(&self) -> &'static str {
        match self {
            ComparisonOperator::Lesser => "<",
            ComparisonOperator::LesserOrEqual => "<=",
            ComparisonOperator::Equal => "==",
            ComparisonOperator::GreaterOrEqual => ">=",
            ComparisonOperator::Greater => ">",
        }
    }

    /**
    Compares the ordering of two values.

    Returns the output of the following comparison: `first_val self second_val`.
    This is best illustrated via the following examples:
    ```
    use compare_variables::ComparisonOperator;

    assert!(ComparisonOperator::Lesser.is_true(&1, &2));
    assert!(ComparisonOperator::LesserOrEqual.is_true(&2.0, &2.0));
    assert!(!ComparisonOperator::Greater.is_true(&-1i32, &1i32));
    ```
     */
    pub fn is_true<T: PartialOrd>(&self, first_val: &T, second_val: &T) -> bool {
        match self {
            ComparisonOperator::Lesser => return first_val < second_val,
            ComparisonOperator::LesserOrEqual => return first_val <= second_val,
            ComparisonOperator::Equal => return first_val == second_val,
            ComparisonOperator::GreaterOrEqual => return first_val >= second_val,
            ComparisonOperator::Greater => return first_val > second_val,
        }
    }
}

impl From<&ComparisonOperator> for &'static str {
    fn from(value: &ComparisonOperator) -> Self {
        return value.as_str();
    }
}

impl AsRef<str> for ComparisonOperator {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/**
This trait is used to abstract a concrete `ComparisonError<T>` as a trait object where `T` is erased.

As an example for using this trait, let's assume a function performs two comparisons on different types
but only wants to return a single error type:
```
use compare_variables::{compare_variables, ComparisonErrorTrait};

fn example() -> Result<(), Box<dyn ComparisonErrorTrait>> {
    compare_variables!(1 > 2).map_err(|e| Box::new(e) as Box<dyn ComparisonErrorTrait>)?;
    compare_variables!(1.0 > 2.0).map_err(|e| Box::new(e) as Box<dyn ComparisonErrorTrait>)?;
    return Ok(());
}
```
 */
pub trait ComparisonErrorTrait: std::error::Error + Sync + Send + std::any::Any {
    /**
    Writes the representation of the first comparison value into the given formatter.
    This function is especially useful if a [`ComparisonError`] is used as a trait
    object [`ComparisonErrorTrait`] in order to erase the underlying type.
     */
    fn fmt_first_val(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    /**
    Writes the representation of the second comparison value into the given formatter.
    This function is especially useful if a [`ComparisonError`] is used as a trait
    object [`ComparisonErrorTrait`] in order to erase the underlying type.
     */
    fn fmt_second_val(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    /**
    Writes the representation of the third comparison value into the given formatter.
    This function is especially useful if a [`ComparisonError`] is used as a trait
    object [`ComparisonErrorTrait`] in order to erase the underlying type.

    If no third value exists, this function returns an error.
     */
    fn fmt_third_val(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    /**
    Returns the comparison operator between the first and the second value.
     */
    fn comp_first_to_second(&self) -> ComparisonOperator;

    /**
    Returns the comparison operator between the second and the third value.
     */
    fn comp_second_to_third(&self) -> ComparisonOperator;
}

impl<T: PartialOrd + std::fmt::Debug + Sync + Send + 'static> ComparisonErrorTrait
    for ComparisonError<T>
{
    fn fmt_first_val(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return std::fmt::Display::fmt(self.first_val(), f);
    }

    fn fmt_second_val(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return std::fmt::Display::fmt(self.second_val(), f);
    }

    fn fmt_third_val(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.third_val() {
            Some(v) => return std::fmt::Display::fmt(v, f),
            None => return Err(std::fmt::Error),
        }
    }

    fn comp_first_to_second(&self) -> ComparisonOperator {
        return self.comp_first_to_second;
    }

    fn comp_second_to_third(&self) -> ComparisonOperator {
        return self.comp_second_to_third;
    }
}
