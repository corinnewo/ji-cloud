//! Types for additional resources for JIGs.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Wrapper type around [`Uuid`](Uuid), represents the ID of an additional resource.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct AdditionalResourceId(pub Uuid);

#[derive(Serialize, Deserialize, Debug)]
/// Over-the-wire representation of a JIG additional resource.
pub struct AdditionalResource {
    /// The additional resources's ID.
    pub id: AdditionalResourceId,

    /// The URL of the additional resource.
    /// Stored as a `String`.
    pub url: String,
}

/// Request to create a new `AdditionalResource`.
///
/// [`additional_resource::Create`](crate::api::endpoints::jig::additional_resource::Create)
#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalResourceCreateRequest {
    /// The URL of the additional resource.
    /// Stored as a `String`.
    pub url: String,
}

/// Response for successfully requesting an additional resource.
#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalResourceResponse {
    /// The additional resource found.
    pub url: String,
}

into_uuid![AdditionalResourceId];
