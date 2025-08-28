#![cfg_attr(debug_assertions, allow(unused_imports))]

use proc_macro::{self, TokenStream};
use proc_macro_error::abort;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::ext::IdentExt;
use syn::parse::Parse;
use syn::{Token, parse_macro_input};

/**
A macro to compare types which implement `PartialOrd`.

# Overview

This macro performs comparison between two or three values of any type `T` which implements  `PartialOrd`.
If the comparison evaluates to `true`, the macro returns `Result::Ok(())`, otherwise it returns a
`Result::Err(compare_variables::ComparisonError)` which can be formatted into a string showcasing
the failed comparison.

The macro syntax is
```math
compare_variables(x * y)
```
for comparing two values and
```math
compare_variables(x * y * z)
```
for comparing three values with `*` being any of the comparison operators `<, <=, ==, >, >=`.

`x`, `y` and `z` can be either a literal (e.g. `3.141` or `1e10`) or a variable:

```rust
use compare_variables::compare_variables;

assert!(compare_variables!(2.0 > 1.5).is_ok());

let x = 1;
let y = 2;
assert!(compare_variables!(x < 2 == y).is_ok());
assert!(compare_variables!(x >= 2).is_err());
```

It is possible to combine the macro with the question mark operator:
```rust
use compare_variables::{compare_variables, ComparisonError};

fn checked_sub(left: u16, right: u16) -> Result<u16, ComparisonError<u16>> {
    compare_variables!(left >= right)?;
    return Ok(left - right);
}

assert_eq!(checked_sub(2, 1).unwrap(), 1);
assert_eq!(checked_sub(2, 2).unwrap(), 0);
assert!(checked_sub(2, 3).is_err());
```

It is also possible to use named struct fields as inputs:

```
use compare_variables::compare_variables;

struct NamedField {
   x: f64
}

let n = NamedField {x: 1.0};
assert!(compare_variables!(n.x > -1.0).is_ok());
assert!(compare_variables!(n.x > 1.0).is_err());
```

# Error message

The error message is created via the struct [`ComparisonError`](https://docs.rs/compare_variables/0.1.0/compare_variables/struct.ComparisonError.html).
Please refer to its documentation for more details. The keywords `raw` and `as` allow to customize the treatment of variable names in the error message:

```
use compare_variables::compare_variables;

// Error message with literals only
let err = compare_variables!(5i32 <= -1i32);
assert_eq!(err.unwrap_err().to_string(), "`5 <= -1` is false");

let x = 1;
let y = 2;

// Default error message
let err = compare_variables!(x > y);
assert_eq!(err.unwrap_err().to_string(), "`x (value: 1) > y (value: 2)` is false");

// Rename x in the error message
let err = compare_variables!(x as variable > y);
assert_eq!(err.unwrap_err().to_string(), "`variable (value: 1) > y (value: 2)` is false");

// Only display the underlying value, not the variable name:
let err = compare_variables!(raw x > y);
assert_eq!(err.unwrap_err().to_string(), "`1 > y (value: 2)` is false");

// `as` is ignored if used together with `raw`:
let err = compare_variables!(raw x as variable > y);
assert_eq!(err.unwrap_err().to_string(), "`1 > y (value: 2)` is false");
```

# Examples

```rust
use compare_variables::compare_variables;

// Different float types:
assert!(compare_variables!(1.5 < 2.0 == 3.0).is_err());
assert!(compare_variables!(1.7f32 == 1.7f32).is_ok());
let f = 2.0;
assert!(compare_variables!(f < 5.2).is_ok());
assert!(compare_variables!(f as f_var == raw f).is_ok());

// Signed and unsigned integers
assert!(compare_variables!(1i32 >= 2i32).is_err());
let u = 3usize;
assert!(compare_variables!(2usize < u < 4usize).is_ok());
let i = 15i64;
assert!(compare_variables!(-10i64 <= i).is_ok());
assert!(compare_variables!(-10i64 <= i <= 10i64).is_err());

// Custom types implementing `PartialOrd`.
// Clone and Copy are not required and are only used here for the example.
#[derive(PartialEq, Clone, Copy)]
struct MyFloat64(f64);

impl PartialOrd for MyFloat64 {
   fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
       return self.0.partial_cmp(&other.0);
   }
}

let myfloat1 = MyFloat64(1.0);
let myfloat2 = MyFloat64(2.0);
assert!(compare_variables!(myfloat1 == myfloat1).is_ok());
assert!(compare_variables!(myfloat1 <= myfloat2).is_ok());
assert!(compare_variables!(myfloat1 >= myfloat2).is_err());
```
 */
#[proc_macro]
pub fn compare_variables(input: TokenStream) -> TokenStream {
    let comparison_error_info: ComparisonErrorInfo = parse_macro_input!(input);

    let first_arg = comparison_error_info.first_arg.as_token_stream();
    let relation_first_to_second = comparison_error_info
        .relation_first_to_second
        .as_token_stream();
    let second_arg = comparison_error_info.second_arg.as_token_stream();
    let relation_second_to_third = comparison_error_info
        .relation_second_to_third
        .as_token_stream();
    let third_arg = match comparison_error_info.third_arg {
        Some(arg) => {
            let ts = arg.as_token_stream();
            quote! {Some(#ts)}
        }
        None => quote! {None},
    };

    // Build the input for the compare_variables function
    let stream = quote! {
        compare_variables::ComparisonError::new(
            #first_arg,
            #relation_first_to_second,
            #second_arg,
            #relation_second_to_third,
            #third_arg,
        )
    };

    return TokenStream::from(stream);
}

#[repr(u8)]
enum ComparisonError {
    Lesser,
    LesserOrEqual,
    Equal,
    GreaterOrEqual,
    Greater,
}

impl ComparisonError {
    fn as_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            ComparisonError::Lesser => {
                quote! {
                    compare_variables::ComparisonOperator::Lesser
                }
            }
            ComparisonError::LesserOrEqual => {
                quote! {
                    compare_variables::ComparisonOperator::LesserOrEqual
                }
            }
            ComparisonError::Equal => {
                quote! {
                    compare_variables::ComparisonOperator::Equal
                }
            }
            ComparisonError::GreaterOrEqual => {
                quote! {
                    compare_variables::ComparisonOperator::GreaterOrEqual
                }
            }
            ComparisonError::Greater => {
                quote! {
                    compare_variables::ComparisonOperator::Greater
                }
            }
        }
    }
}

enum VariableOrLiteral {
    Other {
        arg_names: Vec<syn::Ident>,
        arg_names_display: Vec<syn::Ident>,
    },
    LitFloat(syn::LitFloat),
    LitInt(syn::LitInt),
}

impl VariableOrLiteral {
    fn as_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            VariableOrLiteral::Other {
                arg_names,
                arg_names_display,
            } => {
                // Build a token stream out of arg_name and arg_name_display, using . as a delimiter
                let arg_value = arg_names
                    .into_iter()
                    .map(|ident| ident.to_string())
                    .collect::<Vec<String>>()
                    .join(".");
                let arg_value_ts: TokenStream2 = match str::parse::<TokenStream2>(&arg_value) {
                    Ok(ts) => ts,
                    Err(_) => abort!(
                        Span::call_site(),
                        format!("could not interpret {arg_value} as rust code")
                    ),
                };
                if arg_names_display.is_empty() {
                    quote! {
                        compare_variables::ComparisonValue::new(#arg_value_ts, None)
                    }
                } else {
                    let arg_name_display = arg_names_display
                        .into_iter()
                        .map(|ident| ident.to_string())
                        .collect::<Vec<String>>()
                        .join(".");
                    quote! {
                        compare_variables::ComparisonValue::new(#arg_value_ts, Some(#arg_name_display))
                    }
                }
            }
            VariableOrLiteral::LitFloat(lit) => {
                quote! {
                    compare_variables::ComparisonValue::new(#lit, None)
                }
            }
            VariableOrLiteral::LitInt(lit) => {
                quote! {
                    compare_variables::ComparisonValue::new(#lit, None)
                }
            }
        }
    }
}

// Parser for the compare_variables macro
struct ComparisonErrorInfo {
    first_arg: VariableOrLiteral,
    relation_first_to_second: ComparisonError,
    second_arg: VariableOrLiteral,
    relation_second_to_third: ComparisonError,
    third_arg: Option<VariableOrLiteral>,
}

impl Parse for ComparisonErrorInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        fn parse_arg(input: &syn::parse::ParseStream) -> syn::Result<VariableOrLiteral> {
            if input.peek(syn::LitFloat) {
                // Parse the float literal
                let val = input.parse::<syn::LitFloat>()?;
                return Ok(VariableOrLiteral::LitFloat(val));
            } else if input.peek(syn::LitInt) {
                // Parse the float literal
                let val = input.parse::<syn::LitInt>()?;
                return Ok(VariableOrLiteral::LitInt(val));
            } else {
                // If the first token is "raw", do not display the variable name
                let no_arg_name_display = input.peek(Token![raw]);
                if no_arg_name_display {
                    // Remove the "raw" token
                    let _ = input.parse::<Token![raw]>()?;
                }

                // Resolve field accesses like self.field or variable.field.field
                let mut arg_names: Vec<Ident> = Vec::new();
                loop {
                    let arg_name: Ident = input.call(Ident::parse_any)?; // parse_any also handles stuff like self
                    arg_names.push(arg_name);

                    if input.peek(Token![.]) {
                        // Throw the token away
                        let _ = input.parse::<Token![.]>()?;
                    } else {
                        // Field access is done ==> Finish the loop
                        break;
                    }
                }

                let arg_names_display = if input.peek(Token![as]) {
                    input.parse::<Token![as]>()?;
                    let mut arg_names_display: Vec<Ident> = Vec::new();
                    loop {
                        let arg_name: Ident = input.call(Ident::parse_any)?; // parse_any also handles stuff like self
                        if !no_arg_name_display {
                            arg_names_display.push(arg_name);
                        }

                        if input.peek(Token![.]) {
                            // Throw the token away
                            let _ = input.parse::<Token![.]>()?;
                        } else {
                            // Field access is done ==> Finish the loop
                            break;
                        }
                    }
                    arg_names_display
                } else {
                    if no_arg_name_display {
                        Vec::new()
                    } else {
                        arg_names.clone()
                    }
                };
                return Ok(VariableOrLiteral::Other {
                    arg_names,
                    arg_names_display,
                });
            }
        }

        fn parse_comparison_operator(
            input: &syn::parse::ParseStream,
        ) -> syn::Result<ComparisonError> {
            // If Token![<] is tested before Token![<=], then "<" is parsed, leaving only "=". This will then lead to a compile error.
            if input.peek(Token![<=]) {
                input.parse::<Token![<=]>()?;
                Ok(ComparisonError::LesserOrEqual)
            } else if input.peek(Token![>=]) {
                input.parse::<Token![>=]>()?;
                Ok(ComparisonError::GreaterOrEqual)
            } else if input.peek(Token![==]) {
                input.parse::<Token![==]>()?;
                Ok(ComparisonError::Equal)
            } else if input.peek(Token![<]) {
                input.parse::<Token![<]>()?;
                Ok(ComparisonError::Lesser)
            } else if input.peek(Token![>]) {
                input.parse::<Token![>]>()?;
                Ok(ComparisonError::Greater)
            } else {
                Err(syn::Error::new(
                    input.span(),
                    "no comparison operator could be identified. Valid operators are \"<\", \"<=\", \"==\", \">=\" or \">\".",
                ))
            }
        }

        // Read the arguments
        let first_arg: VariableOrLiteral = parse_arg(&input)?;
        let relation_first_to_second = parse_comparison_operator(&input)?;
        let second_arg: VariableOrLiteral = parse_arg(&input)?;

        // If the input continues, parse the third argument
        let (relation_second_to_third, third_arg) =
            if let Ok(operator) = parse_comparison_operator(&input) {
                (operator, Some(parse_arg(&input)?))
            } else {
                (ComparisonError::Equal, None)
            };

        return Ok(ComparisonErrorInfo {
            first_arg,
            relation_first_to_second,
            second_arg,
            relation_second_to_third,
            third_arg,
        });
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_check_bounds_info() {
        // Assert that the parse was successfull

        // Float
        let _: ComparisonErrorInfo = syn::parse_quote!(0.0 < arg);
        let _: ComparisonErrorInfo = syn::parse_quote!(0.0 <= arg);
        let _: ComparisonErrorInfo = syn::parse_quote!(0.0 <= arg as alternative_arg);
        let _: ComparisonErrorInfo = syn::parse_quote!(0.0 < arg <= 1.0);
        let _: ComparisonErrorInfo = syn::parse_quote!(0.0 < arg as alternative_arg <= 1.0);
        let _: ComparisonErrorInfo = syn::parse_quote!(arg < 1.0);
        let _: ComparisonErrorInfo = syn::parse_quote!(arg <= 1.0);
        let _: ComparisonErrorInfo = syn::parse_quote!(arg as alternative_arg <= 1.0);

        // Int
        let _: ComparisonErrorInfo = syn::parse_quote!(-1 < arg);
        let _: ComparisonErrorInfo = syn::parse_quote!(-1 < -2);
        let _: ComparisonErrorInfo = syn::parse_quote!(-1 < arg as alternative_arg <= 2);
    }
}
