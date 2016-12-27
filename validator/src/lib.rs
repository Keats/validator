extern crate url;
extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate idna;


mod types;
mod ip;
mod email;
mod length;
mod range;
mod urls;


pub use types::{Errors, Validate, Validator};
pub use ip::{validate_ip, validate_ip_v4, validate_ip_v6};
pub use email::{validate_email};
pub use length::{HasLen, validate_length};
pub use range::{validate_range};
pub use urls::{validate_url};
