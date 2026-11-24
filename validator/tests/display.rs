#[cfg(feature = "derive")]
mod tests {
    use validator::Validate;

    #[derive(Validate, Clone)]
    struct Foo {
        #[validate(length(equal = 5, message = "Please provide a valid foo!"))]
        foo: String,
    }

    #[test]
    fn test_message() {
        let bad_foo = Foo { foo: "hi!".into() };
        let err = format!("{}", bad_foo.validate().unwrap_err());
        assert_eq!(err, "foo: Please provide a valid foo!");
    }

    #[derive(Validate)]
    struct Bar {
        #[validate(nested)]
        bar: Foo,
    }

    #[derive(Validate)]
    struct DeepBar {
        #[validate(nested)]
        deep_bar: Bar,
    }

    #[test]
    fn test_nested_struct() {
        let bad_foo = Foo { foo: "hi!".into() };
        let bad_bar = Bar { bar: bad_foo };
        let err = format!("{}", bad_bar.validate().unwrap_err());
        assert_eq!(err, "bar.foo: Please provide a valid foo!");

        let bad_deep_bar = DeepBar { deep_bar: bad_bar };
        let err = format!("{}", bad_deep_bar.validate().unwrap_err());
        assert_eq!(err, "deep_bar.bar.foo: Please provide a valid foo!");
    }

    #[derive(Validate)]
    struct Baz {
        #[validate(nested)]
        baz: Vec<Foo>,
    }

    #[test]
    fn test_nested_vec() {
        let bad_foo = Foo { foo: "hi!".into() };
        let bad_baz = Baz { baz: vec![bad_foo] };
        let err = format!("{}", bad_baz.validate().unwrap_err());
        assert_eq!(err, "baz[0].foo: Please provide a valid foo!");
    }

    #[derive(Validate)]
    struct Pagination {
        #[validate(range(min = 1, message = "Page must be greater than 0."))]
        page: u64,
        #[validate(range(min = 1, max = 100, message = "The page size must be between 1 and 100."))]
        size: u64,
    }

    #[derive(Validate)]
    struct Query {
        #[validate(nested)]
        pagination: Pagination,
    }

    #[test]
    fn test_nested_multiple_field_errors_are_delimited() {
        let q = Query {
            pagination: Pagination { page: 0, size: 0 },
        };
        let err = format!("{}", q.validate().unwrap_err());
        assert!(
            err.contains('\n'),
            "multiple nested field errors should be separated by a newline, got: {err:?}"
        );
        assert!(err.contains("pagination.page:"));
        assert!(err.contains("pagination.size:"));
    }
}
