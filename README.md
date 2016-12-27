# validator

[![Build Status](https://travis-ci.org/Keats/validator.svg)](https://travis-ci.org/Keats/validator)

Status: Experimental - do not use in production

Macros 1.1 custom derive to simplify struct validation inspired by [marshmallow](http://marshmallow.readthedocs.io/en/latest/) and
[Django validators](https://docs.djangoproject.com/en/1.10/ref/validators/).
It currently only works on nightly but should work on stable in Rust 1.15.

A short example:
```rust
#![feature(proc_macro, attr_literals)]

#[macro_use] extern crate validator_derive;
extern crate validator;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

// A trait that the Validate derive will impl
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
struct SignupData {
    #[validate(email)]
    mail: String,
    #[validate(url)]
    site: String,
    #[validate(length(min = 1), custom = "validate_unique_username")]
    #[serde(rename = "firstName")]
    first_name: String,
    #[validate(range(min = 18, max = 20))]
    age: u32,
}

fn validate_unique_username(username: &str) -> Option<String> {
    if username == "xXxShad0wxXx" {
        return Some("terrible_username".to_string());
    }

    None
}

// load the struct from some json...
// `validate` returns `Result<(), HashMap<String, Vec<String>>>`
signup_data.validate()?;
```

This crate only sends back error codes for each field, it's up to you to write a message
for each error code.

Note that `validator` works in conjunction with serde: in the example we can see that the `first_name`
field is renamed from/to `firstName`. Any error on that field will be in the `firstName` key of the hashmap,
not `first_name`.

## Usage
You will need to add the `proc_macro` and `attr_literals` as seen in the example, as
well as importing the `Validate` trait.

The `validator` crate can also be used without the custom derive as it exposes all the
validation functions.

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
#[validate(length(min = 1, max = 10))]
#[validate(length(min = 1))]
#[validate(length(max = 10))]
#[validate(length(equal = 10))]
```

### range
Tests whether a number is in the given range. `range` takes 2 number arguments: `min` and `max`.

Examples:

```rust
#[validate(range(min = 1, max = 10))]
#[validate(range(min = 1, max = 10.8))]
#[validate(range(min = 1.1, max = 10.8))]
```

### custom
Calls one of your function to do a custom validation. 
The field will be given as parameter and it should return a `Option<String>` representing the error code,
if there was an error.

Examples:

```rust
#[validate(custom = "validate_something")]
#[validate(custom = "::utils::validate_something"]
```

TODO: have it return a bool and pass a `code` to the `custom` validator instead?
