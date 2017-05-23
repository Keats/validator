extern crate url;
extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate idna;
extern crate serde;
#[macro_use] extern crate serde_derive;


mod types;
mod ip;
mod email;
mod length;
mod range;
mod urls;
mod must_match;
mod contains;


pub use types::{Errors, Validate, Validator};
pub use ip::{validate_ip, validate_ip_v4, validate_ip_v6};
pub use email::{validate_email};
pub use length::{HasLen, validate_length};
pub use range::{validate_range};
pub use urls::{validate_url};
pub use must_match::{validate_must_match};
pub use contains::{Contains, validate_contains};
