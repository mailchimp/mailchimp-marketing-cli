pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Choose whether the campaign should use [Batch Delivery](https://mailchimp.com/help/schedule-batch-delivery/). Cannot be set to `true` for campaigns using [Timewarp](https://mailchimp.com/help/use-timewarp/).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateActionScheduleCampaignsRequestBatchDelivery {
    /// The number of batches for the campaign send.
    #[serde(default)]
    pub batch_count: i64,
    /// The delay, in minutes, between batches.
    #[serde(default)]
    pub batch_delay: i64,
}

impl CreateActionScheduleCampaignsRequestBatchDelivery {
    pub fn builder() -> CreateActionScheduleCampaignsRequestBatchDeliveryBuilder {
        <CreateActionScheduleCampaignsRequestBatchDeliveryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateActionScheduleCampaignsRequestBatchDeliveryBuilder {
    batch_count: Option<i64>,
    batch_delay: Option<i64>,
}

impl CreateActionScheduleCampaignsRequestBatchDeliveryBuilder {
    pub fn batch_count(mut self, value: i64) -> Self {
        self.batch_count = Some(value);
        self
    }

    pub fn batch_delay(mut self, value: i64) -> Self {
        self.batch_delay = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateActionScheduleCampaignsRequestBatchDelivery`].
    /// This method will fail if any of the following fields are not set:
    /// - [`batch_count`](CreateActionScheduleCampaignsRequestBatchDeliveryBuilder::batch_count)
    /// - [`batch_delay`](CreateActionScheduleCampaignsRequestBatchDeliveryBuilder::batch_delay)
    pub fn build(self) -> Result<CreateActionScheduleCampaignsRequestBatchDelivery, BuildError> {
        Ok(CreateActionScheduleCampaignsRequestBatchDelivery {
            batch_count: self.batch_count.ok_or_else(|| BuildError::missing_field("batch_count"))?,
            batch_delay: self.batch_delay.ok_or_else(|| BuildError::missing_field("batch_delay"))?,
        })
    }
}
