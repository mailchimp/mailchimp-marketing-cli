//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Audiences**

use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod audiences;
pub struct ApiClient {
    pub config: ClientConfig,
    pub http_client: HttpClient,
    pub audiences: AudiencesClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            http_client: HttpClient::new(config.clone())?,
            audiences: AudiencesClient::new(config.clone())?,
        })
    }

    /// Get all of the sending domains on the account.
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
    ///     client.list(None).await;
    /// }
    /// ```
    pub async fn list(&self, options: Option<RequestOptions>) -> Result<ListResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "3.0/verified-domains", None, None, options)
            .await
    }

    /// Add a domain to the account.
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
    ///         .create(
    ///             &CreateRequest {
    ///                 verification_email: "verification_email".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "3.0/verified-domains",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get the details for a single domain on the account.
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The domain name.
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
    ///     client.get(&"domain_name".to_string(), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        domain_name: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/verified-domains/{}", domain_name),
                None,
                None,
                options,
            )
            .await
    }

    /// Return the Chimp Chatter for this account ordered by most recent.
    ///
    /// # Arguments
    ///
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_chimp_chatter(
    ///             &ListChimpChatterQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_chimp_chatter(
        &self,
        request: &ListChimpChatterQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListChimpChatterResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/activity-feed/chimp-chatter",
                None,
                QueryBuilder::new()
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
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
    /// use mcapi_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = McapiClient::new(config).expect("Failed to build client");
    ///     client.list_emails(&"workflow_id".to_string(), None).await;
    /// }
    /// ```
    pub async fn list_emails(
        &self,
        workflow_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListEmailsResponse, ApiError> {
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
    ///         .update_email(
    ///             &"workflow_id".to_string(),
    ///             &"workflow_email_id".to_string(),
    ///             &UpdateEmailRequest {
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
        request: &UpdateEmailRequest,
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
    ) -> Result<ListEmailQueueResponse, ApiError> {
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
    ///         .create_email_queue(
    ///             &"workflow_id".to_string(),
    ///             &"workflow_email_id".to_string(),
    ///             &CreateEmailQueueRequest {
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
        request: &CreateEmailQueueRequest,
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
    ///         .list_removed_subscribers(&"workflow_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_removed_subscribers(
        &self,
        workflow_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListRemovedSubscribersResponse, ApiError> {
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
    ///         .create_removed_subscriber(
    ///             &"workflow_id".to_string(),
    ///             &CreateRemovedSubscriberRequest {
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
        request: &CreateRemovedSubscriberRequest,
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

    /// Delete a verified domain from the account.
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The domain name.
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
    ///     client.delete(&"domain_name".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        domain_name: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/verified-domains/{}", domain_name),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the name, HTML, or `folder_id` of an existing template.
    ///
    /// # Arguments
    ///
    /// * `template_id` - The unique id for the template.
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
    ///         .update(
    ///             &"template_id".to_string(),
    ///             &UpdateRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        template_id: &str,
        request: &UpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<TemplateInstance, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/templates/{}", template_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Cancel a scheduled or sending SMS campaign.
    ///
    /// # Arguments
    ///
    /// * `sms_campaign_id` - The unique id for the SMS campaign.
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
    ///         .create_action_cancel_send(&"sms_campaign_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_cancel_send(
        &self,
        sms_campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/sms-campaigns/{}/actions/cancel-send", sms_campaign_id),
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
    ///         .create_action_create_resend(
    ///             &"campaign_id".to_string(),
    ///             &CreateActionCreateResendRequest {
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
        request: &CreateActionCreateResendRequest,
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

    /// Schedule an SMS campaign for delivery.
    ///
    /// # Arguments
    ///
    /// * `sms_campaign_id` - The unique id for the SMS campaign.
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
    ///         .create_action_schedule(
    ///             &"sms_campaign_id".to_string(),
    ///             &CreateActionScheduleRequest {
    ///                 schedule_time: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z").unwrap(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_schedule(
        &self,
        sms_campaign_id: &str,
        request: &CreateActionScheduleRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/sms-campaigns/{}/actions/schedule", sms_campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Send an SMS campaign.
    ///
    /// # Arguments
    ///
    /// * `sms_campaign_id` - The unique id for the SMS campaign.
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
    ///         .create_action_send(&"sms_campaign_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_send(
        &self,
        sms_campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/sms-campaigns/{}/actions/send", sms_campaign_id),
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
    ///         .create_action_test(
    ///             &"campaign_id".to_string(),
    ///             &CreateActionTestRequest {
    ///                 send_type: CreateActionTestRequestSendType::HTML,
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
        request: &CreateActionTestRequest,
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

    /// Get the content for an SMS campaign.
    ///
    /// # Arguments
    ///
    /// * `sms_campaign_id` - The unique id for the SMS campaign.
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
    ///         .get_content(
    ///             &"sms_campaign_id".to_string(),
    ///             &GetContentQueryRequest {
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
        sms_campaign_id: &str,
        request: &GetContentQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SmsCampaignContent, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/sms-campaigns/{}/content", sms_campaign_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Set the content for an SMS campaign.
    ///
    /// # Arguments
    ///
    /// * `sms_campaign_id` - The unique id for the SMS campaign.
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
    ///         .upsert_content(
    ///             &"sms_campaign_id".to_string(),
    ///             &UpsertContentRequest {
    ///                 message_body: "message_body".to_string(),
    ///                 media: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upsert_content(
        &self,
        sms_campaign_id: &str,
        request: &UpsertContentRequest,
        options: Option<RequestOptions>,
    ) -> Result<SmsCampaignContent, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("3.0/sms-campaigns/{}/content", sms_campaign_id),
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
    ) -> Result<ListFeedbackResponse, ApiError> {
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
    ///         .create_feedback(
    ///             &"campaign_id".to_string(),
    ///             &CreateFeedbackRequest {
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
        request: &CreateFeedbackRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateFeedbackResponse, ApiError> {
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
    ///         .update_feedback(
    ///             &"campaign_id".to_string(),
    ///             &"feedback_id".to_string(),
    ///             &UpdateFeedbackRequest {
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
        request: &UpdateFeedbackRequest,
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
    ) -> Result<ListSendChecklistResponse, ApiError> {
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

    /// Verify that the connected sites script has been installed, either via the script URL or fragment.
    ///
    /// # Arguments
    ///
    /// * `connected_site_id` - The unique identifier for the site.
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
    ///         .create_action_verify_script_installation(&"connected_site_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_verify_script_installation(
        &self,
        connected_site_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/connected-sites/{}/actions/verify-script-installation",
                    connected_site_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Get messages from a specific conversation. Conversations has been deprecated in favor of Inbox and these endpoints don't include Inbox data. Past Conversations are still available via this endpoint, but new campaign replies and other Inbox messages aren’t available using this endpoint.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - The unique id for the conversation.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `is_read` - Whether a conversation message has been marked as read.
    /// * `before_timestamp` - Restrict the response to messages created before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `since_timestamp` - Restrict the response to messages created after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
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
    ///         .list_messages(
    ///             &"conversation_id".to_string(),
    ///             &ListMessagesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 is_read: None,
    ///                 before_timestamp: None,
    ///                 since_timestamp: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_messages(
        &self,
        conversation_id: &str,
        request: &ListMessagesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMessagesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/conversations/{}/messages", conversation_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .serialize("is_read", request.is_read.clone())
                    .datetime("before_timestamp", request.before_timestamp.clone())
                    .datetime("since_timestamp", request.since_timestamp.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get an individual message in a conversation. Conversations has been deprecated in favor of Inbox and these endpoints don't include Inbox data. Past Conversations are still available via this endpoint, but new campaign replies and other Inbox messages aren’t available using this endpoint.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - The unique id for the conversation.
    /// * `message_id` - The unique id for the conversation message.
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
    ///         .get_message(
    ///             &"conversation_id".to_string(),
    ///             &"message_id".to_string(),
    ///             &GetMessageQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_message(
        &self,
        conversation_id: &str,
        message_id: &str,
        request: &GetMessageQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConversationMessage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/conversations/{}/messages/{}",
                    conversation_id, message_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// A step trigger in an Automation flow. To use it, create a starting point or step from the Automation flow builder in the app using the Customer Journeys API condition. We’ll provide a url during the process that includes the {journey_id} and {step_id}. You’ll then be able to use this endpoint to trigger the condition for the posted contact.
    ///
    /// # Arguments
    ///
    /// * `journey_id` - The id for the flow.
    /// * `step_id` - The id for the Step.
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
    ///         .create_journey_step_action_trigger(
    ///             1,
    ///             1,
    ///             &CreateJourneyStepActionTriggerRequest {
    ///                 email_address: "email_address".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_journey_step_action_trigger(
        &self,
        journey_id: i64,
        step_id: i64,
        request: &CreateJourneyStepActionTriggerRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/customer-journeys/journeys/{}/steps/{}/actions/trigger",
                    journey_id, step_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about an account's orders.
    ///
    /// # Arguments
    ///
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `campaign_id` - Restrict results to orders with a specific `campaign_id` value.
    /// * `outreach_id` - Restrict results to orders with a specific `outreach_id` value.
    /// * `customer_id` - Restrict results to orders made by a specific customer.
    /// * `has_outreach` - Restrict results to orders that have an outreach attached. For example, an email campaign or Facebook ad.
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
    ///         .list_orders(
    ///             &ListOrdersQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 campaign_id: None,
    ///                 outreach_id: None,
    ///                 customer_id: None,
    ///                 has_outreach: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_orders(
        &self,
        request: &ListOrdersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListOrdersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/ecommerce/orders",
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("campaign_id", request.campaign_id.clone())
                    .string("outreach_id", request.outreach_id.clone())
                    .string("customer_id", request.customer_id.clone())
                    .bool("has_outreach", request.has_outreach.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get information about all stores in the account.
    ///
    /// # Arguments
    ///
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_stores(
    ///             &ListStoresQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_stores(
        &self,
        request: &ListStoresQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStoresResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/ecommerce/stores",
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a new store to your Mailchimp account.
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
    ///         .create_store(
    ///             &CreateStoreRequest {
    ///                 currency_code: "USD".to_string(),
    ///                 id: "example_store".to_string(),
    ///                 list_id: "1a2df69511".to_string(),
    ///                 name: "Freddie's Cat Hat Emporium".to_string(),
    ///                 address: None,
    ///                 domain: None,
    ///                 email_address: None,
    ///                 is_syncing: None,
    ///                 money_format: None,
    ///                 phone: None,
    ///                 platform: None,
    ///                 primary_locale: None,
    ///                 timezone: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store(
        &self,
        request: &CreateStoreRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceStore, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "3.0/ecommerce/stores",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
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
    ///         .get_store(
    ///             &"store_id".to_string(),
    ///             &GetStoreQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store(
        &self,
        store_id: &str,
        request: &GetStoreQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceStore, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}", store_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a store. Deleting a store will also delete any associated subresources, including Customers, Orders, Products, and Carts.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
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
    ///     client.delete_store(&"store_id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete_store(
        &self,
        store_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/ecommerce/stores/{}", store_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
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
    ///         .update_store(
    ///             &"store_id".to_string(),
    ///             &UpdateStoreRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store(
        &self,
        store_id: &str,
        request: &UpdateStoreRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceStore, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/ecommerce/stores/{}", store_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a store's carts.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_store_carts(
    ///             &"store_id".to_string(),
    ///             &ListStoreCartsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_carts(
        &self,
        store_id: &str,
        request: &ListStoreCartsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStoreCartsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}/carts", store_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a new cart to a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
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
    ///         .create_store_cart(
    ///             &"store_id".to_string(),
    ///             &CreateStoreCartRequest {
    ///                 currency_code: "currency_code".to_string(),
    ///                 customer: EcommerceStoresCartsPost {
    ///                     id: "id".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///                 id: CreateStoreCartRequestID::String("id".to_string()),
    ///                 lines: vec![CreateStoreCartRequestLinesItem {
    ///                     id: "id".to_string(),
    ///                     price: CreateStoreCartRequestLinesItemPrice::Double(1.1),
    ///                     product_id: "product_id".to_string(),
    ///                     product_variant_id: "product_variant_id".to_string(),
    ///                     quantity: 1,
    ///                 }],
    ///                 order_total: CreateStoreCartRequestOrderTotal::Double(1.1),
    ///                 campaign_id: None,
    ///                 checkout_url: None,
    ///                 tax_total: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store_cart(
        &self,
        store_id: &str,
        request: &CreateStoreCartRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCart, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/ecommerce/stores/{}/carts", store_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific cart.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `cart_id` - The id for the cart.
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
    ///         .get_store_cart(
    ///             &"store_id".to_string(),
    ///             &"cart_id".to_string(),
    ///             &GetStoreCartQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_cart(
        &self,
        store_id: &str,
        cart_id: &str,
        request: &GetStoreCartQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCart, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}/carts/{}", store_id, cart_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a cart.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `cart_id` - The id for the cart.
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
    ///         .delete_store_cart(&"store_id".to_string(), &"cart_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_cart(
        &self,
        store_id: &str,
        cart_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/ecommerce/stores/{}/carts/{}", store_id, cart_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific cart.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `cart_id` - The id for the cart.
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
    ///         .update_store_cart(
    ///             &"store_id".to_string(),
    ///             &"cart_id".to_string(),
    ///             &UpdateStoreCartRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_cart(
        &self,
        store_id: &str,
        cart_id: &str,
        request: &UpdateStoreCartRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCart, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/ecommerce/stores/{}/carts/{}", store_id, cart_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a cart's line items.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `cart_id` - The id for the cart.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_store_cart_lines(
    ///             &"store_id".to_string(),
    ///             &"cart_id".to_string(),
    ///             &ListStoreCartLinesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_cart_lines(
        &self,
        store_id: &str,
        cart_id: &str,
        request: &ListStoreCartLinesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStoreCartLinesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}/carts/{}/lines", store_id, cart_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a new line item to an existing cart.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `cart_id` - The id for the cart.
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
    ///         .create_store_cart_line(
    ///             &"store_id".to_string(),
    ///             &"cart_id".to_string(),
    ///             &CreateStoreCartLineRequest {
    ///                 id: "id".to_string(),
    ///                 price: CreateStoreCartLineRequestPrice::Double(1.1),
    ///                 product_id: "product_id".to_string(),
    ///                 product_variant_id: "product_variant_id".to_string(),
    ///                 quantity: 1,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store_cart_line(
        &self,
        store_id: &str,
        cart_id: &str,
        request: &CreateStoreCartLineRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCartLineItem, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/ecommerce/stores/{}/carts/{}/lines", store_id, cart_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific cart line item.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `cart_id` - The id for the cart.
    /// * `line_id` - The id for the line item of a cart.
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
    ///         .get_store_cart_line(
    ///             &"store_id".to_string(),
    ///             &"cart_id".to_string(),
    ///             &"line_id".to_string(),
    ///             &GetStoreCartLineQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_cart_line(
        &self,
        store_id: &str,
        cart_id: &str,
        line_id: &str,
        request: &GetStoreCartLineQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCartLineItem, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/carts/{}/lines/{}",
                    store_id, cart_id, line_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a specific cart line item.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `cart_id` - The id for the cart.
    /// * `line_id` - The id for the line item of a cart.
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
    ///         .delete_store_cart_line(
    ///             &"store_id".to_string(),
    ///             &"cart_id".to_string(),
    ///             &"line_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_cart_line(
        &self,
        store_id: &str,
        cart_id: &str,
        line_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/ecommerce/stores/{}/carts/{}/lines/{}",
                    store_id, cart_id, line_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific cart line item.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `cart_id` - The id for the cart.
    /// * `line_id` - The id for the line item of a cart.
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
    ///         .update_store_cart_line(
    ///             &"store_id".to_string(),
    ///             &"cart_id".to_string(),
    ///             &"line_id".to_string(),
    ///             &UpdateStoreCartLineRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_cart_line(
        &self,
        store_id: &str,
        cart_id: &str,
        line_id: &str,
        request: &UpdateStoreCartLineRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCartLineItem, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/ecommerce/stores/{}/carts/{}/lines/{}",
                    store_id, cart_id, line_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a store's customers.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `email_address` - Restrict the response to customers with the email address.
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
    ///         .list_store_customers(
    ///             &"store_id".to_string(),
    ///             &ListStoreCustomersQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 email_address: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_customers(
        &self,
        store_id: &str,
        request: &ListStoreCustomersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStoreCustomersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}/customers", store_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("email_address", request.email_address.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a new customer to a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
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
    ///         .create_store_customer(
    ///             &"store_id".to_string(),
    ///             &CreateStoreCustomerRequest {
    ///                 id: "id".to_string(),
    ///                 opt_in_status: true,
    ///                 address: None,
    ///                 company: None,
    ///                 email_address: None,
    ///                 first_name: None,
    ///                 last_name: None,
    ///                 sms_phone_number: None,
    ///                 total_spent: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store_customer(
        &self,
        store_id: &str,
        request: &CreateStoreCustomerRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCustomer, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/ecommerce/stores/{}/customers", store_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific customer.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `customer_id` - The id for the customer of a store.
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
    ///         .get_store_customer(
    ///             &"store_id".to_string(),
    ///             &"customer_id".to_string(),
    ///             &GetStoreCustomerQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_customer(
        &self,
        store_id: &str,
        customer_id: &str,
        request: &GetStoreCustomerQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCustomer, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/customers/{}",
                    store_id, customer_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add or update a customer.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `customer_id` - The id for the customer of a store.
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
    ///         .upsert_store_customer(
    ///             &"store_id".to_string(),
    ///             &"customer_id".to_string(),
    ///             &UpsertStoreCustomerRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upsert_store_customer(
        &self,
        store_id: &str,
        customer_id: &str,
        request: &UpsertStoreCustomerRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCustomer, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!(
                    "3.0/ecommerce/stores/{}/customers/{}",
                    store_id, customer_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a customer from a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `customer_id` - The id for the customer of a store.
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
    ///         .delete_store_customer(&"store_id".to_string(), &"customer_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_customer(
        &self,
        store_id: &str,
        customer_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/ecommerce/stores/{}/customers/{}",
                    store_id, customer_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a customer.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `customer_id` - The id for the customer of a store.
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
    ///         .update_store_customer(
    ///             &"store_id".to_string(),
    ///             &"customer_id".to_string(),
    ///             &EcommerceStoresCartsPatch {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_customer(
        &self,
        store_id: &str,
        customer_id: &str,
        request: &EcommerceStoresCartsPatch,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceCustomer, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/ecommerce/stores/{}/customers/{}",
                    store_id, customer_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a store's orders.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `customer_id` - Restrict results to orders made by a specific customer.
    /// * `has_outreach` - Restrict results to orders that have an outreach attached. For example, an email campaign or Facebook ad.
    /// * `campaign_id` - Restrict results to orders with a specific `campaign_id` value.
    /// * `outreach_id` - Restrict results to orders with a specific `outreach_id` value.
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
    ///         .list_store_orders(
    ///             &"store_id".to_string(),
    ///             &ListStoreOrdersQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 customer_id: None,
    ///                 has_outreach: None,
    ///                 campaign_id: None,
    ///                 outreach_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_orders(
        &self,
        store_id: &str,
        request: &ListStoreOrdersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStoreOrdersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}/orders", store_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("customer_id", request.customer_id.clone())
                    .bool("has_outreach", request.has_outreach.clone())
                    .string("campaign_id", request.campaign_id.clone())
                    .string("outreach_id", request.outreach_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a new order to a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
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
    ///         .create_store_order(
    ///             &"store_id".to_string(),
    ///             &CreateStoreOrderRequest {
    ///                 currency_code: "currency_code".to_string(),
    ///                 customer: EcommerceStoresCartsPost {
    ///                     id: "id".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///                 id: "id".to_string(),
    ///                 lines: vec![CreateStoreOrderRequestLinesItem {
    ///                     discount: None,
    ///                     id: "id".to_string(),
    ///                     price: CreateStoreOrderRequestLinesItemPrice::Double(1.1),
    ///                     product: None,
    ///                     product_id: "product_id".to_string(),
    ///                     product_variant_id: "product_variant_id".to_string(),
    ///                     quantity: 1,
    ///                 }],
    ///                 order_total: CreateStoreOrderRequestOrderTotal::Double(1.1),
    ///                 billing_address: None,
    ///                 campaign_id: None,
    ///                 cart_id: None,
    ///                 cancelled_at_foreign: None,
    ///                 discount_total: None,
    ///                 financial_status: None,
    ///                 fulfillment_status: None,
    ///                 landing_site: None,
    ///                 order_url: None,
    ///                 outreach: None,
    ///                 processed_at_foreign: None,
    ///                 promos: None,
    ///                 shipping_address: None,
    ///                 shipping_total: None,
    ///                 tax_total: None,
    ///                 tracking_carrier: None,
    ///                 tracking_code: None,
    ///                 tracking_number: None,
    ///                 tracking_url: None,
    ///                 updated_at_foreign: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store_order(
        &self,
        store_id: &str,
        request: &CreateStoreOrderRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceOrder, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/ecommerce/stores/{}/orders", store_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific order.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `order_id` - The id for the order in a store.
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
    ///         .get_store_order(
    ///             &"store_id".to_string(),
    ///             &"order_id".to_string(),
    ///             &GetStoreOrderQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_order(
        &self,
        store_id: &str,
        order_id: &str,
        request: &GetStoreOrderQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceOrder, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}/orders/{}", store_id, order_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete an order.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `order_id` - The id for the order in a store.
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
    ///         .delete_store_order(&"store_id".to_string(), &"order_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_order(
        &self,
        store_id: &str,
        order_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/ecommerce/stores/{}/orders/{}", store_id, order_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific order.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `order_id` - The id for the order in a store.
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
    ///         .update_store_order(
    ///             &"store_id".to_string(),
    ///             &"order_id".to_string(),
    ///             &UpdateStoreOrderRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_order(
        &self,
        store_id: &str,
        order_id: &str,
        request: &UpdateStoreOrderRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceOrder, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/ecommerce/stores/{}/orders/{}", store_id, order_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about an order's line items.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `order_id` - The id for the order in a store.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_store_order_lines(
    ///             &"store_id".to_string(),
    ///             &"order_id".to_string(),
    ///             &ListStoreOrderLinesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_order_lines(
        &self,
        store_id: &str,
        order_id: &str,
        request: &ListStoreOrderLinesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStoreOrderLinesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/orders/{}/lines",
                    store_id, order_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a new line item to an existing order.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `order_id` - The id for the order in a store.
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
    ///         .create_store_order_line(
    ///             &"store_id".to_string(),
    ///             &"order_id".to_string(),
    ///             &CreateStoreOrderLineRequest {
    ///                 id: "id".to_string(),
    ///                 price: CreateStoreOrderLineRequestPrice::Double(1.1),
    ///                 product_id: "product_id".to_string(),
    ///                 product_variant_id: "product_variant_id".to_string(),
    ///                 quantity: 1,
    ///                 discount: None,
    ///                 product: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store_order_line(
        &self,
        store_id: &str,
        order_id: &str,
        request: &CreateStoreOrderLineRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceOrderLineItem, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/ecommerce/stores/{}/orders/{}/lines",
                    store_id, order_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific order line item.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `order_id` - The id for the order in a store.
    /// * `line_id` - The id for the line item of an order.
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
    ///         .get_store_order_line(
    ///             &"store_id".to_string(),
    ///             &"order_id".to_string(),
    ///             &"line_id".to_string(),
    ///             &GetStoreOrderLineQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_order_line(
        &self,
        store_id: &str,
        order_id: &str,
        line_id: &str,
        request: &GetStoreOrderLineQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceOrderLineItem, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/orders/{}/lines/{}",
                    store_id, order_id, line_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a specific order line item.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `order_id` - The id for the order in a store.
    /// * `line_id` - The id for the line item of an order.
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
    ///         .delete_store_order_line(
    ///             &"store_id".to_string(),
    ///             &"order_id".to_string(),
    ///             &"line_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_order_line(
        &self,
        store_id: &str,
        order_id: &str,
        line_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/ecommerce/stores/{}/orders/{}/lines/{}",
                    store_id, order_id, line_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific order line item.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `order_id` - The id for the order in a store.
    /// * `line_id` - The id for the line item of an order.
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
    ///         .update_store_order_line(
    ///             &"store_id".to_string(),
    ///             &"order_id".to_string(),
    ///             &"line_id".to_string(),
    ///             &UpdateStoreOrderLineRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_order_line(
        &self,
        store_id: &str,
        order_id: &str,
        line_id: &str,
        request: &UpdateStoreOrderLineRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceOrderLineItem, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/ecommerce/stores/{}/orders/{}/lines/{}",
                    store_id, order_id, line_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a store's products.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_store_products(
    ///             &"store_id".to_string(),
    ///             &ListStoreProductsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_products(
        &self,
        store_id: &str,
        request: &ListStoreProductsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStoreProductsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}/products", store_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a new product to a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
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
    ///         .create_store_product(
    ///             &"store_id".to_string(),
    ///             &EcommerceStoresOrdersPost {
    ///                 description: None,
    ///                 handle: None,
    ///                 id: EcommerceStoresOrdersPostID::String("id".to_string()),
    ///                 image_url: None,
    ///                 images: None,
    ///                 published_at_foreign: None,
    ///                 title: "Cat Hat".to_string(),
    ///                 r#type: None,
    ///                 url: None,
    ///                 variants: vec![EcommerceStoresOrdersPostVariantsItem {
    ///                     backorders: None,
    ///                     id: EcommerceStoresOrdersPostVariantsItemID::String("id".to_string()),
    ///                     image_url: None,
    ///                     inventory_quantity: None,
    ///                     price: None,
    ///                     sku: None,
    ///                     title: "Cat Hat".to_string(),
    ///                     url: None,
    ///                     visibility: None,
    ///                 }],
    ///                 vendor: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store_product(
        &self,
        store_id: &str,
        request: &EcommerceStoresOrdersPost,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceProduct, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/ecommerce/stores/{}/products", store_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific product.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
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
    ///         .get_store_product(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &GetStoreProductQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_product(
        &self,
        store_id: &str,
        product_id: &str,
        request: &GetStoreProductQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceProduct, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}/products/{}", store_id, product_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Update a specific product.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
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
    ///         .upsert_store_product(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &UpsertStoreProductRequest {
    ///                 id: UpsertStoreProductRequestID::String("id".to_string()),
    ///                 description: None,
    ///                 handle: None,
    ///                 image_url: None,
    ///                 images: None,
    ///                 published_at_foreign: None,
    ///                 title: None,
    ///                 r#type: None,
    ///                 url: None,
    ///                 variants: None,
    ///                 vendor: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upsert_store_product(
        &self,
        store_id: &str,
        product_id: &str,
        request: &UpsertStoreProductRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceProduct, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("3.0/ecommerce/stores/{}/products/{}", store_id, product_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a product.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
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
    ///         .delete_store_product(&"store_id".to_string(), &"product_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_product(
        &self,
        store_id: &str,
        product_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/ecommerce/stores/{}/products/{}", store_id, product_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific product.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
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
    ///         .update_store_product(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &UpdateStoreProductRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_product(
        &self,
        store_id: &str,
        product_id: &str,
        request: &UpdateStoreProductRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceProduct, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/ecommerce/stores/{}/products/{}", store_id, product_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a product's images.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_store_product_images(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &ListStoreProductImagesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_product_images(
        &self,
        store_id: &str,
        product_id: &str,
        request: &ListStoreProductImagesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStoreProductImagesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/images",
                    store_id, product_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a new image to the product.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
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
    ///         .create_store_product_image(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &CreateStoreProductImageRequest {
    ///                 id: "id".to_string(),
    ///                 url: "url".to_string(),
    ///                 variant_ids: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store_product_image(
        &self,
        store_id: &str,
        product_id: &str,
        request: &CreateStoreProductImageRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateStoreProductImageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/images",
                    store_id, product_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific product image.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `image_id` - The id for the product image.
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
    ///         .get_store_product_image(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &"image_id".to_string(),
    ///             &GetStoreProductImageQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_product_image(
        &self,
        store_id: &str,
        product_id: &str,
        image_id: &str,
        request: &GetStoreProductImageQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetStoreProductImageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/images/{}",
                    store_id, product_id, image_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a product image.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `image_id` - The id for the product image.
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
    ///         .delete_store_product_image(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &"image_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_product_image(
        &self,
        store_id: &str,
        product_id: &str,
        image_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/images/{}",
                    store_id, product_id, image_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a product image.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `image_id` - The id for the product image.
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
    ///         .update_store_product_image(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &"image_id".to_string(),
    ///             &UpdateStoreProductImageRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_product_image(
        &self,
        store_id: &str,
        product_id: &str,
        image_id: &str,
        request: &UpdateStoreProductImageRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateStoreProductImageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/images/{}",
                    store_id, product_id, image_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a product's variants.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_store_product_variants(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &ListStoreProductVariantsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_product_variants(
        &self,
        store_id: &str,
        product_id: &str,
        request: &ListStoreProductVariantsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStoreProductVariantsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/variants",
                    store_id, product_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a new variant to the product.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
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
    ///         .create_store_product_variant(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &CreateStoreProductVariantRequest {
    ///                 id: CreateStoreProductVariantRequestID::String("id".to_string()),
    ///                 title: "Cat Hat".to_string(),
    ///                 backorders: None,
    ///                 image_url: None,
    ///                 inventory_quantity: None,
    ///                 price: None,
    ///                 sku: None,
    ///                 url: None,
    ///                 visibility: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store_product_variant(
        &self,
        store_id: &str,
        product_id: &str,
        request: &CreateStoreProductVariantRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceProductVariant, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/variants",
                    store_id, product_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific product variant.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `variant_id` - The id for the product variant.
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
    ///         .get_store_product_variant(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &"variant_id".to_string(),
    ///             &GetStoreProductVariantQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_product_variant(
        &self,
        store_id: &str,
        product_id: &str,
        variant_id: &str,
        request: &GetStoreProductVariantQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceProductVariant, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/variants/{}",
                    store_id, product_id, variant_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add or update a product variant.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `variant_id` - The id for the product variant.
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
    ///         .upsert_store_product_variant(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &"variant_id".to_string(),
    ///             &UpsertStoreProductVariantRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upsert_store_product_variant(
        &self,
        store_id: &str,
        product_id: &str,
        variant_id: &str,
        request: &UpsertStoreProductVariantRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceProductVariant, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/variants/{}",
                    store_id, product_id, variant_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a product variant.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `variant_id` - The id for the product variant.
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
    ///         .delete_store_product_variant(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &"variant_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_product_variant(
        &self,
        store_id: &str,
        product_id: &str,
        variant_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/variants/{}",
                    store_id, product_id, variant_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a product variant.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `product_id` - The id for the product of a store.
    /// * `variant_id` - The id for the product variant.
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
    ///         .update_store_product_variant(
    ///             &"store_id".to_string(),
    ///             &"product_id".to_string(),
    ///             &"variant_id".to_string(),
    ///             &UpdateStoreProductVariantRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_product_variant(
        &self,
        store_id: &str,
        product_id: &str,
        variant_id: &str,
        request: &UpdateStoreProductVariantRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommerceProductVariant, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/ecommerce/stores/{}/products/{}/variants/{}",
                    store_id, product_id, variant_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a store's promo rules.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_store_promo_rules(
    ///             &"store_id".to_string(),
    ///             &ListStorePromoRulesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_promo_rules(
        &self,
        store_id: &str,
        request: &ListStorePromoRulesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStorePromoRulesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/ecommerce/stores/{}/promo-rules", store_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a new promo rule to a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
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
    ///         .create_store_promo_rule(
    ///             &"store_id".to_string(),
    ///             &CreateStorePromoRuleRequest {
    ///                 amount: CreateStorePromoRuleRequestAmount::Double(1.1),
    ///                 description: "Save BIG during our summer sale!".to_string(),
    ///                 id: "id".to_string(),
    ///                 target: CreateStorePromoRuleRequestTarget::PerItem,
    ///                 r#type: CreateStorePromoRuleRequestType::Fixed,
    ///                 created_at_foreign: None,
    ///                 enabled: None,
    ///                 ends_at: None,
    ///                 starts_at: None,
    ///                 title: None,
    ///                 updated_at_foreign: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_store_promo_rule(
        &self,
        store_id: &str,
        request: &CreateStorePromoRuleRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommercePromoRule, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/ecommerce/stores/{}/promo-rules", store_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific promo rule.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `promo_rule_id` - The id for the promo rule of a store.
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
    ///         .get_store_promo_rule(
    ///             &"store_id".to_string(),
    ///             &"promo_rule_id".to_string(),
    ///             &GetStorePromoRuleQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_promo_rule(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        request: &GetStorePromoRuleQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommercePromoRule, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/promo-rules/{}",
                    store_id, promo_rule_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a promo rule from a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `promo_rule_id` - The id for the promo rule of a store.
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
    ///         .delete_store_promo_rule(&"store_id".to_string(), &"promo_rule_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_promo_rule(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/ecommerce/stores/{}/promo-rules/{}",
                    store_id, promo_rule_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a promo rule.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `promo_rule_id` - The id for the promo rule of a store.
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
    ///         .update_store_promo_rule(
    ///             &"store_id".to_string(),
    ///             &"promo_rule_id".to_string(),
    ///             &UpdateStorePromoRuleRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_promo_rule(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        request: &UpdateStorePromoRuleRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommercePromoRule, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/ecommerce/stores/{}/promo-rules/{}",
                    store_id, promo_rule_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a store's promo codes.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `promo_rule_id` - The id for the promo rule of a store.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_store_promo_rule_promo_codes(
    ///             &"store_id".to_string(),
    ///             &"promo_rule_id".to_string(),
    ///             &ListStorePromoRulePromoCodesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_store_promo_rule_promo_codes(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        request: &ListStorePromoRulePromoCodesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListStorePromoRulePromoCodesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/promo-rules/{}/promo-codes",
                    store_id, promo_rule_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a new promo code to a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `promo_rule_id` - The id for the promo rule of a store.
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
    ///     client.create_store_promo_rule_promo_code(&"store_id".to_string(), &"promo_rule_id".to_string(), &CreateStorePromoRulePromoCodeRequest {
    ///         code: "summersale".to_string(),
    ///         id: "id".to_string(),
    ///         redemption_url: "A url that applies promo code directly at checkout or a url that points to sale page or store url".to_string(),
    ///         created_at_foreign: None,
    ///         enabled: None,
    ///         updated_at_foreign: None,
    ///         usage_count: None
    ///     }, None).await;
    /// }
    /// ```
    pub async fn create_store_promo_rule_promo_code(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        request: &CreateStorePromoRulePromoCodeRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommercePromoCode, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/ecommerce/stores/{}/promo-rules/{}/promo-codes",
                    store_id, promo_rule_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific promo code.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `promo_rule_id` - The id for the promo rule of a store.
    /// * `promo_code_id` - The id for the promo code of a store.
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
    ///         .get_store_promo_rule_promo_code(
    ///             &"store_id".to_string(),
    ///             &"promo_rule_id".to_string(),
    ///             &"promo_code_id".to_string(),
    ///             &GetStorePromoRulePromoCodeQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_store_promo_rule_promo_code(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        promo_code_id: &str,
        request: &GetStorePromoRulePromoCodeQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommercePromoCode, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/ecommerce/stores/{}/promo-rules/{}/promo-codes/{}",
                    store_id, promo_rule_id, promo_code_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a promo code from a store.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `promo_rule_id` - The id for the promo rule of a store.
    /// * `promo_code_id` - The id for the promo code of a store.
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
    ///         .delete_store_promo_rule_promo_code(
    ///             &"store_id".to_string(),
    ///             &"promo_rule_id".to_string(),
    ///             &"promo_code_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_store_promo_rule_promo_code(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        promo_code_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/ecommerce/stores/{}/promo-rules/{}/promo-codes/{}",
                    store_id, promo_rule_id, promo_code_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a promo code.
    ///
    /// # Arguments
    ///
    /// * `store_id` - The store id.
    /// * `promo_rule_id` - The id for the promo rule of a store.
    /// * `promo_code_id` - The id for the promo code of a store.
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
    ///         .update_store_promo_rule_promo_code(
    ///             &"store_id".to_string(),
    ///             &"promo_rule_id".to_string(),
    ///             &"promo_code_id".to_string(),
    ///             &UpdateStorePromoRulePromoCodeRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_store_promo_rule_promo_code(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        promo_code_id: &str,
        request: &UpdateStorePromoRulePromoCodeRequest,
        options: Option<RequestOptions>,
    ) -> Result<ECommercePromoCode, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/ecommerce/stores/{}/promo-rules/{}/promo-codes/{}",
                    store_id, promo_rule_id, promo_code_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a list of available images and files stored in the File Manager for the account.
    ///
    /// # Arguments
    ///
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `type_` - The file type for the File Manager file.
    /// * `created_by` - The Mailchimp account user who created the File Manager file.
    /// * `before_created_at` - Restrict the response to files created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `since_created_at` - Restrict the response to files created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `sort_field` - Returns files sorted by the specified field.
    /// * `sort_dir` - Determines the order direction for sorted results.
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
    ///         .list_files(
    ///             &ListFilesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 r#type: None,
    ///                 created_by: None,
    ///                 before_created_at: None,
    ///                 since_created_at: None,
    ///                 sort_field: None,
    ///                 sort_dir: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_files(
        &self,
        request: &ListFilesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListFilesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/file-manager/files",
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("type", request.r#type.clone())
                    .string("created_by", request.created_by.clone())
                    .string("before_created_at", request.before_created_at.clone())
                    .string("since_created_at", request.since_created_at.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Upload a new image or file to the File Manager.
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
    ///         .create_file(
    ///             &CreateFileRequest {
    ///                 file_data: "file_data".to_string(),
    ///                 name: "name".to_string(),
    ///                 folder_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_file(
        &self,
        request: &CreateFileRequest,
        options: Option<RequestOptions>,
    ) -> Result<GalleryFile, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "3.0/file-manager/files",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific file in the File Manager.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The unique id for the File Manager file.
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
    ///         .get_file(
    ///             &"file_id".to_string(),
    ///             &GetFileQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_file(
        &self,
        file_id: &str,
        request: &GetFileQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GalleryFile, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/file-manager/files/{}", file_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Remove a specific file from the File Manager.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The unique id for the File Manager file.
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
    ///     client.delete_file(&"file_id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete_file(
        &self,
        file_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/file-manager/files/{}", file_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a file in the File Manager.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The unique id for the File Manager file.
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
    ///         .update_file(
    ///             &"file_id".to_string(),
    ///             &UpdateFileRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_file(
        &self,
        file_id: &str,
        request: &UpdateFileRequest,
        options: Option<RequestOptions>,
    ) -> Result<GalleryFile, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/file-manager/files/{}", file_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a list of all folders in the File Manager.
    ///
    /// # Arguments
    ///
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `created_by` - The Mailchimp account user who created the File Manager file.
    /// * `before_created_at` - Restrict the response to files created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `since_created_at` - Restrict the response to files created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
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
    ///         .list_folders(
    ///             &ListFoldersQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 created_by: None,
    ///                 before_created_at: None,
    ///                 since_created_at: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_folders(
        &self,
        request: &ListFoldersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListFoldersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/file-manager/folders",
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("created_by", request.created_by.clone())
                    .string("before_created_at", request.before_created_at.clone())
                    .string("since_created_at", request.since_created_at.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new folder in the File Manager.
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
    ///         .create_folder(
    ///             &CreateFolderRequest {
    ///                 name: "name".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_folder(
        &self,
        request: &CreateFolderRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateFolderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "3.0/file-manager/folders",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific folder in the File Manager.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The unique id for the File Manager folder.
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
    ///         .get_folder(
    ///             &"folder_id".to_string(),
    ///             &GetFolderQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_folder(
        &self,
        folder_id: &str,
        request: &GetFolderQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetFolderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/file-manager/folders/{}", folder_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a specific folder in the File Manager.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The unique id for the File Manager folder.
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
    ///     client.delete_folder(&"folder_id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete_folder(
        &self,
        folder_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/file-manager/folders/{}", folder_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific File Manager folder.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The unique id for the File Manager folder.
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
    ///         .update_folder(
    ///             &"folder_id".to_string(),
    ///             &UpdateFolderRequest {
    ///                 name: "name".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_folder(
        &self,
        folder_id: &str,
        request: &UpdateFolderRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateFolderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/file-manager/folders/{}", folder_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a list of available images and files stored in this folder.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The unique id for the File Manager folder.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `type_` - The file type for the File Manager file.
    /// * `created_by` - The Mailchimp account user who created the File Manager file.
    /// * `before_created_at` - Restrict the response to files created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `since_created_at` - Restrict the response to files created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `sort_field` - Returns files sorted by the specified field.
    /// * `sort_dir` - Determines the order direction for sorted results.
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
    ///         .list_folder_files(
    ///             &"folder_id".to_string(),
    ///             &ListFolderFilesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 r#type: None,
    ///                 created_by: None,
    ///                 before_created_at: None,
    ///                 since_created_at: None,
    ///                 sort_field: None,
    ///                 sort_dir: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_folder_files(
        &self,
        folder_id: &str,
        request: &ListFolderFilesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListFolderFilesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/file-manager/folders/{}/files", folder_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("type", request.r#type.clone())
                    .string("created_by", request.created_by.clone())
                    .string("before_created_at", request.before_created_at.clone())
                    .string("since_created_at", request.since_created_at.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .build(),
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
    ) -> Result<ListContentResponse, ApiError> {
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

    /// Batch subscribe or unsubscribe list members.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `skip_merge_validation` - If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false.
    /// * `skip_duplicate_check` - If skip_duplicate_check is true, we will ignore duplicates sent in the request when using the batch sub/unsub on the lists endpoint. The status of the first appearance in the request will be saved. This defaults to false.
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
    ///         .batch_subscribe_or_unsubscribe(
    ///             &"list_id".to_string(),
    ///             &BatchSubscribeOrUnsubscribeRequest {
    ///                 members: vec![],
    ///                 skip_merge_validation: None,
    ///                 skip_duplicate_check: None,
    ///                 sync_tags: None,
    ///                 update_existing: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn batch_subscribe_or_unsubscribe(
        &self,
        list_id: &str,
        request: &BatchSubscribeOrUnsubscribeRequest,
        options: Option<RequestOptions>,
    ) -> Result<BatchSubscribeOrUnsubscribeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool(
                        "skip_merge_validation",
                        request.skip_merge_validation.clone(),
                    )
                    .bool("skip_duplicate_check", request.skip_duplicate_check.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a list of abuse complaints for a specific campaign.
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
    ///         .list_abuse_reports(
    ///             &"campaign_id".to_string(),
    ///             &ListAbuseReportsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_abuse_reports(
        &self,
        campaign_id: &str,
        request: &ListAbuseReportsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAbuseReportsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reports/{}/abuse-reports", campaign_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get information about a specific abuse report for a campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `report_id` - The id for the abuse report.
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
    ///         .get_abuse_report(
    ///             &"campaign_id".to_string(),
    ///             &"report_id".to_string(),
    ///             &GetAbuseReportQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_abuse_report(
        &self,
        campaign_id: &str,
        report_id: &str,
        request: &GetAbuseReportQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AbuseComplaint, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reports/{}/abuse-reports/{}", campaign_id, report_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get up to the previous 180 days of daily detailed aggregated activity stats for a list, not including Automation activity.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_activity(
    ///             &"list_id".to_string(),
    ///             &ListActivityQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_activity(
        &self,
        list_id: &str,
        request: &ListActivityQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListActivityResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/activity", list_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a list of the top email clients based on user-agent strings.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
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
    ///         .list_clients(
    ///             &"list_id".to_string(),
    ///             &ListClientsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_clients(
        &self,
        list_id: &str,
        request: &ListClientsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListClientsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/clients", list_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a month-by-month summary of a specific list's growth activity.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `sort_field` - Returns files sorted by the specified field.
    /// * `sort_dir` - Determines the order direction for sorted results.
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
    ///         .list_growth_history(
    ///             &"list_id".to_string(),
    ///             &ListGrowthHistoryQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 sort_field: None,
    ///                 sort_dir: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_growth_history(
        &self,
        list_id: &str,
        request: &ListGrowthHistoryQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListGrowthHistoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/growth-history", list_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a summary of a specific list's growth activity for a specific month and year.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `month` - A specific month of list growth history.
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
    ///         .get_growth_history(
    ///             &"list_id".to_string(),
    ///             &"month".to_string(),
    ///             &GetGrowthHistoryQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_growth_history(
        &self,
        list_id: &str,
        month: &str,
        request: &GetGrowthHistoryQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GrowthHistory, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/growth-history/{}", list_id, month),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get information about a list's interest categories.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `type_` - Restrict results a type of interest group
    /// * `sort_field` - Returns interest categories sorted by the specified field. Defaults to display_order.
    /// * `sort_dir` - Determines the order direction for sorted results.
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
    ///         .list_interest_categories(
    ///             &"list_id".to_string(),
    ///             &ListInterestCategoriesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 r#type: None,
    ///                 sort_field: None,
    ///                 sort_dir: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_interest_categories(
        &self,
        list_id: &str,
        request: &ListInterestCategoriesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListInterestCategoriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/interest-categories", list_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("type", request.r#type.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new interest category.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
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
    ///         .create_interest_category(
    ///             &"list_id".to_string(),
    ///             &CreateInterestCategoryRequest {
    ///                 title: "title".to_string(),
    ///                 r#type: CreateInterestCategoryRequestType::Checkboxes,
    ///                 display_order: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_interest_category(
        &self,
        list_id: &str,
        request: &CreateInterestCategoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<InterestCategory, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/interest-categories", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific interest category.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `interest_category_id` - The unique ID for the interest category.
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
    ///         .get_interest_category(
    ///             &"list_id".to_string(),
    ///             &"interest_category_id".to_string(),
    ///             &GetInterestCategoryQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_interest_category(
        &self,
        list_id: &str,
        interest_category_id: &str,
        request: &GetInterestCategoryQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<InterestCategory, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/lists/{}/interest-categories/{}",
                    list_id, interest_category_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a specific interest category.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `interest_category_id` - The unique ID for the interest category.
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
    ///         .delete_interest_category(
    ///             &"list_id".to_string(),
    ///             &"interest_category_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_interest_category(
        &self,
        list_id: &str,
        interest_category_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/lists/{}/interest-categories/{}",
                    list_id, interest_category_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific interest category.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `interest_category_id` - The unique ID for the interest category.
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
    ///         .update_interest_category(
    ///             &"list_id".to_string(),
    ///             &"interest_category_id".to_string(),
    ///             &UpdateInterestCategoryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_interest_category(
        &self,
        list_id: &str,
        interest_category_id: &str,
        request: &UpdateInterestCategoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<InterestCategory, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/lists/{}/interest-categories/{}",
                    list_id, interest_category_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a list of this category's interests.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `interest_category_id` - The unique ID for the interest category.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_interest_category_interests(
    ///             &"list_id".to_string(),
    ///             &"interest_category_id".to_string(),
    ///             &ListInterestCategoryInterestsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_interest_category_interests(
        &self,
        list_id: &str,
        interest_category_id: &str,
        request: &ListInterestCategoryInterestsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListInterestCategoryInterestsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/lists/{}/interest-categories/{}/interests",
                    list_id, interest_category_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new interest or 'group name' for a specific category.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `interest_category_id` - The unique ID for the interest category.
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
    ///         .create_interest_category_interest(
    ///             &"list_id".to_string(),
    ///             &"interest_category_id".to_string(),
    ///             &CreateInterestCategoryInterestRequest {
    ///                 name: "name".to_string(),
    ///                 display_order: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_interest_category_interest(
        &self,
        list_id: &str,
        interest_category_id: &str,
        request: &CreateInterestCategoryInterestRequest,
        options: Option<RequestOptions>,
    ) -> Result<Interest, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/lists/{}/interest-categories/{}/interests",
                    list_id, interest_category_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get interests or 'group names' for a specific category.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `interest_category_id` - The unique ID for the interest category.
    /// * `interest_id` - The specific interest or 'group name'.
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
    ///         .get_interest_category_interest(
    ///             &"list_id".to_string(),
    ///             &"interest_category_id".to_string(),
    ///             &"interest_id".to_string(),
    ///             &GetInterestCategoryInterestQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_interest_category_interest(
        &self,
        list_id: &str,
        interest_category_id: &str,
        interest_id: &str,
        request: &GetInterestCategoryInterestQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Interest, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/lists/{}/interest-categories/{}/interests/{}",
                    list_id, interest_category_id, interest_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete interests or group names in a specific category.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `interest_category_id` - The unique ID for the interest category.
    /// * `interest_id` - The specific interest or 'group name'.
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
    ///         .delete_interest_category_interest(
    ///             &"list_id".to_string(),
    ///             &"interest_category_id".to_string(),
    ///             &"interest_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_interest_category_interest(
        &self,
        list_id: &str,
        interest_category_id: &str,
        interest_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/lists/{}/interest-categories/{}/interests/{}",
                    list_id, interest_category_id, interest_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update interests or 'group names' for a specific category.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `interest_category_id` - The unique ID for the interest category.
    /// * `interest_id` - The specific interest or 'group name'.
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
    ///         .update_interest_category_interest(
    ///             &"list_id".to_string(),
    ///             &"interest_category_id".to_string(),
    ///             &"interest_id".to_string(),
    ///             &UpdateInterestCategoryInterestRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_interest_category_interest(
        &self,
        list_id: &str,
        interest_category_id: &str,
        interest_id: &str,
        request: &UpdateInterestCategoryInterestRequest,
        options: Option<RequestOptions>,
    ) -> Result<Interest, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/lists/{}/interest-categories/{}/interests/{}",
                    list_id, interest_category_id, interest_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get top open locations for a specific campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_locations(
    ///             &"campaign_id".to_string(),
    ///             &ListLocationsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_locations(
        &self,
        campaign_id: &str,
        request: &ListLocationsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListLocationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reports/{}/locations", campaign_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get information about members in a specific Mailchimp list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `email_type` - The email type.
    /// * `status` - The subscriber's status.
    /// * `since_timestamp_opt` - Restrict results to subscribers who opted-in after the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `before_timestamp_opt` - Restrict results to subscribers who opted-in before the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `since_last_changed` - Restrict results to subscribers whose information changed after the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `before_last_changed` - Restrict results to subscribers whose information changed before the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `unique_email_id` - A unique identifier for the email address across all Mailchimp lists.
    /// * `vip_only` - A filter to return only the list's VIP members. Passing `true` will restrict results to VIP list members, passing `false` will return all list members.
    /// * `interest_category_id` - The unique id for the interest category.
    /// * `interest_ids` - Used to filter list members by interests. Must be accompanied by interest_category_id and interest_match. The value must be a comma separated list of interest ids present for any supplied interest categories.
    /// * `interest_match` - Used to filter list members by interests. Must be accompanied by interest_category_id and interest_ids. "any" will match a member with any of the interest supplied, "all" will only match members with every interest supplied, and "none" will match members without any of the interest supplied.
    /// * `sort_field` - Returns files sorted by the specified field.
    /// * `sort_dir` - Determines the order direction for sorted results.
    /// * `since_last_campaign` - Filter subscribers by those subscribed/unsubscribed/pending/cleaned since last email campaign send. Member status is required to use this filter.
    /// * `unsubscribed_since` - Filter subscribers by those unsubscribed since a specific date. Using any status other than unsubscribed with this filter will result in an error.
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
    ///         .list_members(
    ///             &"list_id".to_string(),
    ///             &ListMembersQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 email_type: None,
    ///                 status: None,
    ///                 since_timestamp_opt: None,
    ///                 before_timestamp_opt: None,
    ///                 since_last_changed: None,
    ///                 before_last_changed: None,
    ///                 unique_email_id: None,
    ///                 vip_only: None,
    ///                 interest_category_id: None,
    ///                 interest_ids: None,
    ///                 interest_match: None,
    ///                 sort_field: None,
    ///                 sort_dir: None,
    ///                 since_last_campaign: None,
    ///                 unsubscribed_since: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_members(
        &self,
        list_id: &str,
        request: &ListMembersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMembersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/members", list_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("email_type", request.email_type.clone())
                    .serialize("status", request.status.clone())
                    .string("since_timestamp_opt", request.since_timestamp_opt.clone())
                    .string("before_timestamp_opt", request.before_timestamp_opt.clone())
                    .string("since_last_changed", request.since_last_changed.clone())
                    .string("before_last_changed", request.before_last_changed.clone())
                    .string("unique_email_id", request.unique_email_id.clone())
                    .bool("vip_only", request.vip_only.clone())
                    .string("interest_category_id", request.interest_category_id.clone())
                    .string("interest_ids", request.interest_ids.clone())
                    .serialize("interest_match", request.interest_match.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .bool("since_last_campaign", request.since_last_campaign.clone())
                    .string("unsubscribed_since", request.unsubscribed_since.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a new member to the list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `skip_merge_validation` - If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false.
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
    ///         .create_member(
    ///             &"list_id".to_string(),
    ///             &CreateMemberRequest {
    ///                 email_address: "email_address".to_string(),
    ///                 status: CreateMemberRequestStatus::Subscribed,
    ///                 skip_merge_validation: None,
    ///                 email_type: None,
    ///                 interests: None,
    ///                 ip_opt: None,
    ///                 ip_signup: None,
    ///                 language: None,
    ///                 location: None,
    ///                 marketing_permissions: None,
    ///                 merge_fields: None,
    ///                 tags: None,
    ///                 timestamp_opt: None,
    ///                 timestamp_signup: None,
    ///                 vip: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_member(
        &self,
        list_id: &str,
        request: &CreateMemberRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMembers, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/members", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool(
                        "skip_merge_validation",
                        request.skip_merge_validation.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Get information about a specific list member, including a currently subscribed, unsubscribed, or bounced member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
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
    ///         .get_member(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &GetMemberQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_member(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &GetMemberQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMembers, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/members/{}", list_id, subscriber_hash),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add or update a list member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    /// * `skip_merge_validation` - If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false.
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
    ///         .upsert_member(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &UpsertMemberRequest {
    ///                 email_address: "email_address".to_string(),
    ///                 skip_merge_validation: None,
    ///                 email_type: None,
    ///                 interests: None,
    ///                 ip_opt: None,
    ///                 ip_signup: None,
    ///                 language: None,
    ///                 location: None,
    ///                 marketing_permissions: None,
    ///                 merge_fields: None,
    ///                 status: None,
    ///                 status_if_new: None,
    ///                 tags: None,
    ///                 timestamp_opt: None,
    ///                 timestamp_signup: None,
    ///                 vip: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upsert_member(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &UpsertMemberRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMembers, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("3.0/lists/{}/members/{}", list_id, subscriber_hash),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool(
                        "skip_merge_validation",
                        request.skip_merge_validation.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Archive a list member. To permanently delete, use the delete-permanent action.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
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
    ///         .delete_member(&"list_id".to_string(), &"subscriber_hash".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_member(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/lists/{}/members/{}", list_id, subscriber_hash),
                None,
                None,
                options,
            )
            .await
    }

    /// Update information for a specific list member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    /// * `skip_merge_validation` - If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false.
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
    ///         .update_member(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &UpdateMemberRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_member(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &UpdateMemberRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMembers, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/lists/{}/members/{}", list_id, subscriber_hash),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool(
                        "skip_merge_validation",
                        request.skip_merge_validation.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Delete all personally identifiable information related to a list member, and remove them from a list. This will make it impossible to re-import the list member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address.
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
    ///         .create_member_action_delete_permanent(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_member_action_delete_permanent(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/lists/{}/members/{}/actions/delete-permanent",
                    list_id, subscriber_hash
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Get the last 50 events of a member's activity on a specific list, including opens, clicks, and unsubscribes.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `action` - A comma seperated list of actions to return.
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
    ///         .list_member_activity(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &ListMemberActivityQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 action: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_member_activity(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &ListMemberActivityQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMemberActivityResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/members/{}/activity", list_id, subscriber_hash),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .serialize_array("action", request.action.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a member's activity on a specific list, including opens, clicks, and unsubscribes.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `activity_filters` - A comma-separated list of activity filters that correspond to a set of activity types, e.g "?activity_filters=open,bounce,click".
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
    ///         .list_member_activity_feed(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &ListMemberActivityFeedQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 activity_filters: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_member_activity_feed(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &ListMemberActivityFeedQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMemberActivityFeedResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/lists/{}/members/{}/activity-feed",
                    list_id, subscriber_hash
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .serialize_array("activity_filters", request.activity_filters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get events for a contact.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_member_events(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &ListMemberEventsQueryRequest {
    ///                 count: None,
    ///                 offset: None,
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_member_events(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &ListMemberEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMemberEventsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/members/{}/events", list_id, subscriber_hash),
                None,
                QueryBuilder::new()
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add an event for a list member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
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
    ///         .create_member_event(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &CreateMemberEventRequest {
    ///                 name: "name".to_string(),
    ///                 is_syncing: None,
    ///                 occurred_at: None,
    ///                 properties: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_member_event(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &CreateMemberEventRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/members/{}/events", list_id, subscriber_hash),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get the last 50 Goal events for a member on a specific list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
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
    ///         .list_member_goals(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &ListMemberGoalsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_member_goals(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &ListMemberGoalsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMemberGoalsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/members/{}/goals", list_id, subscriber_hash),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get recent notes for a specific list member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address.
    /// * `sort_field` - Returns notes sorted by the specified field.
    /// * `sort_dir` - Determines the order direction for sorted results.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_member_notes(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &ListMemberNotesQueryRequest {
    ///                 sort_field: None,
    ///                 sort_dir: None,
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_member_notes(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &ListMemberNotesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMemberNotesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/members/{}/notes", list_id, subscriber_hash),
                None,
                QueryBuilder::new()
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a new note for a specific subscriber.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
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
    ///         .create_member_note(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &CreateMemberNoteRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_member_note(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &CreateMemberNoteRequest,
        options: Option<RequestOptions>,
    ) -> Result<MemberNotes, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/members/{}/notes", list_id, subscriber_hash),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a specific note for a specific list member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    /// * `note_id` - The id for the note.
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
    ///         .get_member_note(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &"note_id".to_string(),
    ///             &GetMemberNoteQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_member_note(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        note_id: &str,
        request: &GetMemberNoteQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MemberNotes, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/lists/{}/members/{}/notes/{}",
                    list_id, subscriber_hash, note_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a specific note for a specific list member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    /// * `note_id` - The id for the note.
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
    ///         .delete_member_note(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &"note_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_member_note(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        note_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/lists/{}/members/{}/notes/{}",
                    list_id, subscriber_hash, note_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific note for a specific list member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    /// * `note_id` - The id for the note.
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
    ///         .update_member_note(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &"note_id".to_string(),
    ///             &UpdateMemberNoteRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_member_note(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        note_id: &str,
        request: &UpdateMemberNoteRequest,
        options: Option<RequestOptions>,
    ) -> Result<MemberNotes, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "3.0/lists/{}/members/{}/notes/{}",
                    list_id, subscriber_hash, note_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get the tags on a list member.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_member_tags(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &ListMemberTagsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_member_tags(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &ListMemberTagsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMemberTagsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/members/{}/tags", list_id, subscriber_hash),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add or remove tags from a list member. If a tag that does not exist is passed in and set as 'active', a new tag will be created.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address.
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
    ///         .create_member_tag(
    ///             &"list_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &CreateMemberTagRequest {
    ///                 tags: vec![CreateMemberTagRequestTagsItem {
    ///                     name: "name".to_string(),
    ///                     status: CreateMemberTagRequestTagsItemStatus::Inactive,
    ///                 }],
    ///                 is_syncing: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_member_tag(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        request: &CreateMemberTagRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/members/{}/tags", list_id, subscriber_hash),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a list of all merge fields for an audience.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `type_` - The merge field type.
    /// * `required` - Whether it's a required merge field.
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
    ///         .list_merge_fields(
    ///             &"list_id".to_string(),
    ///             &ListMergeFieldsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 r#type: None,
    ///                 required: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_merge_fields(
        &self,
        list_id: &str,
        request: &ListMergeFieldsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMergeFieldsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/merge-fields", list_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("type", request.r#type.clone())
                    .bool("required", request.required.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a new merge field for a specific audience.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
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
    ///         .create_merge_field(
    ///             &"list_id".to_string(),
    ///             &CreateMergeFieldRequest {
    ///                 name: "name".to_string(),
    ///                 r#type: CreateMergeFieldRequestType::Text,
    ///                 default_value: None,
    ///                 display_order: None,
    ///                 help_text: None,
    ///                 options: None,
    ///                 public: None,
    ///                 required: None,
    ///                 tag: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_merge_field(
        &self,
        list_id: &str,
        request: &CreateMergeFieldRequest,
        options: Option<RequestOptions>,
    ) -> Result<MergeField, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/merge-fields", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific merge field.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `merge_id` - The id for the merge field.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
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
    ///         .get_merge_field(
    ///             &"list_id".to_string(),
    ///             &"merge_id".to_string(),
    ///             &GetMergeFieldQueryRequest {
    ///                 exclude_fields: vec![],
    ///                 fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_merge_field(
        &self,
        list_id: &str,
        merge_id: &str,
        request: &GetMergeFieldQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MergeField, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/merge-fields/{}", list_id, merge_id),
                None,
                QueryBuilder::new()
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .string_array("fields", request.fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a specific merge field.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `merge_id` - The id for the merge field.
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
    ///         .delete_merge_field(&"list_id".to_string(), &"merge_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_merge_field(
        &self,
        list_id: &str,
        merge_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/lists/{}/merge-fields/{}", list_id, merge_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific merge field.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `merge_id` - The id for the merge field.
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
    ///         .update_merge_field(
    ///             &"list_id".to_string(),
    ///             &"merge_id".to_string(),
    ///             &UpdateMergeFieldRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_merge_field(
        &self,
        list_id: &str,
        merge_id: &str,
        request: &UpdateMergeFieldRequest,
        options: Option<RequestOptions>,
    ) -> Result<MergeField, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/lists/{}/merge-fields/{}", list_id, merge_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about all available segments for a specific list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `type_` - Limit results based on segment type.
    /// * `since_created_at` - Restrict results to segments created after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `before_created_at` - Restrict results to segments created before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `include_cleaned` - Include cleaned members in response
    /// * `include_transactional` - Include transactional members in response
    /// * `include_unsubscribed` - Include unsubscribed members in response
    /// * `since_updated_at` - Restrict results to segments update after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `before_updated_at` - Restrict results to segments update before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `exclude_type` - Exclude results based on segment type. For example, use `exclude_type=static` to exclude tags from the response.
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
    ///         .list_segments(
    ///             &"list_id".to_string(),
    ///             &ListSegmentsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 r#type: None,
    ///                 since_created_at: None,
    ///                 before_created_at: None,
    ///                 include_cleaned: None,
    ///                 include_transactional: None,
    ///                 include_unsubscribed: None,
    ///                 since_updated_at: None,
    ///                 before_updated_at: None,
    ///                 exclude_type: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_segments(
        &self,
        list_id: &str,
        request: &ListSegmentsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSegmentsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/segments", list_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("type", request.r#type.clone())
                    .string("since_created_at", request.since_created_at.clone())
                    .string("before_created_at", request.before_created_at.clone())
                    .bool("include_cleaned", request.include_cleaned.clone())
                    .bool(
                        "include_transactional",
                        request.include_transactional.clone(),
                    )
                    .bool("include_unsubscribed", request.include_unsubscribed.clone())
                    .string("since_updated_at", request.since_updated_at.clone())
                    .string("before_updated_at", request.before_updated_at.clone())
                    .serialize("exclude_type", request.exclude_type.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new segment in a specific list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
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
    ///         .create_segment(
    ///             &"list_id".to_string(),
    ///             &CreateSegmentRequest {
    ///                 name: "name".to_string(),
    ///                 options: None,
    ///                 static_segment: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_segment(
        &self,
        list_id: &str,
        request: &CreateSegmentRequest,
        options: Option<RequestOptions>,
    ) -> Result<List, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/segments", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific segment.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `segment_id` - The unique id for the segment.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `include_cleaned` - Include cleaned members in response
    /// * `include_transactional` - Include transactional members in response
    /// * `include_unsubscribed` - Include unsubscribed members in response
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
    ///         .get_segment(
    ///             &"list_id".to_string(),
    ///             &"segment_id".to_string(),
    ///             &GetSegmentQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 include_cleaned: None,
    ///                 include_transactional: None,
    ///                 include_unsubscribed: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_segment(
        &self,
        list_id: &str,
        segment_id: &str,
        request: &GetSegmentQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<List, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/segments/{}", list_id, segment_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .bool("include_cleaned", request.include_cleaned.clone())
                    .bool(
                        "include_transactional",
                        request.include_transactional.clone(),
                    )
                    .bool("include_unsubscribed", request.include_unsubscribed.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Batch add/remove list members to static segment
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `segment_id` - The unique id for the segment.
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
    ///         .batch_add_or_remove_members(
    ///             &"list_id".to_string(),
    ///             &"segment_id".to_string(),
    ///             &BatchAddOrRemoveMembersRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn batch_add_or_remove_members(
        &self,
        list_id: &str,
        segment_id: &str,
        request: &BatchAddOrRemoveMembersRequest,
        options: Option<RequestOptions>,
    ) -> Result<BatchAddOrRemoveMembersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/segments/{}", list_id, segment_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a specific segment in a list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `segment_id` - The unique id for the segment.
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
    ///         .delete_segment(&"list_id".to_string(), &"segment_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_segment(
        &self,
        list_id: &str,
        segment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/lists/{}/segments/{}", list_id, segment_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific segment in a list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `segment_id` - The unique id for the segment.
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
    ///         .update_segment(
    ///             &"list_id".to_string(),
    ///             &"segment_id".to_string(),
    ///             &UpdateSegmentRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_segment(
        &self,
        list_id: &str,
        segment_id: &str,
        request: &UpdateSegmentRequest,
        options: Option<RequestOptions>,
    ) -> Result<List, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/lists/{}/segments/{}", list_id, segment_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about members in a saved segment.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `segment_id` - The unique id for the segment.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `include_cleaned` - Include cleaned members in response
    /// * `include_transactional` - Include transactional members in response
    /// * `include_unsubscribed` - Include unsubscribed members in response
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
    ///         .list_segment_members(
    ///             &"list_id".to_string(),
    ///             &"segment_id".to_string(),
    ///             &ListSegmentMembersQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 include_cleaned: None,
    ///                 include_transactional: None,
    ///                 include_unsubscribed: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_segment_members(
        &self,
        list_id: &str,
        segment_id: &str,
        request: &ListSegmentMembersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSegmentMembersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/segments/{}/members", list_id, segment_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .bool("include_cleaned", request.include_cleaned.clone())
                    .bool(
                        "include_transactional",
                        request.include_transactional.clone(),
                    )
                    .bool("include_unsubscribed", request.include_unsubscribed.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a member to a static segment.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `segment_id` - The unique id for the segment.
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
    ///         .create_segment_member(
    ///             &"list_id".to_string(),
    ///             &"segment_id".to_string(),
    ///             &CreateSegmentMemberRequest {
    ///                 email_address: "email_address".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_segment_member(
        &self,
        list_id: &str,
        segment_id: &str,
        request: &CreateSegmentMemberRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListsSegmentsMembers, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/segments/{}/members", list_id, segment_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Remove a member from the specified static segment.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `segment_id` - The unique id for the segment.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address.
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
    ///         .delete_segment_member(
    ///             &"list_id".to_string(),
    ///             &"segment_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_segment_member(
        &self,
        list_id: &str,
        segment_id: &str,
        subscriber_hash: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "3.0/lists/{}/segments/{}/members/{}",
                    list_id, segment_id, subscriber_hash
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Get signup forms for a specific list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
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
    ///     client.list_signup_forms(&"list_id".to_string(), None).await;
    /// }
    /// ```
    pub async fn list_signup_forms(
        &self,
        list_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListSignupFormsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/signup-forms", list_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Customize a list's default signup form.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
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
    ///         .create_signup_form(
    ///             &"list_id".to_string(),
    ///             &CreateSignupFormRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_signup_form(
        &self,
        list_id: &str,
        request: &CreateSignupFormRequest,
        options: Option<RequestOptions>,
    ) -> Result<SignupForm, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/signup-forms", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get reports for surveys.
    ///
    /// # Arguments
    ///
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_surveys(
    ///             &ListSurveysQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_surveys(
        &self,
        request: &ListSurveysQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSurveysResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/reporting/surveys",
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a draft survey for an audience.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
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
    ///         .create_survey(
    ///             &"list_id".to_string(),
    ///             &CreateSurveyRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_survey(
        &self,
        list_id: &str,
        request: &CreateSurveyRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/surveys", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get report for a survey.
    ///
    /// # Arguments
    ///
    /// * `survey_id` - The ID of the survey.
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
    ///         .get_survey(
    ///             &"survey_id".to_string(),
    ///             &GetSurveyQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_survey(
        &self,
        survey_id: &str,
        request: &GetSurveyQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSurveyResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reporting/surveys/{}", survey_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a survey.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `survey_id` - The ID of the survey.
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
    ///         .delete_survey(&"list_id".to_string(), &"survey_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_survey(
        &self,
        list_id: &str,
        survey_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/lists/{}/surveys/{}", list_id, survey_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a survey. When sections is provided, send the complete section list in display order. Any existing section not included is deleted.
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
    ///         .update_survey(
    ///             &"list_id".to_string(),
    ///             &"survey_id".to_string(),
    ///             &UpdateSurveyRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_survey(
        &self,
        list_id: &str,
        survey_id: &str,
        request: &UpdateSurveyRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/lists/{}/surveys/{}", list_id, survey_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
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

    /// Replicate a survey.
    ///
    /// # Arguments
    ///
    /// * `list_id_path_param` - The unique ID for the list.
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
    ///         .create_list_survey_action_replicate(
    ///             &"list_id".to_string(),
    ///             &"survey_id".to_string(),
    ///             &CreateListSurveyActionReplicateRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_list_survey_action_replicate(
        &self,
        list_id_path_param: &str,
        survey_id: &str,
        request: &CreateListSurveyActionReplicateRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "3.0/lists/{}/surveys/{}/actions/replicate",
                    list_id_path_param, survey_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Search for tags on a list by name. If no name is provided, will return all tags on the list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `name` - The search query used to filter tags.  The search query will be compared to each tag as a prefix, so all tags that have a name starting with this field will be returned.
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
    ///         .list_tag_search(
    ///             &"list_id".to_string(),
    ///             &ListTagSearchQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_tag_search(
        &self,
        list_id: &str,
        request: &ListTagSearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListTagSearchResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/tag-search", list_id),
                None,
                QueryBuilder::new()
                    .string("name", request.name.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get information about all webhooks for a specific list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
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
    ///     client.list_webhooks(&"list_id".to_string(), None).await;
    /// }
    /// ```
    pub async fn list_webhooks(
        &self,
        list_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListWebhooksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/webhooks", list_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Create a new webhook for a specific list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
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
    ///         .create_webhook(
    ///             &"list_id".to_string(),
    ///             &AddWebhook {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_webhook(
        &self,
        list_id: &str,
        request: &AddWebhook,
        options: Option<RequestOptions>,
    ) -> Result<CreateWebhookResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/lists/{}/webhooks", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific webhook.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `webhook_id` - The webhook's id.
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
    ///         .get_webhook(&"list_id".to_string(), &"webhook_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_webhook(
        &self,
        list_id: &str,
        webhook_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListWebhooks, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/lists/{}/webhooks/{}", list_id, webhook_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a specific webhook in a list.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `webhook_id` - The webhook's id.
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
    ///         .delete_webhook(&"list_id".to_string(), &"webhook_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_webhook(
        &self,
        list_id: &str,
        webhook_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/lists/{}/webhooks/{}", list_id, webhook_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the settings for an existing webhook.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The unique ID for the list.
    /// * `webhook_id` - The webhook's id.
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
    ///         .update_webhook(
    ///             &"list_id".to_string(),
    ///             &"webhook_id".to_string(),
    ///             &AddWebhook {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_webhook(
        &self,
        list_id: &str,
        webhook_id: &str,
        request: &AddWebhook,
        options: Option<RequestOptions>,
    ) -> Result<ListWebhooks, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/lists/{}/webhooks/{}", list_id, webhook_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get reports of Facebook ads.
    ///
    /// # Arguments
    ///
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `sort_field` - Returns files sorted by the specified field.
    /// * `sort_dir` - Determines the order direction for sorted results.
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
    ///         .list_facebook_ads(
    ///             &ListFacebookAdsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 sort_field: None,
    ///                 sort_dir: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_facebook_ads(
        &self,
        request: &ListFacebookAdsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListFacebookAdsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/reporting/facebook-ads",
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get report of a Facebook ad.
    ///
    /// # Arguments
    ///
    /// * `outreach_id` - The outreach id.
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
    ///         .get_facebook_ad(
    ///             &"outreach_id".to_string(),
    ///             &GetFacebookAdQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_facebook_ad(
        &self,
        outreach_id: &str,
        request: &GetFacebookAdQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReportingFacebookAd, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reporting/facebook-ads/{}", outreach_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get breakdown of product activity for an outreach.
    ///
    /// # Arguments
    ///
    /// * `outreach_id` - The outreach id.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `sort_field` - Returns files sorted by the specified field.
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
    ///         .list_facebook_ad_ecommerce_product_activity(
    ///             &"outreach_id".to_string(),
    ///             &ListFacebookAdEcommerceProductActivityQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 sort_field: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_facebook_ad_ecommerce_product_activity(
        &self,
        outreach_id: &str,
        request: &ListFacebookAdEcommerceProductActivityQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListFacebookAdEcommerceProductActivityResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/reporting/facebook-ads/{}/ecommerce-product-activity",
                    outreach_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get reports of landing pages.
    ///
    /// # Arguments
    ///
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_landing_pages(
    ///             &ListLandingPagesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_landing_pages(
        &self,
        request: &ListLandingPagesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListLandingPagesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/reporting/landing-pages",
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get report of a landing page.
    ///
    /// # Arguments
    ///
    /// * `outreach_id` - The outreach id.
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
    ///         .get_landing_page(
    ///             &"outreach_id".to_string(),
    ///             &GetLandingPageQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_landing_page(
        &self,
        outreach_id: &str,
        request: &GetLandingPageQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<LandingPageReport, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reporting/landing-pages/{}", outreach_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get reports for survey questions.
    ///
    /// # Arguments
    ///
    /// * `survey_id` - The ID of the survey.
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
    ///         .list_survey_questions(
    ///             &"survey_id".to_string(),
    ///             &ListSurveyQuestionsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_survey_questions(
        &self,
        survey_id: &str,
        request: &ListSurveyQuestionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSurveyQuestionsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reporting/surveys/{}/questions", survey_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get report for a survey question.
    ///
    /// # Arguments
    ///
    /// * `survey_id` - The ID of the survey.
    /// * `question_id` - The ID of the survey question.
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
    ///         .get_survey_question(
    ///             &"survey_id".to_string(),
    ///             &"question_id".to_string(),
    ///             &GetSurveyQuestionQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_survey_question(
        &self,
        survey_id: &str,
        question_id: &str,
        request: &GetSurveyQuestionQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SurveyQuestionReport, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/reporting/surveys/{}/questions/{}",
                    survey_id, question_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get answers for a survey question.
    ///
    /// # Arguments
    ///
    /// * `survey_id` - The ID of the survey.
    /// * `question_id` - The ID of the survey question.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `respondent_familiarity_is` - Filter survey responses by familiarity of the respondents.
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
    ///         .list_survey_question_answers(
    ///             &"survey_id".to_string(),
    ///             &"question_id".to_string(),
    ///             &ListSurveyQuestionAnswersQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 respondent_familiarity_is: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_survey_question_answers(
        &self,
        survey_id: &str,
        question_id: &str,
        request: &ListSurveyQuestionAnswersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSurveyQuestionAnswersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/reporting/surveys/{}/questions/{}/answers",
                    survey_id, question_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .serialize(
                        "respondent_familiarity_is",
                        request.respondent_familiarity_is.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Get responses to a survey.
    ///
    /// # Arguments
    ///
    /// * `survey_id` - The ID of the survey.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `answered_question` - The ID of the question that was answered.
    /// * `chose_answer` - The ID of the option chosen to filter responses on.
    /// * `respondent_familiarity_is` - Filter survey responses by familiarity of the respondents.
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
    ///         .list_survey_responses(
    ///             &"survey_id".to_string(),
    ///             &ListSurveyResponsesQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 answered_question: None,
    ///                 chose_answer: None,
    ///                 respondent_familiarity_is: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_survey_responses(
        &self,
        survey_id: &str,
        request: &ListSurveyResponsesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSurveyResponsesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reporting/surveys/{}/responses", survey_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("answered_question", request.answered_question.clone())
                    .string("chose_answer", request.chose_answer.clone())
                    .serialize(
                        "respondent_familiarity_is",
                        request.respondent_familiarity_is.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Get a single survey response.
    ///
    /// # Arguments
    ///
    /// * `survey_id` - The ID of the survey.
    /// * `response_id` - The ID of the survey response.
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
    ///         .get_survey_respons(&"survey_id".to_string(), &"response_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_survey_respons(
        &self,
        survey_id: &str,
        response_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetSurveyResponsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/reporting/surveys/{}/responses/{}",
                    survey_id, response_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Get feedback based on a campaign's statistics. Advice feedback is based on campaign stats like opens, clicks, unsubscribes, bounces, and more.
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
    ///         .list_advice(
    ///             &"campaign_id".to_string(),
    ///             &ListAdviceQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_advice(
        &self,
        campaign_id: &str,
        request: &ListAdviceQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAdviceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reports/{}/advice", campaign_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get information about clicks on specific links in your Mailchimp campaigns.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `sort_field` - Returns click reports sorted by the specified field.
    /// * `sort_dir` - Determines the order direction for sorted results.
    /// * `filter_bots` - When true, exclude automated bot clicks so the returned click counts reflect human clicks only, matching the in-app Recipient Activity view. Filtering changes a link's counts, but never removes a link from the response. Defaults to false (all clicks).
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
    ///         .list_click_details(
    ///             &"campaign_id".to_string(),
    ///             &ListClickDetailsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 sort_field: None,
    ///                 sort_dir: None,
    ///                 filter_bots: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_click_details(
        &self,
        campaign_id: &str,
        request: &ListClickDetailsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListClickDetailsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reports/{}/click-details", campaign_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .bool("filter_bots", request.filter_bots.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get click details for a specific link in a campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `link_id` - The id for the link.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `filter_bots` - When true, exclude automated bot clicks so the returned click counts reflect human clicks only, matching the in-app Recipient Activity view. Filtering changes a link's counts, but never removes a link from the response. Defaults to false (all clicks).
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
    ///         .get_click_detail(
    ///             &"campaign_id".to_string(),
    ///             &"link_id".to_string(),
    ///             &GetClickDetailQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 filter_bots: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_click_detail(
        &self,
        campaign_id: &str,
        link_id: &str,
        request: &GetClickDetailQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ClickDetailReport, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reports/{}/click-details/{}", campaign_id, link_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .bool("filter_bots", request.filter_bots.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get information about list members who clicked on a specific link in a campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `link_id` - The id for the link.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_click_detail_members(
    ///             &"campaign_id".to_string(),
    ///             &"link_id".to_string(),
    ///             &ListClickDetailMembersQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_click_detail_members(
        &self,
        campaign_id: &str,
        link_id: &str,
        request: &ListClickDetailMembersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListClickDetailMembersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/reports/{}/click-details/{}/members",
                    campaign_id, link_id
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get information about a specific subscriber who clicked a link in a specific campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `link_id` - The id for the link.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address.
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
    ///         .get_click_detail_member(
    ///             &"campaign_id".to_string(),
    ///             &"link_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &GetClickDetailMemberQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_click_detail_member(
        &self,
        campaign_id: &str,
        link_id: &str,
        subscriber_hash: &str,
        request: &GetClickDetailMemberQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ClickDetailMember, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/reports/{}/click-details/{}/members/{}",
                    campaign_id, link_id, subscriber_hash
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get statistics for the top-performing email domains in a campaign.
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
    ///         .list_domain_performance(
    ///             &"campaign_id".to_string(),
    ///             &ListDomainPerformanceQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_domain_performance(
        &self,
        campaign_id: &str,
        request: &ListDomainPerformanceQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListDomainPerformanceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reports/{}/domain-performance", campaign_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get breakdown of product activity for a campaign
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `sort_field` - Returns files sorted by the specified field.
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
    ///         .list_ecommerce_product_activity(
    ///             &"campaign_id".to_string(),
    ///             &ListEcommerceProductActivityQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 sort_field: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_ecommerce_product_activity(
        &self,
        campaign_id: &str,
        request: &ListEcommerceProductActivityQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListEcommerceProductActivityResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reports/{}/ecommerce-product-activity", campaign_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a summary of social activity for the campaign, tracked by EepURL.
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
    ///         .list_eepurl(
    ///             &"campaign_id".to_string(),
    ///             &ListEepurlQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_eepurl(
        &self,
        campaign_id: &str,
        request: &ListEepurlQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListEepurlResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reports/{}/eepurl", campaign_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a list of member's subscriber activity in a specific campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `since` - Restrict results to email activity events that occur after a specific time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `filter_bots` - When true, exclude automated bot and Apple Mail Privacy Protection (MPP) proxy activity so the returned activity reflects human-only opens and clicks, matching the in-app Recipient Activity view. Filtering removes events from a member's activity, but never removes the member from the response. Defaults to false (all activity).
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
    ///         .list_email_activity(
    ///             &"campaign_id".to_string(),
    ///             &ListEmailActivityQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 since: None,
    ///                 filter_bots: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_email_activity(
        &self,
        campaign_id: &str,
        request: &ListEmailActivityQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListEmailActivityResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reports/{}/email-activity", campaign_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("since", request.since.clone())
                    .bool("filter_bots", request.filter_bots.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a specific list member's activity in a campaign including opens, clicks, and bounces.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `since` - Restrict results to email activity events that occur after a specific time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `filter_bots` - When true, exclude automated bot and Apple Mail Privacy Protection (MPP) proxy activity so the returned activity reflects human-only opens and clicks, matching the in-app Recipient Activity view. Filtering removes events from a member's activity, but never removes the member from the response. Defaults to false (all activity).
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
    ///         .get_email_activity(
    ///             &"campaign_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &GetEmailActivityQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 since: None,
    ///                 filter_bots: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_email_activity(
        &self,
        campaign_id: &str,
        subscriber_hash: &str,
        request: &GetEmailActivityQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<EmailActivity, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/reports/{}/email-activity/{}",
                    campaign_id, subscriber_hash
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .string("since", request.since.clone())
                    .bool("filter_bots", request.filter_bots.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get detailed information about any campaign emails that were opened by a list member.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `since` - Restrict results to campaign open events that occur after a specific time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `sort_field` - Returns open reports sorted by the specified field.
    /// * `sort_dir` - Determines the order direction for sorted results.
    /// * `filter_bots` - When true, exclude automated (proxy/bot) opens so the returned open counts reflect human opens only, matching the in-app Recipient Activity view. A member whose opens are all automated is excluded from the human-only view. Defaults to false (all opens).
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
    ///         .list_open_details(
    ///             &"campaign_id".to_string(),
    ///             &ListOpenDetailsQueryRequest {
    ///                 since: Some("2016-04-12 12:00:00".to_string()),
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 sort_field: None,
    ///                 sort_dir: None,
    ///                 filter_bots: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_open_details(
        &self,
        campaign_id: &str,
        request: &ListOpenDetailsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListOpenDetailsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reports/{}/open-details", campaign_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("since", request.since.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .bool("filter_bots", request.filter_bots.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get information about a specific subscriber who opened a campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `filter_bots` - When true, exclude automated (proxy/bot) opens so the returned open counts reflect human opens only, matching the in-app Recipient Activity view. A member whose opens are all automated is excluded from the human-only view. Defaults to false (all opens).
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
    ///         .get_open_detail(
    ///             &"campaign_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &GetOpenDetailQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 filter_bots: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_open_detail(
        &self,
        campaign_id: &str,
        subscriber_hash: &str,
        request: &GetOpenDetailQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<OpenActivity, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/reports/{}/open-details/{}",
                    campaign_id, subscriber_hash
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .bool("filter_bots", request.filter_bots.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get information about campaign recipients.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_sent_to(
    ///             &"campaign_id".to_string(),
    ///             &ListSentToQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_sent_to(
        &self,
        campaign_id: &str,
        request: &ListSentToQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSentToResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reports/{}/sent-to", campaign_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get information about a specific campaign recipient.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address.
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
    ///         .get_sent_to(
    ///             &"campaign_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &GetSentToQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_sent_to(
        &self,
        campaign_id: &str,
        subscriber_hash: &str,
        request: &GetSentToQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SentTo, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reports/{}/sent-to/{}", campaign_id, subscriber_hash),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a list of reports with child campaigns for a specific parent campaign.
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
    ///         .list_sub_reports(
    ///             &"campaign_id".to_string(),
    ///             &ListSubReportsQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_sub_reports(
        &self,
        campaign_id: &str,
        request: &ListSubReportsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSubReportsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reports/{}/sub-reports", campaign_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get information about members who have unsubscribed from a specific campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .list_unsubscribed(
    ///             &"campaign_id".to_string(),
    ///             &ListUnsubscribedQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_unsubscribed(
        &self,
        campaign_id: &str,
        request: &ListUnsubscribedQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListUnsubscribedResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/reports/{}/unsubscribed", campaign_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get information about a specific list member who unsubscribed from a campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - The unique id for the campaign.
    /// * `subscriber_hash` - The MD5 hash of the lowercase version of the list member's email address.
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
    ///         .get_unsubscribed(
    ///             &"campaign_id".to_string(),
    ///             &"subscriber_hash".to_string(),
    ///             &GetUnsubscribedQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_unsubscribed(
        &self,
        campaign_id: &str,
        subscriber_hash: &str,
        request: &GetUnsubscribedQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Unsubscribes, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "3.0/reports/{}/unsubscribed/{}",
                    campaign_id, subscriber_hash
                ),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get the sections that you can edit in a template, including each section's default content.
    ///
    /// # Arguments
    ///
    /// * `template_id` - The unique id for the template.
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
    ///         .list_default_content(
    ///             &"template_id".to_string(),
    ///             &ListDefaultContentQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_default_content(
        &self,
        template_id: &str,
        request: &ListDefaultContentQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListDefaultContentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/templates/{}/default-content", template_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Verify a domain for sending.
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The domain name.
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
    ///         .create_action_verify(
    ///             &"domain_name".to_string(),
    ///             &CreateActionVerifyRequest {
    ///                 code: "code".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_verify(
        &self,
        domain_name: &str,
        request: &CreateActionVerifyRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateActionVerifyResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/verified-domains/{}/actions/verify", domain_name),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}

pub use audiences::AudiencesClient;
