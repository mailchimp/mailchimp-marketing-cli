pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateActionCreateResendRequest {
    /// Which campaign resend shortcut to use. Default is `to_non_openers`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut_type: Option<CreateActionCreateResendRequestShortcutType>,
}

impl CreateActionCreateResendRequest {
    pub fn builder() -> CreateActionCreateResendRequestBuilder {
        <CreateActionCreateResendRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateActionCreateResendRequestBuilder {
    shortcut_type: Option<CreateActionCreateResendRequestShortcutType>,
}

impl CreateActionCreateResendRequestBuilder {
    pub fn shortcut_type(mut self, value: CreateActionCreateResendRequestShortcutType) -> Self {
        self.shortcut_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateActionCreateResendRequest`].
    pub fn build(self) -> Result<CreateActionCreateResendRequest, BuildError> {
        Ok(CreateActionCreateResendRequest {
            shortcut_type: self.shortcut_type,
        })
    }
}

