use compare_variables::compare_variables;
use uom::si::{f64::*, length::meter};

#[test]
fn test_compare_variables_float() {
    // Success
    {
        let arg = Length::new::<meter>(0.5);
        let zero = Length::new::<meter>(0.0);
        let one = Length::new::<meter>(1.0);
        let res = compare_variables!(zero < arg as alternative_arg <= one);
        assert!(res.is_ok());
    }

    // Failure
    {
        let arg = Length::new::<meter>(2.0);
        let zero = Length::new::<meter>(0.0);
        let one = Length::new::<meter>(1.0);
        let err = compare_variables!(zero < arg as alternative_arg <= one).unwrap_err();
        assert_eq!(
            format!("{}", err),
            "constraint `zero (value: 0.0 m^1) < alternative_arg (value: 2.0 m^1) <= one (value: 1.0 m^1)` is not fulfilled"
        );
    }
}
