pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The tracking settings applied to this landing page.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateRequestTracking {
    /// Google offers restricted data processing in connection with the California Consumer Privacy Act (CCPA) to restrict how Google uses certain identifiers and other data processed in the provision of its services. You can learn more about Google's restricted data processing within Google Ads [here](https://privacy.google.com/businesses/rdp/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_restricted_data_processing: Option<bool>,
    /// Use cookies to track unique visitors and calculate overall conversion rate. Learn more [here](https://mailchimp.com/help/use-track-mailchimp/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_with_mailchimp: Option<bool>,
}

impl UpdateRequestTracking {
    pub fn builder() -> UpdateRequestTrackingBuilder {
        <UpdateRequestTrackingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateRequestTrackingBuilder {
    enable_restricted_data_processing: Option<bool>,
    track_with_mailchimp: Option<bool>,
}

impl UpdateRequestTrackingBuilder {
    pub fn enable_restricted_data_processing(mut self, value: bool) -> Self {
        self.enable_restricted_data_processing = Some(value);
        self
    }

    pub fn track_with_mailchimp(mut self, value: bool) -> Self {
        self.track_with_mailchimp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateRequestTracking`].
    pub fn build(self) -> Result<UpdateRequestTracking, BuildError> {
        Ok(UpdateRequestTracking {
            enable_restricted_data_processing: self.enable_restricted_data_processing,
            track_with_mailchimp: self.track_with_mailchimp,
        })
    }
}
