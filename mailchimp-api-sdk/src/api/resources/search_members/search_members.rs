use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SearchMembersClient {
    pub http_client: HttpClient,
}

impl SearchMembersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Search for list members. This search can be restricted to a specific list, or can be used to search across all lists in an account.
    ///
    /// # Arguments
    ///
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `query` - The search query used to filter results. Query should be a valid email, or a string representing a contact's first or last name.
    /// * `list_id` - The unique id for the list.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .search_members
    ///         .list(
    ///             &SearchMembersListQueryRequest {
    ///                 query: "query".to_string(),
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 list_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &SearchMembersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSearchMembersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/search-members",
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .structured_query("query", request.query.clone())
                    .string("list_id", request.list_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
