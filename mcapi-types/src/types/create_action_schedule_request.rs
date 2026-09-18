pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateActionScheduleRequest {
    /// The UTC date and time to schedule the campaign.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub schedule_time: DateTime<FixedOffset>,
}

impl CreateActionScheduleRequest {
    pub fn builder() -> CreateActionScheduleRequestBuilder {
        <CreateActionScheduleRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateActionScheduleRequestBuilder {
    schedule_time: Option<DateTime<FixedOffset>>,
}

impl CreateActionScheduleRequestBuilder {
    pub fn schedule_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.schedule_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateActionScheduleRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`schedule_time`](CreateActionScheduleRequestBuilder::schedule_time)
    pub fn build(self) -> Result<CreateActionScheduleRequest, BuildError> {
        Ok(CreateActionScheduleRequest {
            schedule_time: self.schedule_time.ok_or_else(|| BuildError::missing_field("schedule_time"))?,
        })
    }
}

