## Changelog

## 0.15.0 (unreleased)

- Allow passing args to schema validator
- Implement HasLen for map/set types
- 
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
