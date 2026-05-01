/*!
[`Comparison`]: crate::Comparison
[`PartialOrd`]: std::cmp::PartialOrd
[`compare_variables`]: crate::compare_variables

A library for comparing the ordering of variables and producing useful messages.

 */
#![doc = include_str!("../docs/main.md")]

#[cfg(feature = "proc_macro")]
pub use compare_variables_macro::compare_variables;

// ===============================================================================================

/**
Compare the [partial ordering](PartialOrd) of two or three variables and format
the result into a message.

The constructor [`Comparison::new`] compares two to three input values with each
other using the given [`ComparisonOperator`]s and returns an instance of this
struct. The method [`Comparison::is_true`] can then be used to verify whether
the comparison evaluates to true or not. For seamless operation with the `?`
operator, [`Comparison::new_checked`] creates a [`Comparison`] which is wrapped
in [`Ok`] if [`Comparison::is_true`] and in [`Err`] otherwise.

# Examples
```
use compare_variables::{Comparison, ComparisonValue, ComparisonOperator};

fn my_checked_sub(first: usize, second: usize) -> Result<usize, Comparison<usize>> {
    Comparison::new_checked(
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

It is possible to specify variable names which are then included in the error
message string:
```
use compare_variables::{Comparison, ComparisonValue, ComparisonOperator};

let cmp = Comparison::new(
    ComparisonValue::new(1, Some("x")),
    ComparisonOperator::Greater,
    ComparisonValue::new(2, None),
    ComparisonOperator::Equal,
    None,
);
assert_eq!(cmp.to_string(), "`x (value: 1) > 2` is false");

let cmp = Comparison::new(
    ComparisonValue::new(1, Some("x")),
    ComparisonOperator::Greater,
    ComparisonValue::new(2, Some("y")),
    ComparisonOperator::Equal,
    None,
);
assert_eq!(cmp.to_string(), "`x (value: 1) > y (value: 2)` is false");

let cmp = Comparison::new_checked(
    ComparisonValue::new(1, Some("x")),
    ComparisonOperator::Inequal,
    ComparisonValue::new(1, Some("x")),
    ComparisonOperator::Equal,
    None,
);
assert_eq!(cmp.to_string(), "`x (value: 1) != x (value: 1)` is false");
```

## Construction via macro

It is recommended to use the procedural macro [`compare_variables`] to construct
this struct (available via the feature flag  `proc_macro ` which is enabled by
default). With the macro, the previous example simplifies to:
```
use compare_variables::compare_variables;

let x = 1;
let y = 2;

let msg = compare_variables!(x > 2).unwrap_or_else(|x| x);
assert_eq!(err.to_string(), "`x (value: 1) > 2` is false");

let msg = compare_variables!(x > y).unwrap_or_else(|x| x);
assert_eq!(err.to_string(), "`x (value: 1) > y (value: 2)` is false");
```
For more examples, consult the macro documentation.
 */
#[derive(Clone)]
pub struct Comparison<T: PartialOrd> {
    first_val: ComparisonValue<T>,
    comp_first_to_second: ComparisonOperator,
    second_val: ComparisonValue<T>,
    comp_second_to_third: ComparisonOperator,
    third_val: Option<ComparisonValue<T>>,
    is_true: bool,
}

impl<T: PartialOrd> Comparison<T> {
    /**
    Returns a new [`Comparison`] by comparing two or three values.

    # Examples
    ```
    use compare_variables::{Comparison, ComparisonValue, ComparisonOperator, ComparisonErrorTrait};

    let cmp = Comparison::new(
        ComparisonValue::new(1, None),
        ComparisonOperator::Lesser,
        ComparisonValue::new(2, None),
        ComparisonOperator::Equal,
        None,
    );
    assert_eq!(cmp.to_string(), "`1 < 2` is true");

    let cmp = Comparison::new(
        ComparisonValue::new(1, Some("x")),
        ComparisonOperator::Greater,
        ComparisonValue::new(2, None),
        ComparisonOperator::Equal,
        None,
    );
    assert_eq!(cmp.to_string(), "`x (value: 1) > 2` is false");
    ```
    */
    pub fn new(
        first_val: ComparisonValue<T>,
        comp_first_to_second: ComparisonOperator,
        second_val: ComparisonValue<T>,
        comp_second_to_third: ComparisonOperator,
        third_val: Option<ComparisonValue<T>>,
    ) -> Self {
        let mut is_true = true;
        if !comp_first_to_second.is_true(&first_val.value, &second_val.value) {
            is_true = false;
        }
        if let Some(third_val) = third_val.as_ref() {
            if !comp_second_to_third.is_true(&second_val.value, &third_val.value) {
                is_true = false;
            }
        };

        return Self {
            first_val,
            comp_first_to_second,
            second_val,
            comp_second_to_third,
            third_val,
            is_true,
        };
    }

    /**
    Returns a new instance of [`Comparison`] and wraps it in [`Ok`] if
    [`Comparison::is_true`] is true and in [`Err`] otherwise.

    This method is useful when used in conjunction with the `?` operator:

    ```
    use compare_variables::{Comparison, ComparisonValue, ComparisonOperator, ComparisonErrorTrait};

    fn smaller_than_zero(input: i32) -> Result<(), Comparison<i32>> {
        Comparison::new_checked(
            ComparisonValue::new(0, None),
            ComparisonOperator::Greater,
            ComparisonValue::new(input, Some("input")),
            ComparisonOperator::Equal,
            None,
        )?;
    }
    assert!(smaller_than_zero(-3).is_ok());
    assert_eq!(smaller_than_zero(3).unwrap_err().to_string(), "`0 > input (value: -3)` is false");
    ```

    To unwrap the underlying [`Comparison`] regardless of whether the comparison
    evaluated to true or false, the pattern `.unwrap_or_else(|x| x)` can be used
    on the returned result.

    ```
    use compare_variables::{Comparison, ComparisonValue, ComparisonOperator, ComparisonErrorTrait};

    let ok = Comparison::new_checked(
        ComparisonValue::new(0, None),
        ComparisonOperator::Greater,
        ComparisonValue::new(2, None)),
        ComparisonOperator::Equal,
        None,
    );
    assert_eq!(ok.unwrap_or_else(|x| x).to_string(), "`0 > input (value: -3)` is false");

    let err = Comparison::new_checked(
        ComparisonValue::new(0, None),
        ComparisonOperator::Lesser,
        ComparisonValue::new(2, None)),
        ComparisonOperator::Equal,
        None,
    );
    assert_eq!(err.unwrap_or_else(|x| x).to_string(), "`0 > input (value: -3)` is false");
    ```
     */
    pub fn new_checked(
        first_val: ComparisonValue<T>,
        comp_first_to_second: ComparisonOperator,
        second_val: ComparisonValue<T>,
        comp_second_to_third: ComparisonOperator,
        third_val: Option<ComparisonValue<T>>,
    ) -> Result<Self, Self> {
        let this = Self::new(
            first_val,
            comp_first_to_second,
            second_val,
            comp_second_to_third,
            third_val,
        );

        if this.is_true() {
            return Ok(this);
        } else {
            return Err(this);
        }
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

    /**
    Returns the comparison operator between the first and second value.
     */
    pub fn comp_first_to_second(&self) -> ComparisonOperator {
        return self.comp_first_to_second;
    }

    /**
    Returns the comparison operator between the second and third value.
     */
    pub fn comp_second_to_third(&self) -> ComparisonOperator {
        return self.comp_second_to_third;
    }

    /**
    Returns whether the comparison evaluates to true or false.

    To evaluate the comparison, [`Comparison::first_val`] is compared to
    [`Comparison::second_val`] using the [`Comparison::comp_first_to_second`]
    operator. If a [`Comparison::third_val`] has been given, it is compared to
    [`Comparison::second_val`] using the [`Comparison::comp_second_to_third`]
    operator. If both these individual comparisons return true, this method
    returns true as well.

    This method is used within [`Comparison::new_checked`] to decide whether the
    [`Comparison`] should be wrapped in [`Ok`] or [`Err`].
     */
    pub fn is_true(&self) -> bool {
        self.is_true
    }
}

impl<T: PartialOrd + std::fmt::Debug> std::error::Error for Comparison<T> {}

impl<T: PartialOrd + std::fmt::Debug> std::fmt::Debug for Comparison<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return std::fmt::Display::fmt(self, f);
    }
}

impl<T: PartialOrd + std::fmt::Debug> std::fmt::Display for Comparison<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "`{} {} {}",
            self.first_val, self.comp_first_to_second, self.second_val
        )?;

        if let Some(third_val) = self.third_val.as_ref() {
            write!(f, " {} {}", self.comp_second_to_third, third_val)?;
        }
        if self.is_true() {
            write!(f, "` is true")
        } else {
            write!(f, "` is false")
        }
    }
}

/**
Wrapper around the value with an additional optional field for the variable name (if comparing variables instead of literal values).

If a variable name is given, it is used in constructing the error message of [`Comparison`] in addition to the value itself.
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
    Inequal,
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
            ComparisonOperator::Inequal => "!=",
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
            ComparisonOperator::Inequal => return first_val != second_val,
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
This trait is used to abstract a concrete `Comparison<T>` as a trait object where `T` is erased.

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
    This function is especially useful if a [`Comparison`] is used as a trait
    object [`ComparisonErrorTrait`] in order to erase the underlying type.
     */
    fn fmt_first_val(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    /**
    Writes the representation of the second comparison value into the given formatter.
    This function is especially useful if a [`Comparison`] is used as a trait
    object [`ComparisonErrorTrait`] in order to erase the underlying type.
     */
    fn fmt_second_val(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    /**
    Writes the representation of the third comparison value into the given formatter.
    This function is especially useful if a [`Comparison`] is used as a trait
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
    for Comparison<T>
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
