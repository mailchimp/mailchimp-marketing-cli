use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct AutomationsClient {
    pub http_client: HttpClient,
}

impl AutomationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get a summary of an account's classic automations.
    ///
    /// # Arguments
    ///
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `before_create_time` - Restrict the response to automations created before this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `since_create_time` - Restrict the response to automations created after this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `before_start_time` - Restrict the response to automations started before this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `since_start_time` - Restrict the response to automations started after this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `status` - Restrict the results to automations with the specified status.
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
    ///         .automations
    ///         .list(
    ///             &AutomationsListQueryRequest {
    ///                 count: None,
    ///                 offset: None,
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 before_create_time: None,
    ///                 since_create_time: None,
    ///                 before_start_time: None,
    ///                 since_start_time: None,
    ///                 status: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AutomationsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAutomationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/automations",
                None,
                QueryBuilder::new()
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .datetime("before_create_time", request.before_create_time.clone())
                    .datetime("since_create_time", request.since_create_time.clone())
                    .datetime("before_start_time", request.before_start_time.clone())
                    .datetime("since_start_time", request.since_start_time.clone())
                    .serialize("status", request.status.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &AutomationsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .int("count", request.count.clone())
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .datetime("before_create_time", request.before_create_time.clone())
            .datetime("since_create_time", request.since_create_time.clone())
            .datetime("before_start_time", request.before_start_time.clone())
            .datetime("since_start_time", request.since_start_time.clone())
            .serialize("status", request.status.clone())
            .build();
        let options_clone = options.clone();

        AsyncPaginator::new(
            http_client,
            move |client, page_token| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();

                // Use page_token as offset/page number (start from 0 if None)
                let current_page = page_token.unwrap_or_else(|| "0".to_string());
                query_params.push(("offset".to_string(), current_page.clone()));

                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            "3.0/automations",
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("automations")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let has_next_page = !items.is_empty();
                    let next_cursor: Option<String> = if has_next_page {
                        let current_offset: i64 = current_page.parse().unwrap_or(0);
                        Some((current_offset + 1).to_string())
                    } else {
                        None
                    };

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with page 0
        )
    }

    /// Create a new classic automation in your Mailchimp account.
    ///
    /// # Arguments
    ///
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
    ///         .automations
    ///         .create(
    ///             &CreateAutomationsRequest {
    ///                 recipients: CreateAutomationsRequestRecipients {
    ///                     ..Default::default()
    ///                 },
    ///                 trigger_settings: CreateAutomationsRequestTriggerSettings {
    ///                     workflow_type:
    ///                         CreateAutomationsRequestTriggerSettingsWorkflowType::AbandonedBrowse,
    ///                 },
    ///                 settings: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateAutomationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AutomationWorkflow, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "3.0/automations",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a summary of an individual classic automation workflow's settings and content. The `trigger_settings` object returns information for the first email in the workflow.
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The unique id for the Automation workflow.
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
    ///         .automations
    ///         .get(
    ///             &"workflow_id".to_string(),
    ///             &AutomationsGetQueryRequest {
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
        workflow_id: &str,
        request: &AutomationsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AutomationWorkflow, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/automations/{}", workflow_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Archiving will permanently end your automation and keep the report data. You’ll be able to replicate your archived automation, but you can’t restart it.
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The unique id for the Automation workflow.
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
    ///         .automations
    ///         .create_action_archive(&"workflow_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_archive(
        &self,
        workflow_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/automations/{}/actions/archive", workflow_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Pause all emails in a specific classic automation workflow.
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The unique id for the Automation workflow.
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
    ///         .automations
    ///         .create_action_pause_all_email(&"workflow_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_pause_all_email(
        &self,
        workflow_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/automations/{}/actions/pause-all-emails", workflow_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Start all emails in a classic automation workflow.
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The unique id for the Automation workflow.
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
    ///         .automations
    ///         .create_action_start_all_email(&"workflow_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_start_all_email(
        &self,
        workflow_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/automations/{}/actions/start-all-emails", workflow_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get a summary of the emails in a classic automation workflow.
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The unique id for the Automation workflow.
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
    ///         .automations
    ///         .list_emails(&"workflow_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_emails(
        &self,
        workflow_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListEmailsAutomationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/automations/{}/emails", workflow_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get information about an individual classic automation workflow email.
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The unique id for the Automation workflow.
    /// * `workflow_email_id` - The unique id for the Automation workflow email.
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
    ///         .automations
    ///         .get_email(
    ///             &"workflow_id".to_string(),
    ///             &"workflow_email_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_email(
        &self,
        workflow_id: &str,
        workflow_email_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AutomationWorkflowEmail, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/automations/{}/emails/{}",
                    workflow_id, workflow_email_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Removes an individual classic automation workflow email. Emails from certain workflow types, including the Abandoned Cart Email (abandonedCart) and Product Retargeting Email (abandonedBrowse) Workflows, cannot be deleted.
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The unique id for the Automation workflow.
    /// * `workflow_email_id` - The unique id for the Automation workflow email.
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
    ///         .automations
    ///         .delete_email(
    ///             &"workflow_id".to_string(),
    ///             &"workflow_email_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_email(
        &self,
        workflow_id: &str,
        workflow_email_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/automations/{}/emails/{}",
                    workflow_id, workflow_email_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update settings for a classic automation workflow email.  Only works with workflows of type: abandonedBrowse, abandonedCart, emailFollowup, or singleWelcome.
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The unique id for the Automation workflow.
    /// * `workflow_email_id` - The unique id for the Automation workflow email.
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
    ///         .automations
    ///         .update_email(
    ///             &"workflow_id".to_string(),
    ///             &"workflow_email_id".to_string(),
    ///             &UpdateEmailAutomationsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_email(
        &self,
        workflow_id: &str,
        workflow_email_id: &str,
        request: &UpdateEmailAutomationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AutomationWorkflowEmail, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/automations/{}/emails/{}",
                    workflow_id, workflow_email_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Pause an automated email.
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The unique id for the Automation workflow.
    /// * `workflow_email_id` - The unique id for the Automation workflow email.
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
    ///         .automations
    ///         .create_email_action_pause(
    ///             &"workflow_id".to_string(),
    ///             &"workflow_email_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_email_action_pause(
        &self,
        workflow_id: &str,
        workflow_email_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/automations/{}/emails/{}/actions/pause",
                    workflow_id, workflow_email_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Start an automated email.
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The unique id for the Automation workflow.
    /// * `workflow_email_id` - The unique id for the Automation workflow email.
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
    ///         .automations
    ///         .create_email_action_start(
    ///             &"workflow_id".to_string(),
    ///             &"workflow_email_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_email_action_start(
        &self,
        workflow_id: &str,
        workflow_email_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/automations/{}/emails/{}/actions/start",
                    workflow_id, workflow_email_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Get information about a classic automation email queue.
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The unique id for the Automation workflow.
    /// * `workflow_email_id` - The unique id for the Automation workflow email.
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
    ///         .automations
    ///         .list_email_queue(
    ///             &"workflow_id".to_string(),
    ///             &"workflow_email_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_email_queue(
        &self,
        workflow_id: &str,
        workflow_email_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListEmailQueueAutomationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/automations/{}/emails/{}/queue",
                    workflow_id, workflow_email_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Manually add a subscriber to a workflow, bypassing the default trigger settings. You can also use this endpoint to trigger a series of automated emails in an API 3.0 workflow type.
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The unique id for the Automation workflow.
    /// * `workflow_email_id` - The unique id for the Automation workflow email.
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
    ///         .automations
    ///         .create_email_queue(
    ///             &"workflow_id".to_string(),
    ///             &"workflow_email_id".to_string(),
    ///             &CreateEmailQueueAutomationsRequest {
    ///                 email_address: "email_address".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_email_queue(
        &self,
        workflow_id: &str,
        workflow_email_id: &str,
        request: &CreateEmailQueueAutomationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubscriberInAutomationQueue, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/automations/{}/emails/{}/queue",
                    workflow_id, workflow_email_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific subscriber in a classic automation email queue.
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The unique id for the Automation workflow.
    /// * `workflow_email_id` - The unique id for the Automation workflow email.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address.
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
    ///         .automations
    ///         .get_email_queue(
    ///             &"workflow_id".to_string(),
    ///             &"workflow_email_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_email_queue(
        &self,
        workflow_id: &str,
        workflow_email_id: &str,
        subscriber_hash: &str,
        options: Option<RequestOptions>,
    ) -> Result<SubscriberInAutomationQueue, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/automations/{}/emails/{}/queue/{}",
                    workflow_id, workflow_email_id, subscriber_hash
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Get information about subscribers who were removed from a classic automation workflow.
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The unique id for the Automation workflow.
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
    ///         .automations
    ///         .list_removed_subscribers(&"workflow_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_removed_subscribers(
        &self,
        workflow_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListRemovedSubscribersAutomationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/automations/{}/removed-subscribers", workflow_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Remove a subscriber from a specific classic automation workflow. You can remove a subscriber at any point in an automation workflow, regardless of how many emails they've been sent from that workflow. Once they're removed, they can never be added back to the same workflow.
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The unique id for the Automation workflow.
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
    ///         .automations
    ///         .create_removed_subscriber(
    ///             &"workflow_id".to_string(),
    ///             &CreateRemovedSubscriberAutomationsRequest {
    ///                 email_address: "email_address".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_removed_subscriber(
        &self,
        workflow_id: &str,
        request: &CreateRemovedSubscriberAutomationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubscriberRemovedFromAutomationWorkflow, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/automations/{}/removed-subscribers", workflow_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific subscriber who was removed from a classic automation workflow.
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The unique id for the Automation workflow.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address.
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
    ///         .automations
    ///         .get_removed_subscriber(
    ///             &"workflow_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_removed_subscriber(
        &self,
        workflow_id: &str,
        subscriber_hash: &str,
        options: Option<RequestOptions>,
    ) -> Result<SubscriberRemovedFromAutomationWorkflow, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/automations/{}/removed-subscribers/{}",
                    workflow_id, subscriber_hash
                ),
                None,
                None,
                options,
            )
            .await
    }
}
