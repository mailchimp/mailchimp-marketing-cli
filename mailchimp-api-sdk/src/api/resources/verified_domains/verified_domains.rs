use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct VerifiedDomainsClient {
    pub http_client: HttpClient,
}

impl VerifiedDomainsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
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
    /// use mailchimp_api_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpApiClient::new(config).expect("Failed to build client");
    ///     client.verified_domains.list(None).await;
    /// }
    /// ```
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<ListVerifiedDomainsResponse, ApiError> {
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
    ///         .verified_domains
    ///         .create(
    ///             &CreateVerifiedDomainsRequest {
    ///                 verification_email: "verification_email".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateVerifiedDomainsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateVerifiedDomainsResponse, ApiError> {
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
    ///         .verified_domains
    ///         .get(&"domain_name".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        domain_name: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetVerifiedDomainsResponse, ApiError> {
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
    ///         .verified_domains
    ///         .delete(&"domain_name".to_string(), None)
    ///         .await;
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
    ///         .verified_domains
    ///         .create_action_verify(
    ///             &"domain_name".to_string(),
    ///             &CreateActionVerifyVerifiedDomainsRequest {
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
        request: &CreateActionVerifyVerifiedDomainsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateActionVerifyVerifiedDomainsResponse, ApiError> {
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
