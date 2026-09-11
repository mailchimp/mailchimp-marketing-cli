pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateActionScheduleSmsCampaignsRequest {
    /// The UTC date and time to schedule the campaign.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub schedule_time: DateTime<FixedOffset>,
}

impl CreateActionScheduleSmsCampaignsRequest {
    pub fn builder() -> CreateActionScheduleSmsCampaignsRequestBuilder {
        <CreateActionScheduleSmsCampaignsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateActionScheduleSmsCampaignsRequestBuilder {
    schedule_time: Option<DateTime<FixedOffset>>,
}

impl CreateActionScheduleSmsCampaignsRequestBuilder {
    pub fn schedule_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.schedule_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateActionScheduleSmsCampaignsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`schedule_time`](CreateActionScheduleSmsCampaignsRequestBuilder::schedule_time)
    pub fn build(self) -> Result<CreateActionScheduleSmsCampaignsRequest, BuildError> {
        Ok(CreateActionScheduleSmsCampaignsRequest {
            schedule_time: self.schedule_time.ok_or_else(|| BuildError::missing_field("schedule_time"))?,
        })
    }
}

