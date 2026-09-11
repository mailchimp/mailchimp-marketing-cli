pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateActionCreateResendCampaignsRequest {
    /// Which campaign resend shortcut to use. Default is `to_non_openers`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut_type: Option<CreateActionCreateResendCampaignsRequestShortcutType>,
}

impl CreateActionCreateResendCampaignsRequest {
    pub fn builder() -> CreateActionCreateResendCampaignsRequestBuilder {
        <CreateActionCreateResendCampaignsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateActionCreateResendCampaignsRequestBuilder {
    shortcut_type: Option<CreateActionCreateResendCampaignsRequestShortcutType>,
}

impl CreateActionCreateResendCampaignsRequestBuilder {
    pub fn shortcut_type(mut self, value: CreateActionCreateResendCampaignsRequestShortcutType) -> Self {
        self.shortcut_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateActionCreateResendCampaignsRequest`].
    pub fn build(self) -> Result<CreateActionCreateResendCampaignsRequest, BuildError> {
        Ok(CreateActionCreateResendCampaignsRequest {
            shortcut_type: self.shortcut_type,
        })
    }
}

