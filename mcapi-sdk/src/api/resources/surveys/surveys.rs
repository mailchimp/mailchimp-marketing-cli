use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SurveysClient {
    pub http_client: HttpClient,
}

impl SurveysClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Utilize the List ID and Survey ID to generate a Campaign that links to your survey.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `survey_id` - The ID of the survey.
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
    ///         .surveys
    ///         .create_list_survey_action_create_email(
    ///             &"list_id".to_string(),
    ///             &"survey_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_list_survey_action_create_email(
        &self,
        list_id: &str,
        survey_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Campaign, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/lists/{}/surveys/{}/actions/create-email",
                    list_id, survey_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Publish a survey that is in draft, unpublished, or has been previously published and edited.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `survey_id` - The ID of the survey.
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
    ///         .surveys
    ///         .create_list_survey_action_publish(&"list_id".to_string(), &"survey_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn create_list_survey_action_publish(
        &self,
        list_id: &str,
        survey_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/lists/{}/surveys/{}/actions/publish",
                    list_id, survey_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Unpublish a survey that has been published.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `survey_id` - The ID of the survey.
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
    ///         .surveys
    ///         .create_list_survey_action_unpublish(&"list_id".to_string(), &"survey_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn create_list_survey_action_unpublish(
        &self,
        list_id: &str,
        survey_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/lists/{}/surveys/{}/actions/unpublish",
                    list_id, survey_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
