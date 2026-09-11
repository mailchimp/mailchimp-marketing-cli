use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct CampaignsClient {
    pub http_client: HttpClient,
}

impl CampaignsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get all campaigns in an account.
    ///
    /// # Arguments
    ///
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `type_` - The campaign type.
    /// * `status` - The status of the campaign.
    /// * `before_send_time` - Restrict the response to campaigns sent before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `since_send_time` - Restrict the response to campaigns sent after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `before_create_time` - Restrict the response to campaigns created before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `since_create_time` - Restrict the response to campaigns created after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `list_id` - The unique id for the list.
    /// * `folder_id` - The unique folder id.
    /// * `member_id` - Retrieve campaigns sent to a particular list member. Member ID is The MD5 hash of the lowercase version of the list member’s email address.
    /// * `sort_field` - Returns files sorted by the specified field.
    /// * `sort_dir` - Determines the order direction for sorted results.
    /// * `include_resend_shortcut_eligibility` - Return the `resend_shortcut_eligibility` field in the response, which tells you if the campaign is eligible for the various Campaign Resend Shortcuts offered.
    /// * `include_resend_shortcut_usage` - Return the `resend_shortcut_usage` field in the response.  This includes information about campaigns related by a shortcut.
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
    ///         .campaigns
    ///         .list(
    ///             &CampaignsListQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 r#type: None,
    ///                 status: None,
    ///                 before_send_time: None,
    ///                 since_send_time: None,
    ///                 before_create_time: None,
    ///                 since_create_time: None,
    ///                 list_id: None,
    ///                 folder_id: None,
    ///                 member_id: None,
    ///                 sort_field: None,
    ///                 sort_dir: None,
    ///                 include_resend_shortcut_eligibility: None,
    ///                 include_resend_shortcut_usage: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &CampaignsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCampaignsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/campaigns",
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .serialize("type", request.r#type.clone())
                    .serialize("status", request.status.clone())
                    .datetime("before_send_time", request.before_send_time.clone())
                    .datetime("since_send_time", request.since_send_time.clone())
                    .datetime("before_create_time", request.before_create_time.clone())
                    .datetime("since_create_time", request.since_create_time.clone())
                    .string("list_id", request.list_id.clone())
                    .string("folder_id", request.folder_id.clone())
                    .string("member_id", request.member_id.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .bool(
                        "include_resend_shortcut_eligibility",
                        request.include_resend_shortcut_eligibility.clone(),
                    )
                    .bool(
                        "include_resend_shortcut_usage",
                        request.include_resend_shortcut_usage.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &CampaignsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string_array("fields", request.fields.clone())
            .string_array("exclude_fields", request.exclude_fields.clone())
            .int("count", request.count.clone())
            .serialize("type", request.r#type.clone())
            .serialize("status", request.status.clone())
            .datetime("before_send_time", request.before_send_time.clone())
            .datetime("since_send_time", request.since_send_time.clone())
            .datetime("before_create_time", request.before_create_time.clone())
            .datetime("since_create_time", request.since_create_time.clone())
            .string("list_id", request.list_id.clone())
            .string("folder_id", request.folder_id.clone())
            .string("member_id", request.member_id.clone())
            .serialize("sort_field", request.sort_field.clone())
            .serialize("sort_dir", request.sort_dir.clone())
            .bool(
                "include_resend_shortcut_eligibility",
                request.include_resend_shortcut_eligibility.clone(),
            )
            .bool(
                "include_resend_shortcut_usage",
                request.include_resend_shortcut_usage.clone(),
            )
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
                            "3.0/campaigns",
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("campaigns")
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

    /// Create a new Mailchimp campaign.
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
    ///         .campaigns
    ///         .create(
    ///             &CreateCampaignsRequest {
    ///                 r#type: CreateCampaignsRequestType::Regular,
    ///                 content_type: None,
    ///                 recipients: None,
    ///                 rss_opts: None,
    ///                 settings: None,
    ///                 social_card: None,
    ///                 tracking: None,
    ///                 variate_settings: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateCampaignsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Campaign, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "3.0/campaigns",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `include_resend_shortcut_eligibility` - Return the `resend_shortcut_eligibility` field in the response, which tells you if the campaign is eligible for the various Campaign Resend Shortcuts offered.
    /// * `include_resend_shortcut_usage` - Return the `resend_shortcut_usage` field in the response.  This includes information about campaigns related by a shortcut.
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
    ///         .campaigns
    ///         .get(
    ///             &"campaign_id".to_string(),
    ///             &CampaignsGetQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 include_resend_shortcut_eligibility: None,
    ///                 include_resend_shortcut_usage: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        campaign_id: &str,
        request: &CampaignsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Campaign, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/campaigns/{}", campaign_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .bool(
                        "include_resend_shortcut_eligibility",
                        request.include_resend_shortcut_eligibility.clone(),
                    )
                    .bool(
                        "include_resend_shortcut_usage",
                        request.include_resend_shortcut_usage.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Remove a campaign from your Mailchimp account.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .campaigns
    ///         .delete(&"campaign_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/campaigns/{}", campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update some or all of the settings for a specific campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
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
    ///         .campaigns
    ///         .update(
    ///             &"campaign_id".to_string(),
    ///             &UpdateCampaignsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        campaign_id: &str,
        request: &UpdateCampaignsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Campaign, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/campaigns/{}", campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Cancel a Regular or Plain-Text Campaign after you send, before all of your recipients receive it. This feature is included with Mailchimp Pro.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .campaigns
    ///         .create_action_cancel_send(&"campaign_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_cancel_send(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/campaigns/{}/actions/cancel-send", campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Remove the guesswork for resending a campaign to certain segments. You can use this endpoint as a shortcut to replicate a campaign and resend it to common segments, such as those who didn't open the campaign, or any new subscribers since it was sent.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
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
    ///         .campaigns
    ///         .create_action_create_resend(
    ///             &"campaign_id".to_string(),
    ///             &CreateActionCreateResendCampaignsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_create_resend(
        &self,
        campaign_id: &str,
        request: &CreateActionCreateResendCampaignsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Campaign, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/campaigns/{}/actions/create-resend", campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Pause an RSS-Driven campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .campaigns
    ///         .create_action_pause(&"campaign_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_pause(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/campaigns/{}/actions/pause", campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Replicate a campaign in saved or send status.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
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
    ///         .campaigns
    ///         .create_action_replicate(&"campaign_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_replicate(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Campaign, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/campaigns/{}/actions/replicate", campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Resume an RSS-Driven campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .campaigns
    ///         .create_action_resume(&"campaign_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_resume(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/campaigns/{}/actions/resume", campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Schedule a campaign for delivery. If you're using Multivariate Campaigns to test send times or sending RSS Campaigns, use the send action instead.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .campaigns
    ///         .create_action_schedule(
    ///             &"campaign_id".to_string(),
    ///             &CreateActionScheduleCampaignsRequest {
    ///                 schedule_time: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z").unwrap(),
    ///                 batch_delivery: None,
    ///                 timewarp: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_schedule(
        &self,
        campaign_id: &str,
        request: &CreateActionScheduleCampaignsRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/campaigns/{}/actions/schedule", campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Send a Mailchimp campaign. For RSS Campaigns, the campaign will send according to its schedule. All other campaigns will send immediately.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .campaigns
    ///         .create_action_send(&"campaign_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_send(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/campaigns/{}/actions/send", campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Send a test email.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .campaigns
    ///         .create_action_test(
    ///             &"campaign_id".to_string(),
    ///             &CreateActionTestCampaignsRequest {
    ///                 send_type: CreateActionTestCampaignsRequestSendType::HTML,
    ///                 test_emails: vec!["test_emails".to_string()],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_test(
        &self,
        campaign_id: &str,
        request: &CreateActionTestCampaignsRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/campaigns/{}/actions/test", campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Unschedule a scheduled campaign that hasn't started sending.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .campaigns
    ///         .create_action_unschedule(&"campaign_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_unschedule(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/campaigns/{}/actions/unschedule", campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get the the HTML and plain-text content for a campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
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
    ///         .campaigns
    ///         .get_content(
    ///             &"campaign_id".to_string(),
    ///             &CampaignsGetContentQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_content(
        &self,
        campaign_id: &str,
        request: &CampaignsGetContentQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CampaignContent, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/campaigns/{}/content", campaign_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Set the content for a campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
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
    ///         .campaigns
    ///         .upsert_content(
    ///             &"campaign_id".to_string(),
    ///             &CampaignContent {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upsert_content(
        &self,
        campaign_id: &str,
        request: &CampaignContent,
        options: Option<RequestOptions>,
    ) -> Result<CampaignContent, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("3.0/campaigns/{}/content", campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get team feedback while you're working together on a Mailchimp campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
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
    ///         .campaigns
    ///         .list_feedback(
    ///             &"campaign_id".to_string(),
    ///             &ListFeedbackQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_feedback(
        &self,
        campaign_id: &str,
        request: &ListFeedbackQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListFeedbackCampaignsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/campaigns/{}/feedback", campaign_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add feedback on a specific campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
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
    ///         .campaigns
    ///         .create_feedback(
    ///             &"campaign_id".to_string(),
    ///             &CreateFeedbackCampaignsRequest {
    ///                 message: "message".to_string(),
    ///                 block_id: None,
    ///                 is_complete: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_feedback(
        &self,
        campaign_id: &str,
        request: &CreateFeedbackCampaignsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateFeedbackCampaignsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/campaigns/{}/feedback", campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a specific feedback message from a campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `feedback_id` - The unique id for the feedback message.
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
    ///         .campaigns
    ///         .get_feedback(
    ///             &"campaign_id".to_string(),
    ///             &"feedback_id".to_string(),
    ///             &GetFeedbackQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_feedback(
        &self,
        campaign_id: &str,
        feedback_id: &str,
        request: &GetFeedbackQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CampaignFeedback, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/campaigns/{}/feedback/{}", campaign_id, feedback_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Remove a specific feedback message for a campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `feedback_id` - The unique id for the feedback message.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .campaigns
    ///         .delete_feedback(&"campaign_id".to_string(), &"feedback_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_feedback(
        &self,
        campaign_id: &str,
        feedback_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/campaigns/{}/feedback/{}", campaign_id, feedback_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific feedback message for a campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `feedback_id` - The unique id for the feedback message.
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
    ///         .campaigns
    ///         .update_feedback(
    ///             &"campaign_id".to_string(),
    ///             &"feedback_id".to_string(),
    ///             &UpdateFeedbackCampaignsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_feedback(
        &self,
        campaign_id: &str,
        feedback_id: &str,
        request: &UpdateFeedbackCampaignsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CampaignFeedback, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/campaigns/{}/feedback/{}", campaign_id, feedback_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Review the send checklist for a campaign, and resolve any issues before sending.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
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
    ///         .campaigns
    ///         .list_send_checklist(
    ///             &"campaign_id".to_string(),
    ///             &ListSendChecklistQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_send_checklist(
        &self,
        campaign_id: &str,
        request: &ListSendChecklistQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSendChecklistCampaignsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/campaigns/{}/send-checklist", campaign_id),
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
