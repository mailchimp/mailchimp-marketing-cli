use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct LandingPagesClient {
    pub http_client: HttpClient,
}

impl LandingPagesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get all landing pages.
    ///
    /// # Arguments
    ///
    /// * `sort_dir` - Determines the order direction for sorted results.
    /// * `sort_field` - Returns files sorted by the specified field.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
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
    ///         .landing_pages
    ///         .list(
    ///             &LandingPagesListQueryRequest {
    ///                 sort_dir: None,
    ///                 sort_field: None,
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &LandingPagesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListLandingPagesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/landing-pages",
                None,
                QueryBuilder::new()
                    .serialize("sort_dir", request.sort_dir.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create an unpublished and contentless Mailchimp landing page.
    ///
    /// # Arguments
    ///
    /// * `use_default_list` - Will create the Landing Page using the account's Default List instead of requiring a list_id.
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
    ///         .landing_pages
    ///         .create(
    ///             &CreateLandingPagesRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateLandingPagesRequest,
        options: Option<RequestOptions>,
    ) -> Result<LandingPage, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "3.0/landing-pages",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool("use_default_list", request.use_default_list.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get information about a specific page.
    ///
    /// # Arguments
    ///
    /// * `page_id` - The unique id for the page.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
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
    ///         .landing_pages
    ///         .get(
    ///             &"page_id".to_string(),
    ///             &LandingPagesGetQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        page_id: &str,
        request: &LandingPagesGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<LandingPage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/landing-pages/{}", page_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a landing page.
    ///
    /// # Arguments
    ///
    /// * `page_id` - The unique id for the page.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .landing_pages
    ///         .delete(&"page_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        page_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/landing-pages/{}", page_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a landing page.
    ///
    /// # Arguments
    ///
    /// * `page_id` - The unique id for the page.
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
    ///         .landing_pages
    ///         .update(
    ///             &"page_id".to_string(),
    ///             &UpdateLandingPagesRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        page_id: &str,
        request: &UpdateLandingPagesRequest,
        options: Option<RequestOptions>,
    ) -> Result<LandingPage, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/landing-pages/{}", page_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Publish a landing page that is in draft, unpublished, or has been previously published and edited.
    ///
    /// # Arguments
    ///
    /// * `page_id` - The unique id for the page.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .landing_pages
    ///         .create_action_publish(&"page_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_publish(
        &self,
        page_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/landing-pages/{}/actions/publish", page_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Unpublish a landing page that is in draft or has been published.
    ///
    /// # Arguments
    ///
    /// * `page_id` - The unique id for the page.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .landing_pages
    ///         .create_action_unpublish(&"page_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_unpublish(
        &self,
        page_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/landing-pages/{}/actions/unpublish", page_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get the the HTML for your landing page.
    ///
    /// # Arguments
    ///
    /// * `page_id` - The unique id for the page.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
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
    ///         .landing_pages
    ///         .list_content(
    ///             &"page_id".to_string(),
    ///             &ListContentQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_content(
        &self,
        page_id: &str,
        request: &ListContentQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListContentLandingPagesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/landing-pages/{}/content", page_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }
}
