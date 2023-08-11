#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use serde::Serialize;
use std::{borrow::Cow, collections::{HashMap, HashSet}};
use validator::{
    validate_length, Validate, ValidationError, ValidationErrors, ValidationErrorsKind,
};
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "is_fine_with_nested_validations"]
pub const is_fine_with_nested_validations: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("is_fine_with_nested_validations"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "validator_derive_tests/tests/nested.rs",
        start_line: 11usize,
        start_col: 4usize,
        end_line: 11usize,
        end_col: 35usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        is_fine_with_nested_validations(),
    )),
};
fn is_fine_with_nested_validations() {
    struct Root<'a> {
        #[validate(length(min = 5, max = 10))]
        value: String,
        #[validate(nested)]
        a: &'a A,
    }
    impl<'a> ::validator::Validate for Root<'a> {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            use validator::ValidateNested;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.value.validate_length(Some(5), Some(10), None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &5);
                err.add_param(::std::borrow::Cow::from("max"), &10);
                err.add_param(::std::borrow::Cow::from("value"), &self.value);
                errors.add("value", err);
            }
            errors.add_non_nested("a", self.a.validate_nested("a"));
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    struct A {
        #[validate(length(min = 5, max = 10))]
        value: String,
        #[validate(nested)]
        b: B,
    }
    impl ::validator::Validate for A {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            use validator::ValidateNested;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.value.validate_length(Some(5), Some(10), None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &5);
                err.add_param(::std::borrow::Cow::from("max"), &10);
                err.add_param(::std::borrow::Cow::from("value"), &self.value);
                errors.add("value", err);
            }
            errors.add_non_nested("b", self.b.validate_nested("b"));
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    struct B {
        #[validate(length(min = 5, max = 10))]
        value: String,
    }
    impl ::validator::Validate for B {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.value.validate_length(Some(5), Some(10), None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &5);
                err.add_param(::std::borrow::Cow::from("max"), &10);
                err.add_param(::std::borrow::Cow::from("value"), &self.value);
                errors.add("value", err);
            }
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    let root = Root {
        value: "valid".to_string(),
        a: &A {
            value: "valid".to_string(),
            b: B { value: "valid".to_string() },
        },
    };
    if !root.validate().is_ok() {
        ::core::panicking::panic("assertion failed: root.validate().is_ok()")
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "fails_nested_validation"]
pub const fails_nested_validation: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("fails_nested_validation"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "validator_derive_tests/tests/nested.rs",
        start_line: 43usize,
        start_col: 4usize,
        end_line: 43usize,
        end_col: 27usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(fails_nested_validation())),
};
fn fails_nested_validation() {
    struct Root<'a> {
        #[validate(length(min = 5, max = 10))]
        value: String,
        #[validate(nested)]
        a: &'a A,
    }
    impl<'a> ::validator::Validate for Root<'a> {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            use validator::ValidateNested;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.value.validate_length(Some(5), Some(10), None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &5);
                err.add_param(::std::borrow::Cow::from("max"), &10);
                err.add_param(::std::borrow::Cow::from("value"), &self.value);
                errors.add("value", err);
            }
            errors.add_non_nested("a", self.a.validate_nested("a"));
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    struct A {
        #[validate(length(min = 5, max = 10))]
        value: String,
        #[validate(nested)]
        b: B,
    }
    impl ::validator::Validate for A {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            use validator::ValidateNested;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.value.validate_length(Some(5), Some(10), None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &5);
                err.add_param(::std::borrow::Cow::from("max"), &10);
                err.add_param(::std::borrow::Cow::from("value"), &self.value);
                errors.add("value", err);
            }
            errors.add_non_nested("b", self.b.validate_nested("b"));
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    struct B {
        #[validate(length(min = 5, max = 10))]
        value: String,
    }
    impl ::validator::Validate for B {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.value.validate_length(Some(5), Some(10), None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &5);
                err.add_param(::std::borrow::Cow::from("max"), &10);
                err.add_param(::std::borrow::Cow::from("value"), &self.value);
                errors.add("value", err);
            }
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    let root = Root {
        value: "valid".to_string(),
        a: &A {
            value: "invalid value".to_string(),
            b: B { value: "valid".to_string() },
        },
    };
    if !root.validate().is_err() {
        ::core::panicking::panic("assertion failed: root.validate().is_err()")
    }
    let root = Root {
        value: "valid".to_string(),
        a: &A {
            value: "valid".to_string(),
            b: B {
                value: "invalid value".to_string(),
            },
        },
    };
    if !root.validate().is_err() {
        ::core::panicking::panic("assertion failed: root.validate().is_err()")
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_can_validate_option_fields_without_lifetime"]
pub const test_can_validate_option_fields_without_lifetime: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_can_validate_option_fields_without_lifetime"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "validator_derive_tests/tests/nested.rs",
        start_line: 133usize,
        start_col: 4usize,
        end_line: 133usize,
        end_col: 52usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        test_can_validate_option_fields_without_lifetime(),
    )),
};
fn test_can_validate_option_fields_without_lifetime() {
    struct ParentWithOptionalChild {
        #[validate(nested)]
        child: Option<Child>,
    }
    impl ::validator::Validate for ParentWithOptionalChild {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateNested;
            let mut errors = ::validator::ValidationErrors::new();
            errors.add_non_nested("child", self.child.validate_nested("child"));
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    struct Child {
        #[validate(length(min = 1))]
        value: String,
    }
    impl ::validator::Validate for Child {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.value.validate_length(Some(1), None, None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &1);
                err.add_param(::std::borrow::Cow::from("value"), &self.value);
                errors.add("value", err);
            }
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    let instance = ParentWithOptionalChild {
        child: Some(Child { value: String::new() }),
    };
    let res = instance.validate();
    if !res.is_err() {
        ::core::panicking::panic("assertion failed: res.is_err()")
    }
    let err = res.unwrap_err();
    let errs = err.errors();
    match (&errs.len(), &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    if !errs.contains_key("child") {
        ::core::panicking::panic("assertion failed: errs.contains_key(\\\"child\\\")")
    }
    if let ValidationErrorsKind::Struct(ref errs) = errs["child"] {
        unwrap_map(
            errs,
            |errs| {
                match (&errs.len(), &1) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                if !errs.contains_key("value") {
                    ::core::panicking::panic(
                        "assertion failed: errs.contains_key(\\\"value\\\")",
                    )
                }
                if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                    match (&errs.len(), &1) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    match (&errs[0].code, &"length") {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                } else {
                    {
                        ::std::rt::begin_panic("Expected field validation errors");
                    };
                }
            },
        );
    } else {
        {
            ::std::rt::begin_panic("Expected struct validation errors");
        };
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_can_validate_option_fields_with_lifetime"]
pub const test_can_validate_option_fields_with_lifetime: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_can_validate_option_fields_with_lifetime"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "validator_derive_tests/tests/nested.rs",
        start_line: 172usize,
        start_col: 4usize,
        end_line: 172usize,
        end_col: 49usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        test_can_validate_option_fields_with_lifetime(),
    )),
};
fn test_can_validate_option_fields_with_lifetime() {
    struct ParentWithLifetimeAndOptionalChild<'a> {
        #[validate(nested)]
        child: Option<&'a Child>,
    }
    impl<'a> ::validator::Validate for ParentWithLifetimeAndOptionalChild<'a> {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateNested;
            let mut errors = ::validator::ValidationErrors::new();
            errors.add_non_nested("child", self.child.validate_nested("child"));
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    struct Child {
        #[validate(length(min = 1))]
        value: String,
    }
    impl ::validator::Validate for Child {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.value.validate_length(Some(1), None, None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &1);
                err.add_param(::std::borrow::Cow::from("value"), &self.value);
                errors.add("value", err);
            }
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    let child = Child { value: String::new() };
    let instance = ParentWithLifetimeAndOptionalChild {
        child: Some(&child),
    };
    let res = instance.validate();
    if !res.is_err() {
        ::core::panicking::panic("assertion failed: res.is_err()")
    }
    let err = res.unwrap_err();
    let errs = err.errors();
    match (&errs.len(), &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    if !errs.contains_key("child") {
        ::core::panicking::panic("assertion failed: errs.contains_key(\\\"child\\\")")
    }
    if let ValidationErrorsKind::Struct(ref errs) = errs["child"] {
        unwrap_map(
            errs,
            |errs| {
                match (&errs.len(), &1) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                if !errs.contains_key("value") {
                    ::core::panicking::panic(
                        "assertion failed: errs.contains_key(\\\"value\\\")",
                    )
                }
                if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                    match (&errs.len(), &1) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    match (&errs[0].code, &"length") {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                } else {
                    {
                        ::std::rt::begin_panic("Expected field validation errors");
                    };
                }
            },
        );
    } else {
        {
            ::std::rt::begin_panic("Expected struct validation errors");
        };
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_works_with_none_values"]
pub const test_works_with_none_values: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_works_with_none_values"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "validator_derive_tests/tests/nested.rs",
        start_line: 212usize,
        start_col: 4usize,
        end_line: 212usize,
        end_col: 31usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        test_works_with_none_values(),
    )),
};
fn test_works_with_none_values() {
    struct ParentWithOptionalChild {
        #[validate(nested)]
        child: Option<Child>,
    }
    impl ::validator::Validate for ParentWithOptionalChild {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateNested;
            let mut errors = ::validator::ValidationErrors::new();
            errors.add_non_nested("child", self.child.validate_nested("child"));
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    struct Child {
        #[validate(length(min = 1))]
        value: String,
    }
    impl ::validator::Validate for Child {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.value.validate_length(Some(1), None, None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &1);
                err.add_param(::std::borrow::Cow::from("value"), &self.value);
                errors.add("value", err);
            }
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    let instance = ParentWithOptionalChild {
        child: None,
    };
    let res = instance.validate();
    if !res.is_ok() {
        ::core::panicking::panic("assertion failed: res.is_ok()")
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_can_validate_vector_fields"]
pub const test_can_validate_vector_fields: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_can_validate_vector_fields"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "validator_derive_tests/tests/nested.rs",
        start_line: 232usize,
        start_col: 4usize,
        end_line: 232usize,
        end_col: 35usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        test_can_validate_vector_fields(),
    )),
};
fn test_can_validate_vector_fields() {
    struct ParentWithVectorOfChildren {
        #[validate(length(min = 1))]
        #[validate(nested)]
        child: Vec<Child>,
    }
    impl ::validator::Validate for ParentWithVectorOfChildren {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            use validator::ValidateNested;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.child.validate_length(Some(1), None, None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &1);
                err.add_param(::std::borrow::Cow::from("value"), &self.child);
                errors.add("child", err);
            }
            errors.add_non_nested("child", self.child.validate_nested("child"));
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    struct Child {
        #[validate(length(min = 1))]
        value: String,
    }
    impl ::validator::Validate for Child {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.value.validate_length(Some(1), None, None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &1);
                err.add_param(::std::borrow::Cow::from("value"), &self.value);
                errors.add("value", err);
            }
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Child {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Child",
                    false as usize + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "value",
                    &self.value,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    let instance = ParentWithVectorOfChildren {
        child: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                Child {
                    value: "valid".to_string(),
                },
                Child { value: String::new() },
                Child {
                    value: "valid".to_string(),
                },
                Child { value: String::new() },
            ]),
        ),
    };
    let res = instance.validate();
    if !res.is_err() {
        ::core::panicking::panic("assertion failed: res.is_err()")
    }
    let err = res.unwrap_err();
    let errs = err.errors();
    match (&errs.len(), &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    if !errs.contains_key("child") {
        ::core::panicking::panic("assertion failed: errs.contains_key(\\\"child\\\")")
    }
    if let ValidationErrorsKind::List(ref errs) = errs["child"] {
        if !errs.contains_key(&1) {
            ::core::panicking::panic("assertion failed: errs.contains_key(&1)")
        }
        unwrap_map(
            &errs[&1],
            |errs| {
                match (&errs.len(), &1) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                if !errs.contains_key("value") {
                    ::core::panicking::panic(
                        "assertion failed: errs.contains_key(\\\"value\\\")",
                    )
                }
                if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                    match (&errs.len(), &1) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    match (&errs[0].code, &"length") {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                } else {
                    {
                        ::std::rt::begin_panic("Expected field validation errors");
                    };
                }
            },
        );
        if !errs.contains_key(&3) {
            ::core::panicking::panic("assertion failed: errs.contains_key(&3)")
        }
        unwrap_map(
            &errs[&3],
            |errs| {
                match (&errs.len(), &1) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                if !errs.contains_key("value") {
                    ::core::panicking::panic(
                        "assertion failed: errs.contains_key(\\\"value\\\")",
                    )
                }
                if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                    match (&errs.len(), &1) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    match (&errs[0].code, &"length") {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                } else {
                    {
                        ::std::rt::begin_panic("Expected field validation errors");
                    };
                }
            },
        );
    } else {
        {
            ::std::rt::begin_panic("Expected list validation errors");
        };
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_can_validate_slice_fields"]
pub const test_can_validate_slice_fields: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_can_validate_slice_fields"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "validator_derive_tests/tests/nested.rs",
        start_line: 291usize,
        start_col: 4usize,
        end_line: 291usize,
        end_col: 34usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        test_can_validate_slice_fields(),
    )),
};
fn test_can_validate_slice_fields() {
    struct ParentWithSliceOfChildren<'a> {
        #[validate(length(min = 1))]
        #[validate(nested)]
        child: &'a [Child],
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for ParentWithSliceOfChildren<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ParentWithSliceOfChildren",
                "child",
                &&self.child,
            )
        }
    }
    impl<'a> ::validator::Validate for ParentWithSliceOfChildren<'a> {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            use validator::ValidateNested;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.child.validate_length(Some(1), None, None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &1);
                err.add_param(::std::borrow::Cow::from("value"), &self.child);
                errors.add("child", err);
            }
            errors.add_non_nested("child", self.child.validate_nested("child"));
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    struct Child {
        #[validate(length(min = 1))]
        value: String,
    }
    impl ::validator::Validate for Child {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.value.validate_length(Some(1), None, None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &1);
                err.add_param(::std::borrow::Cow::from("value"), &self.value);
                errors.add("value", err);
            }
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    let child = <[_]>::into_vec(
        #[rustc_box]
        ::alloc::boxed::Box::new([
            Child {
                value: "valid".to_string(),
            },
            Child { value: String::new() },
            Child {
                value: "valid".to_string(),
            },
            Child { value: String::new() },
        ]),
    );
    let instance = ParentWithSliceOfChildren {
        child: &child,
    };
    let res = instance.validate();
    if !res.is_err() {
        ::core::panicking::panic("assertion failed: res.is_err()")
    }
    let err = res.unwrap_err();
    let errs = err.errors();
    match (&errs.len(), &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    if !errs.contains_key("child") {
        ::core::panicking::panic("assertion failed: errs.contains_key(\\\"child\\\")")
    }
    if let ValidationErrorsKind::List(ref errs) = errs["child"] {
        if !errs.contains_key(&1) {
            ::core::panicking::panic("assertion failed: errs.contains_key(&1)")
        }
        unwrap_map(
            &errs[&1],
            |errs| {
                match (&errs.len(), &1) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                if !errs.contains_key("value") {
                    ::core::panicking::panic(
                        "assertion failed: errs.contains_key(\\\"value\\\")",
                    )
                }
                if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                    match (&errs.len(), &1) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    match (&errs[0].code, &"length") {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                } else {
                    {
                        ::std::rt::begin_panic("Expected field validation errors");
                    };
                }
            },
        );
        if !errs.contains_key(&3) {
            ::core::panicking::panic("assertion failed: errs.contains_key(&3)")
        }
        unwrap_map(
            &errs[&3],
            |errs| {
                match (&errs.len(), &1) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                if !errs.contains_key("value") {
                    ::core::panicking::panic(
                        "assertion failed: errs.contains_key(\\\"value\\\")",
                    )
                }
                if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                    match (&errs.len(), &1) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    match (&errs[0].code, &"length") {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                } else {
                    {
                        ::std::rt::begin_panic("Expected field validation errors");
                    };
                }
            },
        );
    } else {
        {
            ::std::rt::begin_panic("Expected list validation errors");
        };
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_can_validate_array_fields"]
pub const test_can_validate_array_fields: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_can_validate_array_fields"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "validator_derive_tests/tests/nested.rs",
        start_line: 348usize,
        start_col: 4usize,
        end_line: 348usize,
        end_col: 34usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        test_can_validate_array_fields(),
    )),
};
fn test_can_validate_array_fields() {
    struct ParentWithArrayOfChildren {
        #[validate(length(min = 1))]
        #[validate(nested)]
        child: [Child; 4],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ParentWithArrayOfChildren {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ParentWithArrayOfChildren",
                "child",
                &&self.child,
            )
        }
    }
    impl ::validator::Validate for ParentWithArrayOfChildren {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            use validator::ValidateNested;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.child.validate_length(Some(1), None, None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &1);
                err.add_param(::std::borrow::Cow::from("value"), &self.child);
                errors.add("child", err);
            }
            errors.add_non_nested("child", self.child.validate_nested("child"));
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    let instance = ParentWithArrayOfChildren {
        child: [
            Child {
                value: "valid".to_string(),
            },
            Child { value: String::new() },
            Child {
                value: "valid".to_string(),
            },
            Child { value: String::new() },
        ],
    };
    let res = instance.validate();
    if !res.is_err() {
        ::core::panicking::panic("assertion failed: res.is_err()")
    }
    let err = res.unwrap_err();
    let errs = err.errors();
    match (&errs.len(), &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    if !errs.contains_key("child") {
        ::core::panicking::panic("assertion failed: errs.contains_key(\\\"child\\\")")
    }
    if let ValidationErrorsKind::List(ref errs) = errs["child"] {
        if !errs.contains_key(&1) {
            ::core::panicking::panic("assertion failed: errs.contains_key(&1)")
        }
        unwrap_map(
            &errs[&1],
            |errs| {
                match (&errs.len(), &1) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                if !errs.contains_key("value") {
                    ::core::panicking::panic(
                        "assertion failed: errs.contains_key(\\\"value\\\")",
                    )
                }
                if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                    match (&errs.len(), &1) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    match (&errs[0].code, &"length") {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                } else {
                    {
                        ::std::rt::begin_panic("Expected field validation errors");
                    };
                }
            },
        );
        if !errs.contains_key(&3) {
            ::core::panicking::panic("assertion failed: errs.contains_key(&3)")
        }
        unwrap_map(
            &errs[&3],
            |errs| {
                match (&errs.len(), &1) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                if !errs.contains_key("value") {
                    ::core::panicking::panic(
                        "assertion failed: errs.contains_key(\\\"value\\\")",
                    )
                }
                if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                    match (&errs.len(), &1) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    match (&errs[0].code, &"length") {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                } else {
                    {
                        ::std::rt::begin_panic("Expected field validation errors");
                    };
                }
            },
        );
    } else {
        {
            ::std::rt::begin_panic("Expected list validation errors");
        };
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_can_validate_option_vector_fields"]
pub const test_can_validate_option_vector_fields: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_can_validate_option_vector_fields"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "validator_derive_tests/tests/nested.rs",
        start_line: 400usize,
        start_col: 4usize,
        end_line: 400usize,
        end_col: 42usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        test_can_validate_option_vector_fields(),
    )),
};
fn test_can_validate_option_vector_fields() {
    struct ParentWithOptionVectorOfChildren {
        #[validate(length(min = 1))]
        #[validate(nested)]
        child: Option<Vec<Child>>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ParentWithOptionVectorOfChildren {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ParentWithOptionVectorOfChildren",
                "child",
                &&self.child,
            )
        }
    }
    impl ::validator::Validate for ParentWithOptionVectorOfChildren {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            use validator::ValidateNested;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.child.validate_length(Some(1), None, None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &1);
                err.add_param(::std::borrow::Cow::from("value"), &self.child);
                errors.add("child", err);
            }
            errors.add_non_nested("child", self.child.validate_nested("child"));
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    let instance = ParentWithOptionVectorOfChildren {
        child: Some(
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    Child {
                        value: "valid".to_string(),
                    },
                    Child { value: String::new() },
                    Child {
                        value: "valid".to_string(),
                    },
                    Child { value: String::new() },
                ]),
            ),
        ),
    };
    let res = instance.validate();
    if !res.is_err() {
        ::core::panicking::panic("assertion failed: res.is_err()")
    }
    let err = res.unwrap_err();
    let errs = err.errors();
    match (&errs.len(), &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    if !errs.contains_key("child") {
        ::core::panicking::panic("assertion failed: errs.contains_key(\\\"child\\\")")
    }
    if let ValidationErrorsKind::List(ref errs) = errs["child"] {
        if !errs.contains_key(&1) {
            ::core::panicking::panic("assertion failed: errs.contains_key(&1)")
        }
        unwrap_map(
            &errs[&1],
            |errs| {
                match (&errs.len(), &1) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                if !errs.contains_key("value") {
                    ::core::panicking::panic(
                        "assertion failed: errs.contains_key(\\\"value\\\")",
                    )
                }
                if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                    match (&errs.len(), &1) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    match (&errs[0].code, &"length") {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                } else {
                    {
                        ::std::rt::begin_panic("Expected field validation errors");
                    };
                }
            },
        );
        if !errs.contains_key(&3) {
            ::core::panicking::panic("assertion failed: errs.contains_key(&3)")
        }
        unwrap_map(
            &errs[&3],
            |errs| {
                match (&errs.len(), &1) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                if !errs.contains_key("value") {
                    ::core::panicking::panic(
                        "assertion failed: errs.contains_key(\\\"value\\\")",
                    )
                }
                if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                    match (&errs.len(), &1) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    match (&errs[0].code, &"length") {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                } else {
                    {
                        ::std::rt::begin_panic("Expected field validation errors");
                    };
                }
            },
        );
    } else {
        {
            ::std::rt::begin_panic("Expected list validation errors");
        };
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_can_validate_map_fields"]
pub const test_can_validate_map_fields: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_can_validate_map_fields"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "validator_derive_tests/tests/nested.rs",
        start_line: 452usize,
        start_col: 4usize,
        end_line: 452usize,
        end_col: 32usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        test_can_validate_map_fields(),
    )),
};
fn test_can_validate_map_fields() {
    struct ParentWithMapOfChildren {
        #[validate(length(min = 1))]
        #[validate(nested)]
        child: HashMap<i8, Child>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ParentWithMapOfChildren {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ParentWithMapOfChildren",
                "child",
                &&self.child,
            )
        }
    }
    impl ::validator::Validate for ParentWithMapOfChildren {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            use validator::ValidateNested;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.child.validate_length(Some(1), None, None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &1);
                err.add_param(::std::borrow::Cow::from("value"), &self.child);
                errors.add("child", err);
            }
            errors.add_non_nested("child", self.child.validate_nested("child"));
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    let instance = ParentWithMapOfChildren {
        child: [(0, Child { value: String::new() })].iter().cloned().collect(),
    };
    let res = instance.validate();
    if !res.is_err() {
        ::core::panicking::panic("assertion failed: res.is_err()")
    }
    let err = res.unwrap_err();
    let errs = err.errors();
    match (&errs.len(), &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    if !errs.contains_key("child") {
        ::core::panicking::panic("assertion failed: errs.contains_key(\\\"child\\\")")
    }
    if let ValidationErrorsKind::List(ref errs) = errs["child"] {
        if !errs.contains_key(&0) {
            ::core::panicking::panic("assertion failed: errs.contains_key(&0)")
        }
        unwrap_map(
            &errs[&0],
            |errs| {
                match (&errs.len(), &1) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                if !errs.contains_key("value") {
                    ::core::panicking::panic(
                        "assertion failed: errs.contains_key(\\\"value\\\")",
                    )
                }
                if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                    match (&errs.len(), &1) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    match (&errs[0].code, &"length") {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                } else {
                    {
                        ::std::rt::begin_panic("Expected field validation errors");
                    };
                }
            },
        );
    } else {
        {
            ::std::rt::begin_panic("Expected list validation errors");
        };
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_can_validate_ref_map_fields"]
pub const test_can_validate_ref_map_fields: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_can_validate_ref_map_fields"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "validator_derive_tests/tests/nested.rs",
        start_line: 488usize,
        start_col: 4usize,
        end_line: 488usize,
        end_col: 36usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        test_can_validate_ref_map_fields(),
    )),
};
fn test_can_validate_ref_map_fields() {
    struct ParentWithRefMapOfChildren<'a> {
        #[validate(length(min = 1))]
        #[validate(nested)]
        child: &'a HashMap<i8, Child>,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for ParentWithRefMapOfChildren<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ParentWithRefMapOfChildren",
                "child",
                &&self.child,
            )
        }
    }
    impl<'a> ::validator::Validate for ParentWithRefMapOfChildren<'a> {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            use validator::ValidateNested;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.child.validate_length(Some(1), None, None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &1);
                err.add_param(::std::borrow::Cow::from("value"), &self.child);
                errors.add("child", err);
            }
            errors.add_non_nested("child", self.child.validate_nested("child"));
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    let child = [(0, Child { value: String::new() })].iter().cloned().collect();
    let instance = ParentWithRefMapOfChildren {
        child: &child,
    };
    let res = instance.validate();
    if !res.is_err() {
        ::core::panicking::panic("assertion failed: res.is_err()")
    }
    let err = res.unwrap_err();
    let errs = err.errors();
    match (&errs.len(), &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    if !errs.contains_key("child") {
        ::core::panicking::panic("assertion failed: errs.contains_key(\\\"child\\\")")
    }
    if let ValidationErrorsKind::List(ref errs) = errs["child"] {
        if !errs.contains_key(&0) {
            ::core::panicking::panic("assertion failed: errs.contains_key(&0)")
        }
        unwrap_map(
            &errs[&0],
            |errs| {
                match (&errs.len(), &1) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                if !errs.contains_key("value") {
                    ::core::panicking::panic(
                        "assertion failed: errs.contains_key(\\\"value\\\")",
                    )
                }
                if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                    match (&errs.len(), &1) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    match (&errs[0].code, &"length") {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                } else {
                    {
                        ::std::rt::begin_panic("Expected field validation errors");
                    };
                }
            },
        );
    } else {
        {
            ::std::rt::begin_panic("Expected list validation errors");
        };
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_can_validate_option_map_fields"]
pub const test_can_validate_option_map_fields: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_can_validate_option_map_fields"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "validator_derive_tests/tests/nested.rs",
        start_line: 523usize,
        start_col: 4usize,
        end_line: 523usize,
        end_col: 39usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        test_can_validate_option_map_fields(),
    )),
};
fn test_can_validate_option_map_fields() {
    struct ParentWithOptionMapOfChildren {
        #[validate(length(min = 1))]
        #[validate(nested)]
        child: Option<HashMap<i8, Child>>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ParentWithOptionMapOfChildren {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ParentWithOptionMapOfChildren",
                "child",
                &&self.child,
            )
        }
    }
    impl ::validator::Validate for ParentWithOptionMapOfChildren {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            use validator::ValidateNested;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.child.validate_length(Some(1), None, None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &1);
                err.add_param(::std::borrow::Cow::from("value"), &self.child);
                errors.add("child", err);
            }
            errors.add_non_nested("child", self.child.validate_nested("child"));
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    let instance = ParentWithOptionMapOfChildren {
        child: Some([(0, Child { value: String::new() })].iter().cloned().collect()),
    };
    let res = instance.validate();
    if !res.is_err() {
        ::core::panicking::panic("assertion failed: res.is_err()")
    }
    let err = res.unwrap_err();
    let errs = err.errors();
    match (&errs.len(), &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    if !errs.contains_key("child") {
        ::core::panicking::panic("assertion failed: errs.contains_key(\\\"child\\\")")
    }
    if let ValidationErrorsKind::List(ref errs) = errs["child"] {
        if !errs.contains_key(&0) {
            ::core::panicking::panic("assertion failed: errs.contains_key(&0)")
        }
        unwrap_map(
            &errs[&0],
            |errs| {
                match (&errs.len(), &1) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                if !errs.contains_key("value") {
                    ::core::panicking::panic(
                        "assertion failed: errs.contains_key(\\\"value\\\")",
                    )
                }
                if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                    match (&errs.len(), &1) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    match (&errs[0].code, &"length") {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                } else {
                    {
                        ::std::rt::begin_panic("Expected field validation errors");
                    };
                }
            },
        );
    } else {
        {
            ::std::rt::begin_panic("Expected list validation errors");
        };
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_can_validate_set_fields"]
pub const test_can_validate_set_fields: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_can_validate_set_fields"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "validator_derive_tests/tests/nested.rs",
        start_line: 559usize,
        start_col: 4usize,
        end_line: 559usize,
        end_col: 32usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        test_can_validate_set_fields(),
    )),
};
fn test_can_validate_set_fields() {
    struct ParentWithSetOfChildren {
        #[validate(length(min = 1))]
        #[validate(nested)]
        child: HashSet<Child>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ParentWithSetOfChildren {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ParentWithSetOfChildren",
                "child",
                &&self.child,
            )
        }
    }
    impl ::validator::Validate for ParentWithSetOfChildren {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            use validator::ValidateNested;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.child.validate_length(Some(1), None, None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &1);
                err.add_param(::std::borrow::Cow::from("value"), &self.child);
                errors.add("child", err);
            }
            errors.add_non_nested("child", self.child.validate_nested("child"));
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    let instance = ParentWithSetOfChildren {
        child: [Child { value: String::new() }].iter().cloned().collect(),
    };
    let res = instance.validate();
    if !res.is_err() {
        ::core::panicking::panic("assertion failed: res.is_err()")
    }
    let err = res.unwrap_err();
    let errs = err.errors();
    match (&errs.len(), &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    if !errs.contains_key("child") {
        ::core::panicking::panic("assertion failed: errs.contains_key(\\\"child\\\")")
    }
    if let ValidationErrorsKind::List(ref errs) = errs["child"] {
        if !errs.contains_key(&0) {
            ::core::panicking::panic("assertion failed: errs.contains_key(&0)")
        }
        unwrap_map(
            &errs[&0],
            |errs| {
                match (&errs.len(), &1) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                if !errs.contains_key("value") {
                    ::core::panicking::panic(
                        "assertion failed: errs.contains_key(\\\"value\\\")",
                    )
                }
                if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                    match (&errs.len(), &1) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    match (&errs[0].code, &"length") {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                } else {
                    {
                        ::std::rt::begin_panic("Expected field validation errors");
                    };
                }
            },
        );
    } else {
        {
            ::std::rt::begin_panic("Expected list validation errors");
        };
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_can_validate_ref_set_fields"]
pub const test_can_validate_ref_set_fields: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_can_validate_ref_set_fields"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "validator_derive_tests/tests/nested.rs",
        start_line: 595usize,
        start_col: 4usize,
        end_line: 595usize,
        end_col: 36usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        test_can_validate_ref_set_fields(),
    )),
};
fn test_can_validate_ref_set_fields() {
    struct ParentWithRefSetOfChildren<'a> {
        #[validate(length(min = 1))]
        #[validate(nested)]
        child: &'a HashSet<Child>,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for ParentWithRefSetOfChildren<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ParentWithRefSetOfChildren",
                "child",
                &&self.child,
            )
        }
    }
    impl<'a> ::validator::Validate for ParentWithRefSetOfChildren<'a> {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            use validator::ValidateNested;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.child.validate_length(Some(1), None, None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &1);
                err.add_param(::std::borrow::Cow::from("value"), &self.child);
                errors.add("child", err);
            }
            errors.add_non_nested("child", self.child.validate_nested("child"));
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    let child = [Child { value: String::new() }].iter().cloned().collect();
    let instance = ParentWithRefSetOfChildren {
        child: &child,
    };
    let res = instance.validate();
    if !res.is_err() {
        ::core::panicking::panic("assertion failed: res.is_err()")
    }
    let err = res.unwrap_err();
    let errs = err.errors();
    match (&errs.len(), &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    if !errs.contains_key("child") {
        ::core::panicking::panic("assertion failed: errs.contains_key(\\\"child\\\")")
    }
    if let ValidationErrorsKind::List(ref errs) = errs["child"] {
        if !errs.contains_key(&0) {
            ::core::panicking::panic("assertion failed: errs.contains_key(&0)")
        }
        unwrap_map(
            &errs[&0],
            |errs| {
                match (&errs.len(), &1) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                if !errs.contains_key("value") {
                    ::core::panicking::panic(
                        "assertion failed: errs.contains_key(\\\"value\\\")",
                    )
                }
                if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                    match (&errs.len(), &1) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    match (&errs[0].code, &"length") {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                } else {
                    {
                        ::std::rt::begin_panic("Expected field validation errors");
                    };
                }
            },
        );
    } else {
        {
            ::std::rt::begin_panic("Expected list validation errors");
        };
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_can_validate_option_set_fields"]
pub const test_can_validate_option_set_fields: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_can_validate_option_set_fields"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "validator_derive_tests/tests/nested.rs",
        start_line: 630usize,
        start_col: 4usize,
        end_line: 630usize,
        end_col: 39usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        test_can_validate_option_set_fields(),
    )),
};
fn test_can_validate_option_set_fields() {
    struct ParentWithOptionSetOfChildren {
        #[validate(length(min = 1))]
        #[validate(nested)]
        child: Option<HashSet<Child>>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ParentWithOptionSetOfChildren {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ParentWithOptionSetOfChildren",
                "child",
                &&self.child,
            )
        }
    }
    impl ::validator::Validate for ParentWithOptionSetOfChildren {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            use validator::ValidateNested;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.child.validate_length(Some(1), None, None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &1);
                err.add_param(::std::borrow::Cow::from("value"), &self.child);
                errors.add("child", err);
            }
            errors.add_non_nested("child", self.child.validate_nested("child"));
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    let instance = ParentWithOptionSetOfChildren {
        child: Some([Child { value: String::new() }].iter().cloned().collect()),
    };
    let res = instance.validate();
    if !res.is_err() {
        ::core::panicking::panic("assertion failed: res.is_err()")
    }
    let err = res.unwrap_err();
    let errs = err.errors();
    match (&errs.len(), &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    if !errs.contains_key("child") {
        ::core::panicking::panic("assertion failed: errs.contains_key(\\\"child\\\")")
    }
    if let ValidationErrorsKind::List(ref errs) = errs["child"] {
        if !errs.contains_key(&0) {
            ::core::panicking::panic("assertion failed: errs.contains_key(&0)")
        }
        unwrap_map(
            &errs[&0],
            |errs| {
                match (&errs.len(), &1) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                if !errs.contains_key("value") {
                    ::core::panicking::panic(
                        "assertion failed: errs.contains_key(\\\"value\\\")",
                    )
                }
                if let ValidationErrorsKind::Field(ref errs) = errs["value"] {
                    match (&errs.len(), &1) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    match (&errs[0].code, &"length") {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                } else {
                    {
                        ::std::rt::begin_panic("Expected field validation errors");
                    };
                }
            },
        );
    } else {
        {
            ::std::rt::begin_panic("Expected list validation errors");
        };
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_field_validations_take_priority_over_nested_validations"]
pub const test_field_validations_take_priority_over_nested_validations: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName(
            "test_field_validations_take_priority_over_nested_validations",
        ),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "validator_derive_tests/tests/nested.rs",
        start_line: 666usize,
        start_col: 4usize,
        end_line: 666usize,
        end_col: 64usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        test_field_validations_take_priority_over_nested_validations(),
    )),
};
fn test_field_validations_take_priority_over_nested_validations() {
    struct ParentWithVectorOfChildren {
        #[validate(length(min = 1))]
        #[validate(nested)]
        child: Vec<Child>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ParentWithVectorOfChildren {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ParentWithVectorOfChildren",
                "child",
                &&self.child,
            )
        }
    }
    impl ::validator::Validate for ParentWithVectorOfChildren {
        fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
            use validator::ValidateLength;
            use validator::ValidateNested;
            let mut errors = ::validator::ValidationErrors::new();
            if !self.child.validate_length(Some(1), None, None) {
                let mut err = ::validator::ValidationError::new("length");
                err.add_param(::std::borrow::Cow::from("min"), &1);
                err.add_param(::std::borrow::Cow::from("value"), &self.child);
                errors.add("child", err);
            }
            errors.add_non_nested("child", self.child.validate_nested("child"));
            if errors.is_empty() {
                ::std::result::Result::Ok(())
            } else {
                ::std::result::Result::Err(errors)
            }
        }
    }
    let instance = ParentWithVectorOfChildren {
        child: Vec::new(),
    };
    let res = instance.validate();
    if !res.is_err() {
        ::core::panicking::panic("assertion failed: res.is_err()")
    }
    let err = res.unwrap_err();
    let errs = err.errors();
    match (&errs.len(), &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    if !errs.contains_key("child") {
        ::core::panicking::panic("assertion failed: errs.contains_key(\\\"child\\\")")
    }
    if let ValidationErrorsKind::Field(ref errs) = errs["child"] {
        match (&errs.len(), &1) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&errs[0].code, &"length") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    } else {
        {
            ::std::rt::begin_panic("Expected field validation errors");
        };
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_field_validation_errors_replaced_with_nested_validations_fails"]
pub const test_field_validation_errors_replaced_with_nested_validations_fails: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName(
            "test_field_validation_errors_replaced_with_nested_validations_fails",
        ),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "validator_derive_tests/tests/nested.rs",
        start_line: 693usize,
        start_col: 4usize,
        end_line: 693usize,
        end_col: 71usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::YesWithMessage(
            "Attempt to replace non-empty ValidationErrors entry",
        ),
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        test_field_validation_errors_replaced_with_nested_validations_fails(),
    )),
};
#[should_panic(expected = "Attempt to replace non-empty ValidationErrors entry")]
#[allow(unused)]
fn test_field_validation_errors_replaced_with_nested_validations_fails() {
    struct ParentWithOverridingStructValidations {
        child: Vec<Child>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ParentWithOverridingStructValidations {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ParentWithOverridingStructValidations",
                "child",
                &&self.child,
            )
        }
    }
    impl Validate for ParentWithOverridingStructValidations {
        #[allow(unused_mut)]
        fn validate(&self) -> Result<(), ValidationErrors> {
            let mut errors = ValidationErrors::new();
            if !validate_length(&self.child, Some(2u64), None, None) {
                let mut err = ValidationError::new("length");
                err.add_param(Cow::from("min"), &2u64);
                err.add_param(Cow::from("value"), &&self.child);
                errors.add("child", err);
            }
            let mut result = if errors.is_empty() { Ok(()) } else { Err(errors) };
            {
                let results: Vec<_> = self
                    .child
                    .iter()
                    .map(|child| {
                        let mut result = Ok(());
                        result = ValidationErrors::merge(
                            result,
                            "child",
                            child.validate(),
                        );
                        result
                    })
                    .collect();
                result = ValidationErrors::merge_all(result, "child", results);
            }
            result
        }
    }
    let instance = ParentWithOverridingStructValidations {
        child: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([Child { value: String::new() }]),
        ),
    };
    instance.validate();
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_field_validations_evaluated_after_nested_validations_fails"]
pub const test_field_validations_evaluated_after_nested_validations_fails: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName(
            "test_field_validations_evaluated_after_nested_validations_fails",
        ),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "validator_derive_tests/tests/nested.rs",
        start_line: 741usize,
        start_col: 4usize,
        end_line: 741usize,
        end_col: 67usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::YesWithMessage(
            "Attempt to add field validation to a non-Field ValidationErrorsKind instance",
        ),
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(
        test_field_validations_evaluated_after_nested_validations_fails(),
    )),
};
#[should_panic(
    expected = "Attempt to add field validation to a non-Field ValidationErrorsKind instance"
)]
#[allow(unused)]
fn test_field_validations_evaluated_after_nested_validations_fails() {
    struct ParentWithStructValidationsFirst {
        child: Vec<Child>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ParentWithStructValidationsFirst {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ParentWithStructValidationsFirst",
                "child",
                &&self.child,
            )
        }
    }
    impl Validate for ParentWithStructValidationsFirst {
        #[allow(unused_mut)]
        fn validate(&self) -> Result<(), ValidationErrors> {
            let mut result = Ok(());
            if !ValidationErrors::has_error(&result, "child") {
                let results: Vec<_> = self
                    .child
                    .iter()
                    .map(|child| {
                        let mut result = Ok(());
                        result = ValidationErrors::merge(
                            result,
                            "child",
                            child.validate(),
                        );
                        result
                    })
                    .collect();
                result = ValidationErrors::merge_all(result, "child", results);
            }
            if !validate_length(&self.child, Some(2u64), None, None) {
                let mut err = ValidationError::new("length");
                err.add_param(Cow::from("min"), &2u64);
                err.add_param(Cow::from("value"), &&self.child);
                result = result
                    .and_then(|_| Err(ValidationErrors::new()))
                    .map_err(|mut errors| {
                        errors.add("child", err);
                        errors
                    });
            }
            result
        }
    }
    let instance = ParentWithStructValidationsFirst {
        child: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([Child { value: String::new() }]),
        ),
    };
    let res = instance.validate();
}
fn unwrap_map<F>(errors: &ValidationErrors, f: F)
where
    F: FnOnce(HashMap<&'static str, ValidationErrorsKind>),
{
    let errors = errors.clone();
    f(errors.errors().clone());
}
#[rustc_main]
#[no_coverage]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &fails_nested_validation,
            &is_fine_with_nested_validations,
            &test_can_validate_array_fields,
            &test_can_validate_map_fields,
            &test_can_validate_option_fields_with_lifetime,
            &test_can_validate_option_fields_without_lifetime,
            &test_can_validate_option_map_fields,
            &test_can_validate_option_set_fields,
            &test_can_validate_option_vector_fields,
            &test_can_validate_ref_map_fields,
            &test_can_validate_ref_set_fields,
            &test_can_validate_set_fields,
            &test_can_validate_slice_fields,
            &test_can_validate_vector_fields,
            &test_field_validation_errors_replaced_with_nested_validations_fails,
            &test_field_validations_evaluated_after_nested_validations_fails,
            &test_field_validations_take_priority_over_nested_validations,
            &test_works_with_none_values,
        ],
    )
}
