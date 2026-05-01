compare_variables
=================

<!-- This file has ben generated with build.rs by concatenating docs/links.md,
docs/main.md and (if available docs/end.md). Do not modify this file, instead
modify the components. -->

[`Comparison`]: https://docs.rs/compare_variables/0.3.0/compare_variables/struct.Comparison.html
[`PartialOrd`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[`compare_variables`]: https://docs.rs/compare_variables/0.3.0/compare_variables/macro.compare_variables.html

[![Documentation](https://docs.rs/compare_variables/badge.svg)](https://docs.rs/compare_variables)

A library for comparing the ordering of variables and producing useful messages.

The full API documentation is available at https://docs.rs/compare_variables/0.3.0/compare_variables.

> **Feedback welcome!**  
> Found a bug, missing docs, or have a feature request?  
> Please open an issue on [GitHub](https://github.com/StefanMathis/compare_variables.git).

This library can be used to compare the order of two to three variables of any
type `T` implementing [`PartialOrd`] via either the procedural macro
[`compare_variables`] or by directly constructing the underlying [`Comparison`]
struct. [`Comparison`] implements `std::fmt::Display` to create nice messages
with the result of the comparison:

```rust
use compare_variables::{compare_variables, Comparison};

let x = 1.0;
let ok: Comparison<f64> = compare_variables!(0.0 < x <= 1.0).unwrap();
assert_eq!(ok.to_string(), "`0.0 < x (value: 1.0) <= 1.0` is true");

let err: Comparison<f64> = compare_variables!(x > 1.5).unwrap_err();
assert_eq!(err.to_string(), "`x (value: 1.0) > 1.5` is false");
```
As shown in the example, [`compare_variables`] returns a
`Result<Comparison, Comparison>` so it can be used together with the `?`
operator:

```rust
use compare_variables::{compare_variables, Comparison};

fn input_larger_than_10(input: i32) -> Result<(), Comparison<i32>> {
    compare_variables!(input > 10)?;
    return Ok(());
}
assert!(input_larger_than_10(11).is_ok());
assert_eq!(input_larger_than_10(9).unwrap_err().to_string(), "`input (value: 9) > 10` is false");
```

If the underlying [`Comparison`] should be "unpacked", the simple
`.unwrap_or_else(|x| x)` pattern can be used:

```rust
use compare_variables::{compare_variables, Comparison};

fn input_larger_than_10_msg(input: i32) -> String {
    compare_variables!(input > 10).unwrap_or_else(|x| x).to_string()
}

assert_eq!(input_larger_than_10_msg(11), "`input (value: 11) > 10` is true");
assert_eq!(input_larger_than_10_msg(9), "`input (value: 9) > 10` is false");
```

[`compare_variables`] and [`Comparison`] allow evaluating for inequality and
equality as well as (strictly) greater or lesser than, as shown in the examples
below. It is also possible to use named or unnamed struct fields which are then
reported correspondingly in the resulting message.

```rust
use compare_variables::{compare_variables, Comparison};

// Check for equality and inequality
let x = 1.0;
assert!(compare_variables!(1.0 == x < 5.0).is_ok());
assert!(compare_variables!(2.0 != x).is_ok());

// Named and unnamed struct fields:
struct NamedField {
   x: usize
}
let n = NamedField {x: 1};
assert!(compare_variables!(n.x > 0).is_ok());
let err: Comparison<usize> = compare_variables!(n.x > 1).unwrap_err();
assert_eq!(err.to_string(), "`n.x (value: 1) > 1` is false");

struct AnonymousField(i32);
let a = AnonymousField(-5);
let err: Comparison<i32> = compare_variables!(a.0 > 1).unwrap_err();
assert_eq!(err.to_string(), "`a.0 (value: -5) > 1` is false");

// It is also possible to customize the error message via `as` (providing an alias) and `val` (omit the variable name):
let x: u16 = 1;
let y: u16 = 2;
let z: u16 = 3;
let err: Comparison<u16> = compare_variables!(x as arg > val y > z).unwrap_err();
assert_eq!(err.to_string(), "`arg (value: 1) > 2 > z (value: 3)` is false");
```

# Usage without the procedural macro

In order to minimize dependencies, it is possible to use this crate without the
`proc_macro` feature flag. Please see the docstring of [`Comparison`] for
details.

```rust
use compare_variables::{compare_variables, Comparison, ComparisonValue, ComparisonOperator};

let x = 1;
let msg_macro = compare_variables!(x > 2).unwrap_or_else(|x| x);
let msg_no_macro = Comparison::new(
    ComparisonValue::new(1, Some("x")),
    ComparisonOperator::Greater,
    ComparisonValue::new(2, None),
    ComparisonOperator::Equal,
    None,
);
assert_eq!(msg_macro.to_string(), msg_no_macro.to_string());
```
