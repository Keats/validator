# validator

<div align="center">
  <!-- Github Actions -->
  <img src="https://github.com/Keats/validator/workflows/ci/badge.svg" alt="actions status" />
  <!-- Version -->
  <a href="https://crates.io/crates/validator">
    <img src="https://img.shields.io/crates/v/validator.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Docs -->
  <a href="https://docs.rs/validator">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/validator">
    <img src="https://img.shields.io/crates/d/validator.svg?style=flat-square"
      alt="Download" />
  </a>
</div>


Macros 1.1 custom derive to simplify struct validation inspired by [marshmallow](http://marshmallow.readthedocs.io/en/latest/) and
[Django validators](https://docs.djangoproject.com/en/1.10/ref/validators/).

Installation:

```toml
[dependencies]
validator = { version = "0.19", features = ["derive"] }
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
    #[validate(url)]
    site: String,
    #[validate(length(min = 1), custom(function = "validate_unique_username"))]
    #[serde(rename = "firstName")]
    first_name: String,
    #[validate(range(min = 18, max = 20))]
    age: u32,
    #[validate(range(exclusive_min = 0.0, max = 100.0))]
    height: f32,
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

The other two `ValidationErrorsKind` types represent errors discovered in nested (vectors of) structs, as described in
this example:

 ```rust
use serde::Deserialize;
// A trait that the Validate derive will impl
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
struct SignupData {
    #[validate(nested)]
    contact_details: ContactDetails,
    #[validate(nested)]
    preferences: Vec<Preference>,
    #[validate(required)]
    allow_cookies: Option<bool>,
}

#[derive(Debug, Validate, Deserialize)]
struct ContactDetails {
    #[validate(email)]
    mail: String,
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
Tests whether a number is in the given range. `range` takes 1 or 2 arguments, and they can be normal (`min` and `max`) or exclusive (`exclusive_min`, `exclusive_max`, unreachable limits).
These can be a number or a value path.

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
#[validate(range(exclusive_min = 0.0, max = 100.0))]
#[validate(range(exclusive_max = 10))]
// If you get an error saying the literal doesn't fit in i32, specify a number type in the literal directly
#[validate(range(max = 1000000000u64))]

```

### must_match
Tests whether the 2 fields are equal. `must_match` takes 1 string argument. It will error if the field
mentioned is missing or has a different type than the field the attribute is on.

Examples:

```rust
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

### does_not_contain
Pretty much the opposite of contains, provided just for ease-of-use. Tests whether a container does not contain
the substring given if it's a string or if a key is NOT present in a hashmap. `does_not_contain` takes 1 string argument.

Examples:

```rust
#[validate(does_not_contain = "gmail")]
#[validate(does_not_contain(pattern = "gmail"))]
```

### regex
Tests whether the string matches the regex given. `regex` takes
1 string argument: the path to a static Regex instance.

Examples:

```rust
use once_cell::sync::Lazy;

static RE_TWO_CHARS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[a-z]{2}$").unwrap()
});

#[validate(regex(path = *RE_TWO_CHARS)]
```

### credit\_card
Test whether the string is a valid credit card number.

Examples:

```rust
#[validate(credit_card)]
```

### custom
Calls one of your functions to perform a custom validation. The field reference (or value if it can be copied cheaply like numbers) will be given as a parameter to the function,
which should return a `Result<(), ValidationError>`.

Examples:

```rust
#[validate(custom(function = "validate_something"))]
#[validate(custom(function = "::utils::validate_something"))]
```

You can also do your own validation by parsing the arguments from the validation function by setting `context` for struct.　Applying custom validation using the `use_context` argument is accomplished by setting the `use_context` parameter. Defining the `context` parameter will implement the `ValidateArgs` trait with the corresponding function types like this:

```rust
use validator::{Validate, ValidateArgs, ValidationError};

fn validate(value: &str, context: &TestContext) -> Result<(), ValidationError> {
    [...]
}

struct TestContext(i64, i64);

#[derive(Debug, Validate)]
#[validate(context = TestContext)]
struct TestStruct {
    #[validate(custom(function = "validate", use_context))]
    value: String,
}

let test_struct: TestStruct = [...];
let test_context: TestContext = [...];
test_struct.validate_with_args(&test_context).is_ok();
```

It is also possible to pass references by using the lifetime `'v_a` note that this lifetime should only be used for the function parameters like this:

```rust
fn validate_value(_: &str, arg: &mut Database) -> Result<(), ValidationError> {
    [...]
}

#[derive(Debug, Validate)] //   vvvv This is the lifetime for references
#[validate(context = "Database<'v_a>", mutable)]
struct TestStruct {
    #[validate(custom(function = "validate_value", use_context))]
    value: String,
}

let mut database: Database = [...];
let test_struct: TestStruct = [...];
test_struct.validate_with_args(&mut database).is_ok();
```

Custom validation with arguments doesn't work on nested validation. See [`validator_derive_tests/tests/custom.rs`](https://github.com/Keats/validator/blob/master/validator_derive_tests/tests/custom.rs) and [`validator_derive_tests/tests/custom_args.rs`](https://github.com/Keats/validator/blob/master/validator_derive_tests/tests/custom_args.rs) for more examples.

### nested
Performs validation on a field with a type that also implements the Validate trait (or a vector of such types).

Examples:

```rust
#[validate(nested)]
```

### non_control_character
Tests whether the String has any utf-8 control characters, fails validation if it does.
To use this validator, you must enable the `unic` feature for the `validator` crate.
This validator doesn't take any arguments: `#[validate(non_control_character)]`;

### required
Tests whether the `Option<T>` field is `Some`;

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
- `code`: each validator has a default error code (for example the `regex` validator code is `regex`) but it can be overridden
if necessary, mainly needed for the `custom` validator

Note that these arguments can't be applied to nested validation calls with `#[validate]`.

For example, the following attributes all work:

```rust
// code attribute
#[validate(email(code = "code_str"))]
#[validate(credit_card(code = "code_str"))]
#[validate(length(min = 5, max = 10, code = "code_str"))]

#[validate(regex(path = *static_regex, code = "code_str"))]
#[validate(custom(function = "custom_fn", code = "code_str"))]
#[validate(contains(pattern = "pattern_str", code = "code_str"))]
#[validate(does_not_contain(pattern = "pattern_str", code = "code_str"))]
#[validate(must_match(other = "match_value", code = "code_str"))]

// message attribute
#[validate(url(message = "message_str"))]
#[validate(length(min = 5, max = 10, message = "message_str"))]

#[validate(regex(path = *static_regex, message = "message_str"))]
#[validate(custom(function = "custom_fn", message = "message_str"))]
#[validate(contains(pattern = "pattern_str", message = "message_str"))]
#[validate(does_not_contain(pattern = "pattern_str", message = "message_str"))]
#[validate(must_match(other = "match_value", message = "message_str"))]

// both attributes
#[validate(url(message = "message", code = "code_str"))]
#[validate(email(code = "code_str", message = "message"))]
#[validate(custom(function = "custom_fn", code = "code_str", message = "message_str"))]

```

## Features
`derive` - This allows for the use of the derive macro.
`derive_nightly_features` - This imports both derive as well as proc-macro-error2 nightly features. This allows proc-macro-error2 to emit extra nightly warnings.
