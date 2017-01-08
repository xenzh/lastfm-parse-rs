use std::error::Error as StdError;
use serde::de::{Deserialize, Deserializer, Visitor};

#[derive(Debug)]
pub enum ApiErrorKind {
    InvalidService,
    InvalidMethod,
    AuthenticationFailed,
    InvalidFormat,
    InvalidParameters,
    InvalidResource,
    OperationFailed,
    InvalidSessionKey,
    InvalidApiKey,
    ServiceOffline,
    SubscribersOnly,
    InvalidMethodSignature,
    UnauthorizedToken,
    ItemNotAvailableForStreaming,
    ServiceTemporaryUnavailable,
    LoginRequired,
    TrialExpired,
    NotEnoughContent,
    NotEnoughMembers,
    NotEnoughFans,
    NotEnoughNeighbours,
    NoPeakRadio,
    RadioNotFound,
    ApiKeySuspended,
    Deprecated,
    RateLimitExceeded,
    Unknown,
}

impl ApiErrorKind {
    fn from_u64(value: u64) -> ApiErrorKind {
        match value {
            02 => ApiErrorKind::InvalidService,
            03 => ApiErrorKind::InvalidMethod,
            04 => ApiErrorKind::AuthenticationFailed,
            05 => ApiErrorKind::InvalidFormat,
            06 => ApiErrorKind::InvalidParameters,
            07 => ApiErrorKind::InvalidResource,
            08 => ApiErrorKind::OperationFailed,
            09 => ApiErrorKind::InvalidSessionKey,
            10 => ApiErrorKind::InvalidApiKey,
            11 => ApiErrorKind::ServiceOffline,
            12 => ApiErrorKind::SubscribersOnly,
            13 => ApiErrorKind::InvalidMethodSignature,
            14 => ApiErrorKind::UnauthorizedToken,
            15 => ApiErrorKind::ItemNotAvailableForStreaming,
            16 => ApiErrorKind::ServiceTemporaryUnavailable,
            17 => ApiErrorKind::LoginRequired,
            18 => ApiErrorKind::TrialExpired,
            20 => ApiErrorKind::NotEnoughContent,
            21 => ApiErrorKind::NotEnoughMembers,
            22 => ApiErrorKind::NotEnoughFans,
            23 => ApiErrorKind::NotEnoughNeighbours,
            24 => ApiErrorKind::NoPeakRadio,
            25 => ApiErrorKind::RadioNotFound,
            26 => ApiErrorKind::ApiKeySuspended,
            27 => ApiErrorKind::Deprecated,
            29 => ApiErrorKind::RateLimitExceeded,
            _ => ApiErrorKind::Unknown,
        }
    }

    pub fn description(&self) -> &'static str {
        match *self {
            ApiErrorKind::InvalidService               => "This service does not exist",
            ApiErrorKind::InvalidMethod                => "No method with that name in the package",
            ApiErrorKind::AuthenticationFailed         => "You do not have permissions to access the service",
            ApiErrorKind::InvalidFormat                => "This service doesn't exist in that format",
            ApiErrorKind::InvalidParameters            => "Your request is missing a required parameter",
            ApiErrorKind::InvalidResource              => "Invalid resource specified",
            ApiErrorKind::OperationFailed              => "Most likely the backend service failed. Please try again.",
            ApiErrorKind::InvalidSessionKey            => "Please re-authenticate",
            ApiErrorKind::InvalidApiKey                => "You must be granted a valid key by last.fm",
            ApiErrorKind::ServiceOffline               => "This service is temporarily offline. Try again later.",
            ApiErrorKind::SubscribersOnly              => "This station is only available to paid last.fm subscribers",
            ApiErrorKind::InvalidMethodSignature       => "Invalid method signature supplied",
            ApiErrorKind::UnauthorizedToken            => "This token has not been authorized",
            ApiErrorKind::ItemNotAvailableForStreaming => "This item is not available for streaming",
            ApiErrorKind::ServiceTemporaryUnavailable  => "The service is temporarily unavailable, please try again",
            ApiErrorKind::LoginRequired                => "User requires to be logged in",
            ApiErrorKind::TrialExpired                 => "This user has no free radio plays left. Subscription required.",
            ApiErrorKind::NotEnoughContent             => "There is not enough content to play this station",
            ApiErrorKind::NotEnoughMembers             => "This group does not have enough members for radio",
            ApiErrorKind::NotEnoughFans                => "This artist does not have enough fans for for radio",
            ApiErrorKind::NotEnoughNeighbours          => "There are not enough neighbours for radio",
            ApiErrorKind::NoPeakRadio                  => "This user is not allowed to listen to radio during peak usage",
            ApiErrorKind::RadioNotFound                => "Radio station not found",
            ApiErrorKind::ApiKeySuspended              => "This application is not allowed to make requests to the web services",
            ApiErrorKind::Deprecated                   => "This type of request is no longer supported",
            ApiErrorKind::RateLimitExceeded            => "Your IP has made too many requests in a short period, exceeding our API guidelines",
            _                                          => "This error code is not covered by official API reference",
        }
    }
}

impl Deserialize for ApiErrorKind {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        struct ErrorKindVisitor;
        impl Visitor for ErrorKindVisitor {
            type Value = ApiErrorKind;
            fn visit_u64<E>(&mut self, v: u64) -> Result<Self::Value, E>
                where E: StdError
            {
                let kind = ApiErrorKind::from_u64(v);
                Ok(kind)
            }
        }
        deserializer.deserialize_u64(ErrorKindVisitor)
    }
}

#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub error: ApiErrorKind,
    pub message: String,
    // links omitted for now
}