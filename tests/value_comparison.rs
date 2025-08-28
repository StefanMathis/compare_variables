use compare_variables::*;

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

#[test]
fn test_compare_variables_raw() {
    {
        let arg = 1usize;
        let err = compare_variables!(0 > raw arg);
        assert_eq!(format!("{}", err.unwrap_err()), "`0 > 1` is false");
    }
    {
        let arg = 1usize;
        let err = compare_variables!(0 > raw arg > 2);
        assert_eq!(format!("{}", err.unwrap_err()), "`0 > 1 > 2` is false");
    }
    {
        let arg = 1usize;
        let err = compare_variables!(0 > raw arg as alternative_arg);
        assert_eq!(format!("{}", err.unwrap_err()), "`0 > 1` is false");
    }
}

#[test]
fn check_arg_message() {
    let message = ComparisonError::new(
        ComparisonValue::new(1.0, None),
        ComparisonOperator::Equal,
        ComparisonValue::new(2.0, Some("argument")),
        ComparisonOperator::Equal,
        None,
    )
    .unwrap_err();
    assert_eq!(
        format!("{message}"),
        "`1.0 == argument (value: 2.0)` is false"
    );

    let message = ComparisonError::new(
        ComparisonValue::new(2.0, None),
        ComparisonOperator::Equal,
        ComparisonValue::new(2.0, Some("argument")),
        ComparisonOperator::Greater,
        Some(ComparisonValue::new(2.0, Some("argument"))),
    )
    .unwrap_err();
    assert_eq!(
        format!("{message}"),
        "`2.0 == argument (value: 2.0) > argument (value: 2.0)` is false"
    );

    let message = ComparisonError::new(
        ComparisonValue::new(2.0, None),
        ComparisonOperator::Equal,
        ComparisonValue::new(2.0, Some("argument")),
        ComparisonOperator::Greater,
        Some(ComparisonValue::new(2.0, None)),
    )
    .unwrap_err();
    assert_eq!(
        format!("{message}"),
        "`2.0 == argument (value: 2.0) > 2.0` is false"
    );

    let message = ComparisonError::new(
        ComparisonValue::new(2.0, None),
        ComparisonOperator::Lesser,
        ComparisonValue::new(2.0, None),
        ComparisonOperator::Greater,
        Some(ComparisonValue::new(2.0, None)),
    )
    .unwrap_err();
    assert_eq!(format!("{message}"), "`2.0 < 2.0 > 2.0` is false");

    let message = ComparisonError::new(
        ComparisonValue::new(0.0, None),
        ComparisonOperator::Lesser,
        ComparisonValue::new(2.0, None),
        ComparisonOperator::LesserOrEqual,
        Some(ComparisonValue::new(1.0, None)),
    )
    .unwrap_err();
    assert_eq!(format!("{message}"), "`0.0 < 2.0 <= 1.0` is false");

    let _ = ComparisonError::new(
        ComparisonValue::new(2.0, None),
        ComparisonOperator::LesserOrEqual,
        ComparisonValue::new(2.0, None),
        ComparisonOperator::GreaterOrEqual,
        Some(ComparisonValue::new(2.0, None)),
    )
    .is_ok();
}
