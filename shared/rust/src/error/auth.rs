//! Errors for Auth routes.

#[cfg(feature = "backend")]
use super::anyhow_to_ise;
#[cfg(feature = "backend")]
use actix_web::HttpResponse;
#[cfg(feature = "backend")]
use paperclip::actix::api_v2_errors;
use serde::{Deserialize, Serialize};

/// An error occured during registration
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterError {
    /// Which kind of error occurred.
    pub kind: RegisterErrorKind,
}

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
/// Represents the kinds of errors that can occur during registration.
pub enum RegisterErrorKind {
    /// No username was provided.
    EmptyDisplayName,

    /// Another user with the provided email already exists.
    TakenEmail,

    /// Another user with the provided firebase-id already exists.
    TakenId,

    /// Another user with the provided username already exists.
    TakenUsername,
}

#[non_exhaustive]
// todo: fill in descriptions for 401
#[cfg_attr(feature = "backend", api_v2_errors(code = 401, code = 500,))]
#[derive(Serialize, Deserialize)]
/// Represents an error with when authorizing a firebase token.
pub enum FirebaseError {
    /// The `Authorization` header didn't exist or didn't start with `Bearer` (case-insensitive).
    MissingBearerToken,

    /// The provided JWT token was invalid.
    InvalidToken,

    /// An internal server error occurred.
    #[serde(skip)]
    InternalServerError(anyhow::Error),
}

#[cfg(feature = "backend")]
impl From<FirebaseError> for actix_web::Error {
    fn from(e: FirebaseError) -> Self {
        match e {
            FirebaseError::InternalServerError(e) => anyhow_to_ise(e),
            e => HttpResponse::Unauthorized().json(e).into(),
        }
    }
}

from_anyhow![FirebaseError];
