pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateStoreOrderRequestPromosItem {
    pub amount_discounted: UpdateStoreOrderRequestPromosItemAmountDiscounted,
    /// The Promo Code
    #[serde(default)]
    pub code: String,
    /// Type of discount. For free shipping set type to fixed
    pub r#type: UpdateStoreOrderRequestPromosItemType,
}

impl UpdateStoreOrderRequestPromosItem {
    pub fn builder() -> UpdateStoreOrderRequestPromosItemBuilder {
        <UpdateStoreOrderRequestPromosItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateStoreOrderRequestPromosItemBuilder {
    amount_discounted: Option<UpdateStoreOrderRequestPromosItemAmountDiscounted>,
    code: Option<String>,
    r#type: Option<UpdateStoreOrderRequestPromosItemType>,
}

impl UpdateStoreOrderRequestPromosItemBuilder {
    pub fn amount_discounted(mut self, value: UpdateStoreOrderRequestPromosItemAmountDiscounted) -> Self {
        self.amount_discounted = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: UpdateStoreOrderRequestPromosItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateStoreOrderRequestPromosItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_discounted`](UpdateStoreOrderRequestPromosItemBuilder::amount_discounted)
    /// - [`code`](UpdateStoreOrderRequestPromosItemBuilder::code)
    /// - [`r#type`](UpdateStoreOrderRequestPromosItemBuilder::r#type)
    pub fn build(self) -> Result<UpdateStoreOrderRequestPromosItem, BuildError> {
        Ok(UpdateStoreOrderRequestPromosItem {
            amount_discounted: self.amount_discounted.ok_or_else(|| BuildError::missing_field("amount_discounted"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
