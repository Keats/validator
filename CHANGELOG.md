## Changelog


## Unreleased
- Implement `AsRegex` for `std::sync::LazyLock<Regex>`
- Bug fix for nested issue with custom only running nested if outer passes


## 0.19.0 (2024/11/03)

- Swap to using proc-macro-error-2 instead of proc-macro-error for Syn
- Bumped MSRV to 1.81 because of error naming changes.
- Add more ValidateRegex impl


## 0.18.1 (2024/04/11)

- Fix multiple custom validation
- Fix nested error handling


## 0.18.0 (2024/04/05)

- Fix regressions from the derive rewrite, some things are back to 0.16 (eg custom functions)
- Remove require_nested, use required and nested validators instead
- Always require `nested` on the field for nested validation


## 0.17.0 (2024/03/04)

- Derive macro has been entirely rewritten
- Validation is now done through traits that you can implement
- Remove phone validator
- Remove automatic use of serde rename for fields name (temporary)

## 0.16.0 (2022/06/27)

- Allow passing code/message to `required`
- Add `does_not_contain` validator
- Check email length before validating it

## 0.15.0 (2022/05/03)

- Allow passing args to schema validator
- Implement HasLen for map/set types
- Remove `validator_types` from validator crate
- Add ValidationErrors::errors_mut
- Ignore unsupported fields rather than erroring

## 0.14.0 (2021/06/29)

- Allow passing arguments to custom functions
- Better `Display` implementation
- Better parsing of schema validation function in derive

## 0.13.0 (2021/03/22)

- Allow multiple schema-level validations

## 0.12.0 (2020/11/26)

- Allow `length` and `range` validators to take a reference to a variable
- Make validator work with `Option<Vec<_>>`
- Support reference for length types
- Fix `phone`, `unic` and `card` feature to actually work

## 0.11.0 (2020/09/09)

- Add a `derive` feature so you don't need to add `validator_derive` to your `Cargo.toml`

## 0.10.1 (2020/06/09)

- Add a blanket Validate implementation for references
- Add `Required` and `RequiredNested` validators

## 0.10.0 (2019/10/18)

- Add `non_control_characters` validation

## 0.9.0 (2019/05/xx)

- `ValidationErrors::errors` and `ValidationErrors::field_errors` now use `&self` instead of `self`
- Move to edition 2018

## 0.8.0 (2018/09/19)

- Change error type to allow use with nested validation

## 0.7.1 (2018/07/27)

- Make validators work on `Cow`

## 0.7.0 (2018/05/29)

- Feature gate the card validator

## 0.6.2 (2017/11/08)

- Fix credit card validation being incorrect in enum

## 0.6.1 (2017/11/08)

- Add international phone number and credit card validation

## 0.6.0 (2017/08/12)

- Re-design `ValidationError` and `Validate` trait

## 0.11.0 (2020/09/09)

- Errors in the proc macro attributes will now point to the exact place the error is

## 0.10.1 (2020/06/09)

- Handle `Required` and `RequiredNested` validators

## 0.10.0 (2019/10/18)

- Update syn & quote
- Move to edition 2018

## 0.9.0 (2019/05/xx)

- Use literals in macros now that it's stable -> bumping minimum Rust version to 1.30

## 0.8.0 (2018/09/19)

- Allow nested validation

## 0.7.2 (2018/07/27)

- Make validators work on `Cow`

## 0.7.1 (2018/06/28)

- Update dependencies

## 0.7.0 (2018/05/29)

- Feature gate the card validator

## 0.6.5 (2018/04/14)

- Fix path for regex starting with `::`
- Update syn and quote

## 0.6.4 (2018/03/20)

- Support `Option<Option<T>>` types

## 0.6.3 (2018/03/19)

- Fix path for custom validators starting with `::`

## 0.6.2 (2018/03/17)

- Update syn and quote

## 0.6.1 (2017/11/08)

- Add international phone number and credit card derive

## 0.6.0 (2017/08/12)

- Change generated code to make the new design of errors work

## 0.5.0 (2017/05/22) > validator_derive only

- Fix range validator not working on Option
- Update to serde 1.0

## 0.4.1 (2017/02/14) > validator_derive only

- Fix potential conflicts with other attributes

## 0.4.0 (2017/01/30)

- Validators now work on `Option` field and struct/fields with lifetimes

## 0.3.0 (2017/01/17)

- Add `contains` and `regex` validator
- BREAKING: change `Errors` type to be a newtype in order to extend it

## 0.2.0 (2017/01/17)

- Remove need for `attr_literals` feature
- Fix error when not having validation on each field
- Add struct level validation
- Add `must_match` validator
