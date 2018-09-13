# validator

[![Build Status](https://travis-ci.org/Keats/validator.svg)](https://travis-ci.org/Keats/validator)

Macros 1.1 custom derive to simplify struct validation inspired by [marshmallow](http://marshmallow.readthedocs.io/en/latest/) and
[Django validators](https://docs.djangoproject.com/en/1.10/ref/validators/).
It relies on the `proc_macro` feature which is stable since Rust 1.15.

By default all args to a `validate` must be strings if you are using stable.
However, if you are using nightly, you can also activate the `attr_literals` feature to be able to use int, float and boolean as well.


A short example:

```rust
#[macro_use]
extern crate validator_derive;
extern crate validator;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

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
    #[validate(length(min = "1"), custom = "validate_unique_username")]
    #[serde(rename = "firstName")]
    first_name: String,
    #[validate(range(min = "18", max = "20"))]
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

The `validate()` method returns a `Result<(), ValidationErrors>`. In the case of an invalid result, the
`ValidationErrors` instance includes a map of errors keyed against the struct's field names. Errors may be represented
in three ways, as described by the `ValidationErrorsKind` enum:

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

If you are adding a validation on a `Option<..>` field, it will only be ran if there is a value. The exception
being `must_match` that doesn't currently work with `Option` due to me not finding a use case for it. If you have one,
please comment on https://github.com/Keats/validator/issues/7.

The other two `ValidationErrorsKind` types represent errors discovered in nested (vectors of) structs, as described in
this example:

 ```rust
#[macro_use]
extern crate validator_derive;
extern crate validator;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Validate, Deserialize)]
struct SignupData {
    #[validate]
    contact_details: ContactDetails,
    #[validate]
    preferences: Vec<Preference>
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
    #[validate(length(min = "4"))]
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
You will need to import the `Validate` trait, and optionally use the `attr_literals` feature.

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
#[validate(length(min = "1", max = "10"))]
#[validate(length(min = "1"))]
#[validate(length(max = "10"))]
#[validate(length(equal = "10"))]
```

### range
Tests whether a number is in the given range. `range` takes 2 number arguments: `min` and `max`.

Examples:

```rust
#[validate(range(min = "1", max = "10"))]
#[validate(range(min = "1", max = "10.8"))]
#[validate(range(min = "1.1", max = "10.8"))]
```

### must_match
Tests whether the 2 fields are equal. `must_match` takes 1 string argument. It will error if the field
mentioned is missing or has a different type than the field the attribute is on.

Examples:

```rust
#[validate(must_match = "password2")]
```

### contains
Tests whether the string contains the substring given or if a key is present in a hashmap. `contains` takes
1 string argument.

Examples:

```rust
#[validate(contains = "gmail")]
```

### regex
Tests whether the string matches the regex given. `regex` takes
1 string argument: the path to a static Regex instance.

Examples:

```rust
#[validate(regex = "ALLOWED_USERNAMES_RE")]
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
To use this validator, you must enable the `phone` feature for the `validator_derive` crate.
This validator doesn't take any arguments: `#[validate(phone)]`;

### custom
Calls one of your function to do a custom validation.
The field will be given as parameter and it should return a `Option<String>` representing the error code,
if there was an error.

Examples:

```rust
#[validate(custom = "validate_something")]
#[validate(custom = "::utils::validate_something")]
```

### nested
Performs validation on a field with a type that also implements the Validate trait (or a vector of such types).

Examples:

```rust
#[validate]
```

## Struct level validation
Often, some error validation can only be applied when looking at the full struct, here's how it works here:

```rust
#[derive(Debug, Validate, Deserialize)]
#[validate(schema(function = "validate_category", skip_on_field_errors = "false")]
struct CategoryData {
    category: String,
    name: String,
}
```

The function mentioned should return a `Option<(String, String)>` where the tuple is (key error, error code)
and will be called after validation is done for all fields.
This means that the error can be reported on an existing field or on a new key.

The `skip_on_field_errors` defaults to `true` if not present and will ensure that the function is not called
if an error happened while validating the struct fields.

Any error on the schema level validation will appear in the key `__all__` of the hashmap of errors.

## Message and code

Each validator can take 2 optional arguments in addition to their own arguments:

- `message`: a message to go with the error, for example if you want to do i18n
- `code`: each validator has a default error code (for example the `regex` validator code is `regex`) but it can be overriden
if necessary, mainly needed for the `custom` validator

For example, the following attributes all work:

```rust
#[validate(email)]
#[validate(email(code="mail"))]
#[validate(email(message="Email %s is not valid"))]
#[validate(email(code="mail", message="Email %s is not valid"))]
```

## Changelogs

### validator

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
