//! Contains the api types for the FFLogs API v1.
use crate::v1::FFLogsV1Client;
use reqwest::header;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub mod classes;
pub mod common;
pub mod rankings_by_encounter;
pub mod report_events_by_code;
pub mod report_fights_by_code;
pub mod report_tables_by_code;
pub mod zones;

/// Request trait
pub trait ApiRequest {
    /// The response type of this endpoint
    type Response: DeserializeOwned;

    #[doc(hidden)]
    type Path<'a>: Display
    where
        Self: 'a;

    #[doc(hidden)]
    type Query: Serialize;

    /// Returns the client to use for the request.
    #[doc(hidden)]
    fn client(&self) -> &FFLogsV1Client;

    /// Formats the path of the request.
    #[doc(hidden)]
    fn path(&self) -> Self::Path<'_>;

    /// Returns the query to use for the request.
    #[doc(hidden)]
    fn query(&self) -> Option<&Self::Query> {
        None
    }

    /// Execute the request and deserialize the response
    fn execute(&self) -> impl Future<Output = crate::Result<Self::Response>> + Send + Sync {
        let client = self.client().inner.as_ref();
        let mut url = format!(
            "{base_url}{path}?api_key={api_key}",
            base_url = client.base_url,
            path = self.path(),
            api_key = client.api_key,
        );
        if let Some(query) = self.query() {
            let qs = serde_qs::to_string(query)
                .expect("Failed to serialize query parameters, this is a bug");
            url.push_str("&");
            url.push_str(qs.as_str());
        }

        async {
            let text = client
                .client
                .get(url)
                .header(header::ACCEPT, "application/json")
                .send()
                .await
                .map_err(crate::Error::request)?
                .text()
                .await
                .map_err(crate::Error::request)?;

            #[derive(Debug, Deserialize)]
            #[serde(untagged)]
            enum ApiResponse<T> {
                Err(crate::ApiError),
                Ok(T),
            }

            impl<T> From<ApiResponse<T>> for Result<T, crate::ApiError> {
                fn from(res: ApiResponse<T>) -> Self {
                    match res {
                        ApiResponse::Ok(t) => Ok(t),
                        ApiResponse::Err(e) => Err(e),
                    }
                }
            }

            let jd = &mut serde_json::Deserializer::from_str(text.as_str());
            let res: ApiResponse<Self::Response> =
                serde_path_to_error::deserialize(jd).map_err(crate::Error::deserialize)?;
            Ok(Result::from(res)?)
        }
    }
}
