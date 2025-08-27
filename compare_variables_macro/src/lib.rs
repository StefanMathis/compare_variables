#![cfg_attr(debug_assertions, allow(unused_imports))]

use proc_macro::{self, TokenStream};
use proc_macro_error::abort;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::ext::IdentExt;
use syn::parse::Parse;
use syn::{Token, parse_macro_input};

/**


This macro checks whether the defined bounds are respected by a given variable.

# Syntax description

`(lower_bound <(=)) argument_name (as alternative_name) (<(=) upper_bound)`

All arguments in parentheses are optional, see examples below. Either lower or upper bound must be given
The argument can be renamed inside the potentially constructed error via the optional `as alternative_name` syntax.

IMPORTANT NOTE: If a field should be accessed, it is necessary to bind it to a temporary variable and use the temporary variable, see examples below

# Examples
```
use compare_variables::compare_variables;

let arg = 2.0;
let res = compare_variables!(0.0 < arg as alternative_arg_name <= 1.0);
assert!(res.is_err());

// Ok since the upper bound is inclusive
let arg_1 = 0.0;
let arg_2 = 1.0;
let res = compare_variables!(arg_1 as alternative_arg_1 < arg_2 as alternative_arg_2 <= 1.0).is_ok();

// Fails since the lower bound is not inclusive
let arg = 0.0;
let res = compare_variables!(0.0 < arg as alternative_arg_name <= 1.0).is_err();

// Further ways to invoke the macro
let arg_1 = 0.0;
let arg_2 = 1.0;
compare_variables!(0.0 <= arg_1 as alternative_arg_name <= 1.0).is_ok();
compare_variables!(0.0 < arg_1).is_err();
compare_variables!(arg_1 < 0.0).is_err();
compare_variables!(arg_1 <= 0.0).is_ok();
compare_variables!(arg_2 <= arg_1).is_err();
compare_variables!(arg_2 as alternative_arg_name <= arg_1).is_err();

// Also works with negative numbers
let arg = -1.0;
compare_variables!(-2.0 <= arg as alternative_arg_name <= 0.0).is_ok();

// Field access
let tuple = (1, 2);

// This does not compile!
// compare_variables!(tuple.0 < 2);
let temp = tuple.0;
compare_variables!(temp < 2);

struct Test {
    field: f64
}
let test_struct = Test {field: 0.0};

// This does not compile!
// compare_variables!(test_struct.field < 2.0);
let temp = test_struct.field;
compare_variables!(temp < 2.0);
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
        compare_variables::ComparisonError::check(
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
enum Comparison {
    Lesser,
    LesserOrEqual,
    Equal,
    GreaterOrEqual,
    Greater,
}

impl Comparison {
    fn as_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Comparison::Lesser => {
                quote! {
                    compare_variables::ComparisonOperator::Lesser
                }
            }
            Comparison::LesserOrEqual => {
                quote! {
                    compare_variables::ComparisonOperator::LesserOrEqual
                }
            }
            Comparison::Equal => {
                quote! {
                    compare_variables::ComparisonOperator::Equal
                }
            }
            Comparison::GreaterOrEqual => {
                quote! {
                    compare_variables::ComparisonOperator::GreaterOrEqual
                }
            }
            Comparison::Greater => {
                quote! {
                    compare_variables::ComparisonOperator::Greater
                }
            }
        }
    }
}

enum VariableOrLiteral {
    InfluencedByConditions {
        arg_names: Vec<syn::Ident>,
        arg_names_display: Vec<syn::Ident>,
    },
    LitFloat(syn::LitFloat),
    LitInt(syn::LitInt),
}

impl VariableOrLiteral {
    fn as_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            VariableOrLiteral::InfluencedByConditions {
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
                let arg_name_display = arg_names_display
                    .into_iter()
                    .map(|ident| ident.to_string())
                    .collect::<Vec<String>>()
                    .join(".");
                quote! {
                    compare_variables::ComparisonValue::new(#arg_value_ts, Some(#arg_name_display))
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
    relation_first_to_second: Comparison,
    second_arg: VariableOrLiteral,
    relation_second_to_third: Comparison,
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

                let mut arg_names_display: Vec<Ident> = Vec::new();
                let arg_names_display = if input.peek(Token![as]) {
                    input.parse::<Token![as]>()?;
                    loop {
                        let arg_name: Ident = input.call(Ident::parse_any)?; // parse_any also handles stuff like self
                        arg_names_display.push(arg_name);

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
                    arg_names.clone()
                };
                return Ok(VariableOrLiteral::InfluencedByConditions {
                    arg_names,
                    arg_names_display,
                });
            }
        }

        fn parse_comparison_operator(input: &syn::parse::ParseStream) -> syn::Result<Comparison> {
            // If Token![<] is tested before Token![<=], then "<" is parsed, leaving only "=". This will then lead to a compile error.
            if input.peek(Token![<=]) {
                input.parse::<Token![<=]>()?;
                Ok(Comparison::LesserOrEqual)
            } else if input.peek(Token![>=]) {
                input.parse::<Token![>=]>()?;
                Ok(Comparison::GreaterOrEqual)
            } else if input.peek(Token![==]) {
                input.parse::<Token![==]>()?;
                Ok(Comparison::Equal)
            } else if input.peek(Token![<]) {
                input.parse::<Token![<]>()?;
                Ok(Comparison::Lesser)
            } else if input.peek(Token![>]) {
                input.parse::<Token![>]>()?;
                Ok(Comparison::Greater)
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
                (Comparison::Equal, None)
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
