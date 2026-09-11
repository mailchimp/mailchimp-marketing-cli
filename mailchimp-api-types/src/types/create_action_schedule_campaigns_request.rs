pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateActionScheduleCampaignsRequest {
    /// Choose whether the campaign should use [Batch Delivery](https://mailchimp.com/help/schedule-batch-delivery/). Cannot be set to `true` for campaigns using [Timewarp](https://mailchimp.com/help/use-timewarp/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_delivery: Option<CreateActionScheduleCampaignsRequestBatchDelivery>,
    /// The UTC date and time to schedule the campaign for delivery in ISO 8601 format. Campaigns may only be scheduled to send on the quarter-hour (:00, :15, :30, :45).
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub schedule_time: DateTime<FixedOffset>,
    /// Choose whether the campaign should use [Timewarp](https://mailchimp.com/help/use-timewarp/) when sending. Campaigns scheduled with Timewarp are localized based on the recipients' time zones. For example, a Timewarp campaign with a `schedule_time` of 13:00 will be sent to each recipient at 1:00pm in their local time. Cannot be set to `true` for campaigns using [Batch Delivery](https://mailchimp.com/help/schedule-batch-delivery/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timewarp: Option<bool>,
}

impl CreateActionScheduleCampaignsRequest {
    pub fn builder() -> CreateActionScheduleCampaignsRequestBuilder {
        <CreateActionScheduleCampaignsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateActionScheduleCampaignsRequestBuilder {
    batch_delivery: Option<CreateActionScheduleCampaignsRequestBatchDelivery>,
    schedule_time: Option<DateTime<FixedOffset>>,
    timewarp: Option<bool>,
}

impl CreateActionScheduleCampaignsRequestBuilder {
    pub fn batch_delivery(mut self, value: CreateActionScheduleCampaignsRequestBatchDelivery) -> Self {
        self.batch_delivery = Some(value);
        self
    }

    pub fn schedule_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.schedule_time = Some(value);
        self
    }

    pub fn timewarp(mut self, value: bool) -> Self {
        self.timewarp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateActionScheduleCampaignsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`schedule_time`](CreateActionScheduleCampaignsRequestBuilder::schedule_time)
    pub fn build(self) -> Result<CreateActionScheduleCampaignsRequest, BuildError> {
        Ok(CreateActionScheduleCampaignsRequest {
            batch_delivery: self.batch_delivery,
            schedule_time: self.schedule_time.ok_or_else(|| BuildError::missing_field("schedule_time"))?,
            timewarp: self.timewarp,
        })
    }
}

