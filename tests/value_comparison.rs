use compare_variables::compare_variables;

/**
Check the macro hygiene
 */
#[test]
fn test_compare_variables_float() {
    let arg = 0.5;
    let res = compare_variables!(0.0 < arg as alternative_arg <= 1.0);
    assert!(res.is_ok());

    let arg = 2.0;
    let res = compare_variables!(0.0 < arg as alternative_arg <= 1.0);
    assert!(res.is_err());

    let arg = 3.0;
    let res = compare_variables!(3.0 < arg);
    assert!(res.is_err());

    let arg = 2.0;
    let res = compare_variables!(-1.0 < arg);
    assert!(res.is_ok());

    let arg = 2.0;
    let res = compare_variables!(arg <= 2.0);
    assert!(res.is_ok());

    let arg = 2.0;
    let res = compare_variables!(arg < 2.0);
    assert!(res.is_err());
}

#[test]
fn test_compare_variables_i32() {
    let arg = 1;
    let res = compare_variables!(0 < arg as alternative_arg <= 2);
    assert!(res.is_ok());

    let res = compare_variables!(0 < arg as alternative_arg <= -2);
    assert!(res.is_err());

    let res = compare_variables!(0 >= -2);
    assert!(res.is_ok());
}

#[test]
fn test_compare_variables_usize() {
    let arg = 1usize;
    let res = compare_variables!(0 < arg as alternative_arg <= 2);
    assert!(res.is_ok());
}
