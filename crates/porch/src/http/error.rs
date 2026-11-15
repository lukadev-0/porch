use std::{borrow::Cow, sync::Arc};

use aide::OperationOutput;
use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use schemars::JsonSchema;
use serde::Serialize;

use crate::service::{ServiceError, ServiceResult};

pub struct ApiError {
    status: StatusCode,
    message: Cow<'static, str>,
    source: Option<anyhow::Error>,
}

pub type ApiResult<T> = Result<T, ApiError>;

impl ApiError {
    pub fn new(status: StatusCode, message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            status,
            message: message.into(),
            source: None,
        }
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(value: anyhow::Error) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: Cow::Borrowed("An internal error occurred."),
            source: Some(value),
        }
    }
}

pub trait IntoApiError<E> {
    type Output;
    fn into_api_error(self, f: impl FnOnce(E) -> ApiError) -> Self::Output;
}

impl<E> IntoApiError<E> for ServiceError<E> {
    type Output = ApiError;
    fn into_api_error(self, f: impl FnOnce(E) -> ApiError) -> Self::Output {
        match self {
            ServiceError::AccessDenied => ApiError::new(
                StatusCode::FORBIDDEN,
                "Insufficient permissions to perform this action.",
            ),
            ServiceError::NoExist => {
                ApiError::new(StatusCode::NOT_FOUND, "The target resource does not exist.")
            }
            ServiceError::Specific(error) => f(error),
            ServiceError::Other(error) => error.into(),
        }
    }
}

impl<T, E> IntoApiError<E> for ServiceResult<T, E> {
    type Output = ApiResult<T>;
    fn into_api_error(self, f: impl FnOnce(E) -> ApiError) -> Self::Output {
        self.map_err(|error| error.into_api_error(f))
    }
}

impl From<ServiceError> for ApiError {
    fn from(value: ServiceError) -> Self {
        value.into_api_error(|_| unreachable!())
    }
}

#[derive(Serialize, JsonSchema)]
pub struct ApiErrorResponse<'a> {
    message: &'a str,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let mut response = (
            self.status,
            Json(ApiErrorResponse {
                message: &self.message,
            }),
        )
            .into_response();

        if let Some(source) = self.source {
            response.extensions_mut().insert(Arc::new(source));
        }

        response
    }
}

impl OperationOutput for ApiError {
    type Inner = ApiErrorResponse<'static>;

    fn inferred_responses(
        ctx: &mut aide::generate::GenContext,
        operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<u16>, aide::openapi::Response)> {
        vec![(
            Some(500),
            <Json<ApiErrorResponse>>::operation_response(ctx, operation).unwrap(),
        )]
    }
}
