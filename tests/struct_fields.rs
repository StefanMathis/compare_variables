use compare_variables::compare_variables;

struct NamedFields {
    field1: f64,
    field2: f64,
}

struct AnonymousField(f64);

impl NamedFields {
    fn field1_larger_than_0(&self) -> bool {
        return compare_variables!(self.field1 > 0.0).is_ok();
    }

    fn field1_larger_than_field2(&self) -> bool {
        return compare_variables!(self.field1 as field1 > self.field2).is_ok();
    }

    fn value_bigger_than_field1(
        &self,
        value: f64,
    ) -> Result<(), compare_variables::ComparisonError<f64>> {
        return compare_variables!(self.field1 < value);
    }

    fn value_bigger_than_field2(
        &self,
        value: f64,
    ) -> Result<(), compare_variables::ComparisonError<f64>> {
        return compare_variables!(self.field2 as field2 < value);
    }
}

#[test]
fn test_named_struct_fields() {
    let instance = NamedFields {
        field1: 1.0,
        field2: 2.0,
    };

    assert!(compare_variables!(0.0 < instance.field1 <= 1.0).is_ok());
    assert!(instance.field1_larger_than_0());
    assert!(!instance.field1_larger_than_field2());

    // Check the error messages
    let error_msg = compare_variables!(0.0 < instance.field1 as this_struct_instance.field1 < 1.0)
        .unwrap_err()
        .to_string();
    assert!(error_msg.contains("`0.0 < this_struct_instance.field1 (value: 1.0) < 1.0` is false"));

    let error_msg = compare_variables!(0.0 > instance.field2)
        .unwrap_err()
        .to_string();
    assert!(error_msg.contains("`0.0 > instance.field2 (value: 2.0)` is false"));

    let error_msg = instance
        .value_bigger_than_field1(1.0)
        .unwrap_err()
        .to_string();
    assert!(error_msg.contains("`self.field1 (value: 1.0) < value (value: 1.0)` is false"));
    let error_msg = instance
        .value_bigger_than_field2(1.0)
        .unwrap_err()
        .to_string();
    assert!(error_msg.contains("`field2 (value: 2.0) < value (value: 1.0)` is false"));
}

#[test]
fn test_anonymous_struct_fields() {
    let anon = AnonymousField(1.0);
    assert!(compare_variables!(0.0 < anon.0 <= 1.0).is_ok());
    let err = compare_variables!(0.0 > anon.0).unwrap_err();
    assert_eq!(err.to_string(), "`0.0 > anon.0 (value: 1.0)` is false");
}
