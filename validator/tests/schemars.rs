/// Integration test for schemars feature
use std::{
    ops::Deref,
    collections::{HashMap},
    convert::From,
};

use rocket::{
    post,
    http::Status,
    data::{Data, FromData, Limits, Outcome},
    request::{local_cache, },
    response::{content::RawJson, Responder, Result as ResponseResult},
    serde::{
        json::{serde_json, Error as JsonError, Json},
        Serialize, Deserialize
    },
    Request,
};
use rocket_okapi::{
    gen::OpenApiGenerator,
    okapi::{schemars, schemars::JsonSchema, {Map, openapi3::{MediaType, RequestBody, Responses}}},
    request::OpenApiFromData,
    response::OpenApiResponderInner,
    openapi_get_routes,
    openapi,
    util::add_schema_response,
};
use validator::{ValidationError, ValidationErrors};
use validator_derive::Validate;
use std::iter::FromIterator;

/// Input data for validation
#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
#[serde(crate = "rocket::serde")]
pub struct Contact {
    #[validate(phone)]
    pub phone: String
}

/// Tested OpenAPI operation definition
///
/// Returns created `Contact` or `OperationError`.
///
/// Serialized validation error example:
/// ```
/// {
///     "code": "ValidationErrors",
///     "error": {
///         "phone": [
///             {
///                 "code": "phone",
///                 "message": null,
///                 "value": "AAAAAAAAAAAAAAAAAAAAAAA"
///             }
///         ]
///     }
/// }
/// ```
#[openapi(tag = "Contacts")]
#[post("/contact", format = "json", data = "<contact>")]
pub async fn create_contact(contact: ValidatedResult<Contact>) -> JsonResult<Contact> {
    let contact = contact?.into_inner();
    Ok(Json(contact))
}

// test code starts
use rocket::{Build, Rocket, launch};

#[launch]
fn configure_rocket() -> Rocket<Build> {
    rocket::build().mount("/api/", openapi_get_routes![create_contact])
}

use rocket::local::blocking::Client;
use rocket::http::ContentType;

#[test]
fn test_validation_json_error_response() {
    let rocket = configure_rocket();
    let client = Client::untracked(rocket).expect("valid rocket");
    let request = client.post("/api/contact")
        .header(ContentType::JSON).body(r#"{ "phone": "foo" }"#);
    let response = request.dispatch();

    assert_eq!(response.status(), Status::UnprocessableEntity);
    let str_response = response.into_string().expect("some string");
    assert!(str_response.contains(r#"{"code":"ValidationErrors","error":{"phone":[{"code":"phone","message":null,"params":{"value":"foo"}}]}}"#));
}

// test code ends

// below are utilities like API error type and ValidatedResult data guard

/// API operation error is part of our API
#[derive(Debug, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[serde(tag = "code", content = "error")]
pub enum OperationError {
    /// IO related error.
    ///
    /// HTTP status code 400
    IOError(String),
    /// Provided data is not valid json
    ///
    /// HTTP status code 400
    JsonParseError(String),
    /// Request body is too large.
    ///
    /// HTTP status code 413
    PayloadTooLarge(String),
    /// Failed to deserialize json.
    ///
    /// HTTP status code 422
    ///
    /// Relates to:
    ///     required fields /
    ///     enum variants /
    ///     invalid values for string formats (
    ///         ulid /
    ///         decimal
    ///     )
    DeserializationError(String),
    /// Returned when input data doesn't pass validation
    ///
    /// HTTP status code 422
    ///
    /// Field length checks, email, phone formats.
    #[schemars(example = "validation_errors")]
    ValidationErrors(ValidationErrors),
}

/// Shortcut for operation return type.
pub type JsonResult<T> = Result<Json<T>, OperationError>;

impl OperationError {
    /// We may return different http status codes for different error variants
    pub fn http_status(&self) -> Status {
        match self {
            OperationError::IOError(_) => Status::BadRequest,
            OperationError::JsonParseError(_) => Status::BadRequest,
            OperationError::PayloadTooLarge(_) => Status::PayloadTooLarge,
            _ => Status::UnprocessableEntity,
        }
    }
}

impl From<ValidationErrors> for OperationError {
    fn from(err: ValidationErrors) -> Self {
        OperationError::ValidationErrors(err)
    }
}

impl From<std::io::Error> for OperationError {
    fn from(err: std::io::Error) -> Self {
        if err.kind() == std::io::ErrorKind::UnexpectedEof {
            OperationError::PayloadTooLarge(format!("{err}"))
        } else {
            OperationError::IOError(format!("io error: {err}"))
        }
    }
}
impl<'r> From<JsonError<'r>> for OperationError {
    fn from(err: JsonError<'r>) -> Self {
        // repeat logic in Json::from_data
        match err {
            JsonError::Io(e) => Self::from(e),
            JsonError::Parse(s, e) if e.classify() == serde_json::error::Category::Data => {
                OperationError::DeserializationError(format!(
                    "deserialization failed: {e}, json document: {s}"
                ))
            }
            JsonError::Parse(s, e) => OperationError::JsonParseError(format!(
                "json parse error: {e}, json document: {s}"
            )),
        }
    }
}

/// Serializes the wrapped OperationError into JSON. Returns a response with Content-Type
/// JSON and a fixed-size body with the serialized value. If serialization
/// fails, an `Err` of `Status::InternalServerError` is returned.
impl<'r> Responder<'r, 'static> for OperationError {
    fn respond_to(self, req: &'r Request<'_>) -> ResponseResult<'static> {
        let string = serde_json::to_string(&self).map_err(|e| {
            println!("JSON failed to serialize: {:?}", e);
            Status::InternalServerError
        })?;

        (self.http_status(), RawJson(string)).respond_to(req)
    }
}

/// Serializes the wrapped value into JSON. Returns a response with `Content-Type` `JSON` and a
/// fixed-size body with the serialized value. If serialization fails,
/// an `Err` of `Status::InternalServerError` is returned.
impl OpenApiResponderInner for OperationError {
    fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        let mut responses = Responses::default();
        let schema = gen.json_schema::<OperationError>();
        add_schema_response(&mut responses, 400, "application/json", schema.clone())?;
        add_schema_response(&mut responses, 413, "application/json", schema.clone())?;
        add_schema_response(&mut responses, 422, "application/json", schema)?;
        // 500 status is not added because an endpoint can handle this, so it might never return
        // this error type.
        Ok(responses)
    }
}

fn validation_errors() -> OperationError {
    let mut val_errors = ValidationErrors::new();
    val_errors.add(
        "string_upto_20chars",
        ValidationError {
            code: "length".into(),
            message: None,
            params: HashMap::from_iter([
                ("min".into(), serde_json::json!(1_u32)),
                ("max".into(), serde_json::json!(20_u32)),
                ("value".into(), serde_json::json!("AAAAAAAAAAAAAAAAAAAAAAA")),
            ]),
        }
    );
    OperationError::ValidationErrors(val_errors)
}

/// Data guard that combines Json deserialization and validation
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Validated<T>(pub T);

/// Result allows to make data guard "infallible" and pass guard result to a protected request
/// handler.

/// Then request handler can use '?' operator to unwrap and return
/// OperationError type.
///
/// When using "fallible" inner data guard it's Outcome result is handled by default error catchers
/// that are meant to be operation independent and provide basic general output.
///
/// OperationError, returned by the handler, explicitly documents type of the error
/// in OpenAPI schema.
pub type ValidatedResult<T> = Result<Validated<T>, OperationError>;

impl<'r, T: Deserialize<'r> + validator::Validate> Validated<T> {
    pub fn into_inner(self) -> T {
        self.0
    }

    fn from_str(s: &'r str) -> Result<Self, OperationError> {
        let deserialized: T = serde_json::from_str(s).map_err(|e| JsonError::Parse(s, e))?;
        deserialized.validate()?;
        Ok(Self(deserialized))
    }

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Result<Self, OperationError> {
        // based on Json
        let limit = req.limits().get("json").unwrap_or(Limits::JSON);
        let string = match data.open(limit).into_string().await {
            Ok(s) if s.is_complete() => s.into_inner(),
            Ok(_) => {
                let eof = std::io::ErrorKind::UnexpectedEof;
                return Err(OperationError::from(std::io::Error::new(
                    eof,
                    "data limit exceeded",
                )));
            }
            Err(e) => return Err(OperationError::from(e)),
        };

        Self::from_str(local_cache!(req, string))
    }
}

#[rocket::async_trait]
impl<'r, T: Deserialize<'r> + validator::Validate> FromData<'r> for Validated<T> {
    type Error = OperationError;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        match Self::from_data(req, data).await {
            Ok(value) => Outcome::Success(value),
            Err(e) => Outcome::Failure((e.http_status(), e)),
        }
    }
}

impl<'r, T: JsonSchema + Deserialize<'r> + validator::Validate> OpenApiFromData<'r> for Validated<T> {
    fn request_body(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<RequestBody> {
        let schema = gen.json_schema::<T>();
        Ok(RequestBody {
            content: {
                let mut map = Map::new();
                let _ = map.insert(
                    "application/json".to_owned(),
                    MediaType {
                        schema: Some(schema),
                        ..MediaType::default()
                    },
                );
                map
            },
            required: true,
            ..RequestBody::default()
        })
    }
}

impl<T> From<T> for Validated<T> {
    fn from(value: T) -> Self {
        Validated(value)
    }
}

impl<T> Deref for Validated<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.0
    }
}
