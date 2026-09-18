pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Choose whether the campaign should use [Batch Delivery](https://mailchimp.com/help/schedule-batch-delivery/). Cannot be set to `true` for campaigns using [Timewarp](https://mailchimp.com/help/use-timewarp/).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateActionScheduleRequestBatchDelivery {
    /// The number of batches for the campaign send.
    #[serde(default)]
    pub batch_count: i64,
    /// The delay, in minutes, between batches.
    #[serde(default)]
    pub batch_delay: i64,
}

impl CreateActionScheduleRequestBatchDelivery {
    pub fn builder() -> CreateActionScheduleRequestBatchDeliveryBuilder {
        <CreateActionScheduleRequestBatchDeliveryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateActionScheduleRequestBatchDeliveryBuilder {
    batch_count: Option<i64>,
    batch_delay: Option<i64>,
}

impl CreateActionScheduleRequestBatchDeliveryBuilder {
    pub fn batch_count(mut self, value: i64) -> Self {
        self.batch_count = Some(value);
        self
    }

    pub fn batch_delay(mut self, value: i64) -> Self {
        self.batch_delay = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateActionScheduleRequestBatchDelivery`].
    /// This method will fail if any of the following fields are not set:
    /// - [`batch_count`](CreateActionScheduleRequestBatchDeliveryBuilder::batch_count)
    /// - [`batch_delay`](CreateActionScheduleRequestBatchDeliveryBuilder::batch_delay)
    pub fn build(self) -> Result<CreateActionScheduleRequestBatchDelivery, BuildError> {
        Ok(CreateActionScheduleRequestBatchDelivery {
            batch_count: self.batch_count.ok_or_else(|| BuildError::missing_field("batch_count"))?,
            batch_delay: self.batch_delay.ok_or_else(|| BuildError::missing_field("batch_delay"))?,
        })
    }
}
