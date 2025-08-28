compare_variables
=================

A library for comparing the ordering of variables and producing useful error messages.

[`ComparisonError`]: https://docs.rs/compare_variables/0.1.0/compare_variables/struct.ComparisonError.html
[`PartialOrd`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[`compare_variables`]: https://docs.rs/compare_variables/0.1.0/compare_variables/macro.compare_variables.html

This library is based on the struct [`ComparisonError`], which can be used to compare the partial ordering
of two to three variables of any type `T` which implements the [`PartialOrd`] trait. If the comparison evaluates
to false, [`ComparisonError`] can be formatted into a nice error message. To simplify the usage, the procedural
macro [`compare_variables`] is provided via the feature flag **proc_macro** (enabled by default).

The full API documentation is available at [https://docs.rs/compare_variables/0.1.0/compare_variables/](https://docs.rs/compare_variables/0.1.0/compare_variables/).

# Examples

```rust
use compare_variables::{compare_variables, ComparisonError};

let x = 1.0;
assert!(compare_variables!(0.0 < x <= 1.0).is_ok());

let err: ComparisonError<f64> = compare_variables!(x > 1.5).unwrap_err();
assert_eq!(err.to_string(), "`x (value: 1.0) > 1.5` is false");

assert!(compare_variables!(1.0 == x < 5.0).is_ok());

// Named struct fields can also be used:
struct NamedField {
   x: usize
}

let n = NamedField {x: 1};
assert!(compare_variables!(n.x > 0).is_ok());
let err: ComparisonError<usize> = compare_variables!(n.x > 1).unwrap_err();
assert_eq!(err.to_string(), "`n.x (value: 1) > 1` is false");

// It is also possible to customize the error message via `as` (providing an alias) and `raw` (omit the variable name):
let x: u16 = 1;
let y: u16 = 2;
let z: u16 = 3;
let err: ComparisonError<u16> = compare_variables!(x as arg > raw y > z).unwrap_err();
assert_eq!(err.to_string(), "`arg (value: 1) > 2 > z (value: 3)` is false");
```

# Usage without the procedural macro

In order to minimize dependencies, it is possible to use this crate without the **proc_macro** feature flag.
Please see the docstring of [`ComparisonError`] for details.

```rust
use compare_variables::{compare_variables, ComparisonError, ComparisonValue, ComparisonOperator};

let x = 1;
let err_macro = compare_variables!(x > 2).unwrap_err();
let err_no_macro = ComparisonError::new(
    ComparisonValue::new(1, Some("x")),
    ComparisonOperator::Greater,
    ComparisonValue::new(2, None),
    ComparisonOperator::Equal,
    None,
).unwrap_err();
assert_eq!(err_macro.to_string(), err_no_macro.to_string());
```
