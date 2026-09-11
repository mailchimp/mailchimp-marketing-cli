use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CustomerJourneysClient {
    pub http_client: HttpClient,
}

impl CustomerJourneysClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
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
    ///         .customer_journeys
    ///         .create_journey_step_action_trigger(
    ///             1,
    ///             1,
    ///             &CreateJourneyStepActionTriggerCustomerJourneysRequest {
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
        request: &CreateJourneyStepActionTriggerCustomerJourneysRequest,
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
}
