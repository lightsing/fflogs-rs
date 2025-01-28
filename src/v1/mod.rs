//! FFLogs V1 API client.
use compact_str::CompactString;
use reqwest::ClientBuilder;
use std::borrow::Cow;
use std::sync::Arc;

pub mod apis;

/// A client for the FFLogs V1 API.
#[derive(Debug, Clone)]
pub struct FFLogsV1Client {
    pub(crate) inner: Arc<FFLogsV1ClientInner>,
}

#[derive(Debug, Clone)]
pub(crate) struct FFLogsV1ClientInner {
    pub(crate) client: reqwest::Client,
    pub(crate) base_url: Cow<'static, str>,
    pub(crate) api_key: Cow<'static, str>,
}

/// A builder for a `FFLogsV1Client`.
#[derive(Debug)]
#[must_use]
pub struct FFLogsV1ClientBuilder<A> {
    builder: ClientBuilder,
    base_url: Cow<'static, str>,
    api_key: A,
}

impl<A> FFLogsV1ClientBuilder<A> {
    fn new() -> FFLogsV1ClientBuilder<()> {
        FFLogsV1ClientBuilder {
            builder: reqwest::Client::builder()
                .https_only(true)
                .user_agent("FFLogs V1 API Client (github.com/lightsing/fflogs)"),
            base_url: Cow::Borrowed("https://www.fflogs.com/v1"),
            api_key: (),
        }
    }

    /// Sets the base URL for the FFLogs API. Trailing slashes are not required.
    pub fn base_url(mut self, base_url: impl Into<Cow<'static, str>>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Sets the API key to use for requests.
    pub fn api_key<A1: Into<Cow<'static, str>>>(
        self,
        api_key: A1,
    ) -> FFLogsV1ClientBuilder<Cow<'static, str>> {
        FFLogsV1ClientBuilder {
            builder: self.builder,
            base_url: self.base_url,
            api_key: api_key.into(),
        }
    }
}

impl FFLogsV1ClientBuilder<Cow<'static, str>> {
    /// Returns a `FFLogsV1Client`.
    ///
    /// # Errors
    ///
    /// This method fails if a TLS backend cannot be initialized, or the resolver
    /// cannot load the system configuration.
    pub fn build(self) -> crate::Result<FFLogsV1Client> {
        Ok(FFLogsV1Client {
            inner: Arc::new(FFLogsV1ClientInner {
                client: self.builder.build().map_err(crate::Error::builder)?,
                base_url: self.base_url,
                api_key: self.api_key,
            }),
        })
    }
}

impl FFLogsV1Client {
    /// Creates a `FFLogsV1ClientBuilder` to configure a `FFLogsV1Client`.
    pub fn builder() -> FFLogsV1ClientBuilder<()> {
        FFLogsV1ClientBuilder::<()>::new()
    }

    /// Gets an array of Zone objects.
    ///
    /// Each zone corresponds to a raid/dungeon instance in the game and has its own set of
    /// encounters.
    #[must_use]
    pub fn zones(&self) -> apis::zones::Request {
        apis::zones::Request::new(self.clone())
    }

    /// Gets an array of Class objects.
    ///
    /// Each Class corresponds to a class in the game.
    #[must_use]
    pub fn classes(&self) -> apis::classes::Request {
        apis::classes::Request::new(self.clone())
    }

    /// Gets an object that contains a total count and an array of EncounterRanking objects
    /// and a total number of rankings for that encounter.
    ///
    /// Each EncounterRanking corresponds to a single character or guild/team.
    #[must_use]
    pub fn rankings_by_encounter(&self, encounter_id: u64) -> apis::rankings_by_encounter::Request {
        apis::rankings_by_encounter::Request::new(self.clone(), encounter_id)
    }

    /// Gets a set of events based off the view you're asking for.
    ///
    /// This exactly corresponds to the Events view on the site.
    #[must_use]
    pub fn report_events_by_code(
        &self,
        view: apis::common::DataType,
        code: impl Into<CompactString>,
    ) -> apis::report_events_by_code::Request {
        apis::report_events_by_code::Request::new(self.clone(), view, code)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::v1::apis::{ApiRequest, common::DataType};
    use std::sync::LazyLock;
    use strum::IntoEnumIterator;

    static CLIENT: LazyLock<FFLogsV1Client> = LazyLock::new(|| {
        FFLogsV1Client::builder()
            .api_key(env!("FFLOGS_API_KEY"))
            .build()
            .expect("Failed to build client")
    });

    #[tokio::test]
    async fn test_zones() {
        CLIENT
            .zones()
            .execute()
            .await
            .expect("Failed to execute request");
    }

    #[tokio::test]
    async fn test_classes() {
        CLIENT
            .classes()
            .execute()
            .await
            .expect("Failed to execute request");
    }

    #[tokio::test]
    async fn test_rankings_by_encounter() {
        CLIENT
            .rankings_by_encounter(93) // M1S Black Cat
            .region("CN")
            .execute()
            .await
            .expect("Failed to execute request");
    }

    #[tokio::test]
    async fn test_get_report_events_by_code() {
        let req = CLIENT
            .report_events_by_code(DataType::Casts, "HfrpNycxX2FvPKjW")
            .start(1265340)
            .end(1808067)
            .hostility(true)
            .source_id(29);

        for view in DataType::iter() {
            let _ = req
                .clone()
                .view(view)
                .execute()
                .await
                .map_err(|e| panic!("Failed to execute request for {view}: {e:?}"));
        }
    }
}
