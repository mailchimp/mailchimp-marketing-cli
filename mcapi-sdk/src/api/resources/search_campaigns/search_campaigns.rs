use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SearchCampaignsClient {
    pub http_client: HttpClient,
}

impl SearchCampaignsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Search all campaigns for the specified query terms.
    ///
    /// # Arguments
    ///
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `query` - The search query used to filter results.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .search_campaigns
    ///         .list(
    ///             &SearchCampaignsListQueryRequest {
    ///                 query: "query".to_string(),
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &SearchCampaignsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSearchCampaignsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/search-campaigns",
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .structured_query("query", request.query.clone())
                    .build(),
                options,
            )
            .await
    }
}
