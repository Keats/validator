# validator

![Build](https://github.com/Keats/validator/workflows/ci/badge.svg)

Macros 1.1 custom derive to simplify struct validation inspired by [marshmallow](http://marshmallow.readthedocs.io/en/latest/) and
[Django validators](https://docs.djangoproject.com/en/1.10/ref/validators/).

The minimum supported version is Rust 1.42.

Installation:

```toml
[dependencies]
validator = { version = "0.12", features = ["derive"] }
```

A short example:

```rust
use serde::Deserialize;

// A trait that the Validate derive will impl
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
struct SignupData {
    #[validate(email)]
    mail: String,
    #[validate(phone)]
    phone: String,
    #[validate(url)]
    site: String,
    #[validate(length(min = 1), custom = "validate_unique_username")]
    #[serde(rename = "firstName")]
    first_name: String,
    #[validate(range(min = 18, max = 20))]
    age: u32,
}

fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
    if username == "xXxShad0wxXx" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("terrible_username"));
    }

    Ok(())
}

match signup_data.validate() {
  Ok(_) => (),
  Err(e) => return e;
};
```

A validation on an `Option<_>` field will be executed on the contained type if the option is `Some`. The `validate()`
 method returns a `Result<(), ValidationErrors>`. In the case of an invalid result, the `ValidationErrors` instance includes
a map of errors keyed against the struct's field names. Errors may be represented in three ways, as described by the 
`ValidationErrorsKind` enum:

```rust
#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ValidationErrorsKind {
    Struct(Box<ValidationErrors>),
    List(BTreeMap<usize, Box<ValidationErrors>>),
    Field(Vec<ValidationError>),
}
```

In the simple example above, any errors would be of the `Field(Vec<ValidationError>)` type, where a single
`ValidationError` has the following structure:

```rust
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValidationError {
  pub code: Cow<'static, str>,
  pub message: Option<Cow<'static, str>>,
  pub params: HashMap<Cow<'static, str>, Value>,
}
```
The value of the field will automatically be added to the params with a key of `value`.

Note that `validator` works in conjunction with serde: in the example we can see that the `first_name`
field is renamed from/to `firstName`. Any error on that field will be in the `firstName` key of the hashmap,
not `first_name`.

The other two `ValidationErrorsKind` types represent errors discovered in nested (vectors of) structs, as described in
this example:

 ```rust
use serde::Deserialize;
// A trait that the Validate derive will impl
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
struct SignupData {
    #[validate]
    contact_details: ContactDetails,
    #[validate]
    preferences: Vec<Preference>,
    #[validate(required)]
    allow_cookies: Option<bool>,
}

#[derive(Debug, Validate, Deserialize)]
struct ContactDetails {
    #[validate(email)]
    mail: String,
    #[validate(phone)]
    phone: String
}

#[derive(Debug, Validate, Deserialize)]
struct Preference {
    #[validate(length(min = 4))]
    name: String,
    value: bool,
}

match signup_data.validate() {
  Ok(_) => (),
  Err(e) => return e;
};
 ```

Here, the `ContactDetails` and `Preference` structs are nested within the parent `SignupData` struct. Because
these child types also derive `Validate`, the fields where they appear can be tagged for inclusion in the parent
struct's validation method.

Any errors found in a single nested struct (the `contact_details` field in this example) would be returned as a
`Struct(Box<ValidationErrors>)` type in the parent's `ValidationErrors` result.

Any errors found in a vector of nested structs (the `preferences` field in this example) would be returned as a
`List(BTreeMap<usize, Box<ValidationErrors>>)` type in the parent's `ValidationErrors` result, where the map is keyed on
the index of invalid vector entries.


## Usage
You will need to import the `Validate` trait.

The `validator` crate can also be used without the custom derive as it exposes all the
validation functions and types.

## Validators
The crate comes with some built-in validators and you can have several validators for a given field.

### email
Tests whether the String is a valid email according to the HTML5 regex, which means it will mark
some esoteric emails as invalid that won't be valid in a `email` input as well.
This validator doesn't take any arguments: `#[validate(email)]`.

### url
Tests whether the String is a valid URL.
This validator doesn't take any arguments: `#[validate(url)]`;

### length
Tests whether a String or a Vec match the length requirement given. `length` has 3 integer arguments:

- min
- max
- equal

Using `equal` excludes the `min` or `max` and will result in a compilation error if they are found.

At least one argument is required with a maximum of 2 (having `min` and `max` at the same time).

Examples:

```rust
const MIN_CONST: u64 = 1;
const MAX_CONST: u64 = 10;

#[validate(length(min = 1, max = 10))]
#[validate(length(min = 1))]
#[validate(length(max = 10))]
#[validate(length(equal = 10))]
#[validate(length(min = "MIN_CONST", max = "MAX_CONST"))]
```

### range
Tests whether a number is in the given range. `range` takes 1 or 2 arguments `min` and `max` that can be a number or a value path.

Examples:

```rust
const MAX_CONSTANT: i32 = 10;
const MIN_CONSTANT: i32 = 0;

#[validate(range(min = 1))]
#[validate(range(min = "MIN_CONSTANT"))]
#[validate(range(min = 1, max = 10))]
#[validate(range(min = 1.1, max = 10.8))]
#[validate(range(max = 10.8))]
#[validate(range(min = "MAX_CONSTANT"))]
#[validate(range(min = "crate::MAX_CONSTANT"))]
```

### must_match
Tests whether the 2 fields are equal. `must_match` takes 1 string argument. It will error if the field
mentioned is missing or has a different type than the field the attribute is on.

Examples:

```rust
#[validate(must_match = "password2")]
#[validate(must_match(other = "password2"))]
```

### contains
Tests whether the string contains the substring given or if a key is present in a hashmap. `contains` takes
1 string argument.

Examples:

```rust
#[validate(contains = "gmail")]
#[validate(contains(pattern = "gmail"))]
```

### regex
Tests whether the string matches the regex given. `regex` takes
1 string argument: the path to a static Regex instance.

Examples:

```rust
lazy_static! {
    static ref RE_TWO_CHARS: Regex = Regex::new(r"[a-z]{2}$").unwrap();
}

#[validate(regex = "RE_TWO_CHARS")]
#[validate(regex(path = "RE_TWO_CHARS"))]
```

### credit\_card
Test whether the string is a valid credit card number.

Examples:

```rust
#[validate(credit_card)]
```

### phone
Tests whether the String is a valid phone number (in international format, ie.
containing the country indicator like `+14152370800` for an US number — where `4152370800`
is the national number equivalent, which is seen as invalid).
To use this validator, you must enable the `phone` feature for the `validator` crate.
This validator doesn't take any arguments: `#[validate(phone)]`;

### custom
Calls one of your functions to perform a custom validation. The field will reference be given as a parameter to the function,
which should return a `Result<(), ValidationError>`.

Examples:

```rust
#[validate(custom = "validate_something")]
#[validate(custom = "::utils::validate_something")]
#[validate(custom(function = "validate_something"))]
```

You can also parse arguments from the validation function to your custom validation by setting the `arg` parameter. `arg` can only be set to one type but you can set it to a tuple to pass multiple types at once. Defining the `arg` parameter will implement the `ValidateArgs` trait with the corresponding function types like this:

```rust
use validator::{Validate, ValidateArgs, ValidationError};

fn validate(value: &str, arg: (i64, i64)) -> Result<(), ValidationError> {
    [...]
}

#[derive(Debug, Validate)]
struct TestStruct {
    #[validate(custom(function = "validate", arg = "(i64, i64)"))]
    value: String,
}

let test_struct: TestStruct = [...]
test_struct.validate_args((77, 555)).is_ok();
```

It is also possible to pass references by using the lifetime `'v_a` not that this lifetime should only be used for the function parameters like this:

```rust
fn validate_value(_: &str, arg: &mut Database) -> Result<(), ValidationError> {
    [...]
}

#[derive(Debug, Validate)]
struct TestStruct {
    //                                                     vvvv This is the lifetime for references
    #[validate(custom(function = "validate_value", arg = "&'v_a mut Database"))]
    value: String,
}

let mut database: Database = [...]
let test_struct: TestStruct = [...]
test_struct.validate_args(&mut database).is_ok();
```

Custom validation with arguments doesn't work on nested validation. See [`validator_derive_tests/tests/custom.rs`](validator_derive_tests/custom.rs) and [`validator_derive_tests/tests/custom_args.rs`](validator_derive_tests/custom_args.rs) for more examples.

### nested
Performs validation on a field with a type that also implements the Validate trait (or a vector of such types).

Examples:

```rust
#[validate]
```

### non_control_character
Tests whether the String has any utf-8 control caracters, fails validation if it does.
To use this validator, you must enable the `unic` feature for the `validor` crate.
This validator doesn't take any arguments: `#[validate(non_control_character)]`;

### required
Tests whether the `Option<T>` field is `Some`;

### required_nested
Tests whether the `Option<T>` field is `Some` and performs validation as `nested` do;

## Struct level validation
Often, some error validation can only be applied when looking at the full struct, here's how it works here:

```rust
#[derive(Debug, Validate, Deserialize)]
#[validate(schema(function = "validate_category", skip_on_field_errors = false))]
struct CategoryData {
    category: String,
    name: String,
}
```

The function mentioned should return a `Result<(), ValidationError>` and will be called after validation is done for all fields.

The `skip_on_field_errors` defaults to `true` if not present and will ensure that the function is not called
if an error happened while validating the struct fields.

Any error on the struct level validation will appear in the key `__all__` of the hashmap of errors.

## Message and code

Each validator can take 2 optional arguments in addition to their own arguments:

- `message`: a message to go with the error, for example if you want to do i18n
- `code`: each validator has a default error code (for example the `regex` validator code is `regex`) but it can be overriden
if necessary, mainly needed for the `custom` validator

Note that these arguments can't be applied to nested validation calls with `#[validate]`.

For example, the following attributes all work:

```rust
// code attribute
#[validate(email(code = "code_str"))]
#[validate(credit_card(code = "code_str"))]
#[validate(length(min = 5, max = 10, code = "code_str"))]

#[validate(regex(path = "static_regex", code = "code_str"))]
#[validate(custom(function = "custom_fn", code = "code_str"))]
#[validate(contains(pattern = "pattern_str", code = "code_str"))]
#[validate(must_match(other = "match_value", code = "code_str"))]

// message attribute
#[validate(url(message = "message_str"))]
#[validate(length(min = 5, max = 10, message = "message_str"))]

#[validate(regex(path = "static_regex", message = "message_str"))]
#[validate(custom(function = "custom_fn", message = "message_str"))]
#[validate(contains(pattern = "pattern_str", message = "message_str"))]
#[validate(must_match(other = "match_value", message = "message_str"))]

// both attributes
#[validate(url(message = "message", code = "code_str"))]
#[validate(email(code = "code_str", message = "message"))]
#[validate(custom(function = "custom_fn", code = "code_str", message = "message_str"))]

```

## Changelogs

### validator

## 0.12.0 (2020/11/26)

- Allow `length` and `range` validators to take a reference to a variable
- Make validator work with `Option<Vec<_>>`
- Support reference for length types
- Fix `phone`, `unic` and `card` feature to actually work

## 0.11.0 (2020/09/09)

- Add a `derive` feature so you don't need to add `validator_derive` to your `Cargo.toml`

#### 0.10.1 (2020/06/09)

- Add a blanket Validate implementation for references
- Add `Required` and `RequiredNested` validators

#### 0.10.0 (2019/10/18)

- Add `non_control_characters` validation

#### 0.9.0 (2019/05/xx)

- `ValidationErrors::errors` and `ValidationErrors::field_errors` now use `&self` instead of `self`
- Move to edition 2018

#### 0.8.0 (2018/09/19)

- Change error type to allow use with nested validation

#### 0.7.1 (2018/07/27)

- Make validators work on `Cow`

#### 0.7.0 (2018/05/29)

- Feature gate the card validator

#### 0.6.2 (2017/11/08)

- Fix credit card validation being incorrect in enum

#### 0.6.1 (2017/11/08)

- Add international phone number and credit card validation

#### 0.6.0 (2017/08/12)

- Re-design `ValidationError` and `Validate` trait

### validator_derive

#### 0.11.0 (2020/09/09)

- Errors in the proc macro attributes will now point to the exact place the error is

#### 0.10.1 (2020/06/09)

- Handle `Required` and `RequiredNested` validators

#### 0.10.0 (2019/10/18)

- Update syn & quote
- Move to edition 2018

#### 0.9.0 (2019/05/xx)

- Use literals in macros now that it's stable -> bumping minimum Rust version to 1.30

#### 0.8.0 (2018/09/19)

- Allow nested validation

#### 0.7.2 (2018/07/27)

- Make validators work on `Cow`

#### 0.7.1 (2018/06/28)

- Update dependencies

#### 0.7.0 (2018/05/29)

- Feature gate the card validator

#### 0.6.5 (2018/04/14)

- Fix path for regex starting with `::`
- Update syn and quote

#### 0.6.4 (2018/03/20)

- Support `Option<Option<T>>` types

#### 0.6.3 (2018/03/19)

- Fix path for custom validators starting with `::`

#### 0.6.2 (2018/03/17)

- Update syn and quote

#### 0.6.1 (2017/11/08)

- Add international phone number and credit card derive

#### 0.6.0 (2017/08/12)

- Change generated code to make the new design of errors work

### Previous

#### 0.5.0 (2017/05/22) > validator_derive only

- Fix range validator not working on Option
- Update to serde 1.0

#### 0.4.1 (2017/02/14) > validator_derive only

- Fix potential conflicts with other attributes

#### 0.4.0 (2017/01/30)

- Validators now work on `Option` field and struct/fields with lifetimes

#### 0.3.0 (2017/01/17)

- Add `contains` and `regex` validator
- BREAKING: change `Errors` type to be a newtype in order to extend it

#### 0.2.0 (2017/01/17)

- Remove need for `attr_literals` feature
- Fix error when not having validation on each field
- Add struct level validation
- Add `must_match` validator
