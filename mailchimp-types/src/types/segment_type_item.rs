pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "condition_type")]
#[non_exhaustive]
pub enum SegmentTypeItem {
        #[non_exhaustive]
        Aim {
            #[serde(skip_serializing_if = "Option::is_none")]
            field: Option<SegmentTypeItemAimField>,
            #[serde(skip_serializing_if = "Option::is_none")]
            op: Option<SegmentTypeItemAimOp>,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<String>,
        },

        #[non_exhaustive]
        Automation {
            field: SegmentTypeItemAutomationField,
            op: SegmentTypeItemAutomationOp,
            #[serde(default)]
            value: String,
        },

        #[non_exhaustive]
        CampaignPoll {
            field: SegmentTypeItemCampaignPollField,
            op: SegmentTypeItemCampaignPollOp,
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers")]
            value: f64,
        },

        #[non_exhaustive]
        Conversation {
            field: SegmentTypeItemConversationField,
            op: SegmentTypeItemConversationOp,
            #[serde(default)]
            value: String,
        },

        #[non_exhaustive]
        Date {
            #[serde(skip_serializing_if = "Option::is_none")]
            extra: Option<String>,
            field: SegmentTypeItemDateField,
            op: SegmentTypeItemDateOp,
            #[serde(default)]
            value: String,
        },

        #[non_exhaustive]
        EmailClient {
            field: SegmentTypeItemEmailClientField,
            op: SegmentTypeItemEmailClientOp,
            #[serde(default)]
            value: String,
        },

        #[non_exhaustive]
        Language {
            field: SegmentTypeItemLanguageField,
            op: SegmentTypeItemLanguageOp,
            #[serde(default)]
            value: String,
        },

        #[non_exhaustive]
        MemberRating {
            field: SegmentTypeItemMemberRatingField,
            op: SegmentTypeItemMemberRatingOp,
            value: SegmentTypeItemMemberRatingValue,
        },

        #[non_exhaustive]
        SignupSource {
            field: SegmentTypeItemSignupSourceField,
            op: SegmentTypeItemSignupSourceOp,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<String>,
        },

        #[non_exhaustive]
        SurveyMonkey {
            field: SegmentTypeItemSurveyMonkeyField,
            op: SegmentTypeItemSurveyMonkeyOp,
            #[serde(default)]
            value: String,
        },

        #[serde(rename = "VIP")]
        #[non_exhaustive]
        Vip {
            field: SegmentTypeItemVipField,
            op: SegmentTypeItemVipOp,
        },

        #[non_exhaustive]
        Interests {
            #[serde(skip_serializing_if = "Option::is_none")]
            field: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            op: Option<SegmentTypeItemInterestsOp>,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<Vec<String>>,
        },

        #[non_exhaustive]
        EcommCategory {
            #[serde(skip_serializing_if = "Option::is_none")]
            field: Option<SegmentTypeItemEcommCategoryField>,
            #[serde(skip_serializing_if = "Option::is_none")]
            op: Option<SegmentTypeItemEcommCategoryOp>,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<String>,
        },

        #[non_exhaustive]
        EcommNumber {
            field: SegmentTypeItemEcommNumberField,
            op: SegmentTypeItemEcommNumberOp,
            value: SegmentTypeItemEcommNumberValue,
        },

        #[non_exhaustive]
        EcommPurchased {
            #[serde(skip_serializing_if = "Option::is_none")]
            field: Option<SegmentTypeItemEcommPurchasedField>,
            #[serde(skip_serializing_if = "Option::is_none")]
            op: Option<SegmentTypeItemEcommPurchasedOp>,
        },

        #[non_exhaustive]
        EcommSpent {
            #[serde(skip_serializing_if = "Option::is_none")]
            field: Option<SegmentTypeItemEcommSpentField>,
            #[serde(skip_serializing_if = "Option::is_none")]
            op: Option<SegmentTypeItemEcommSpentOp>,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<SegmentTypeItemEcommSpentValue>,
        },

        #[non_exhaustive]
        EcommStore {
            #[serde(skip_serializing_if = "Option::is_none")]
            field: Option<SegmentTypeItemEcommStoreField>,
            #[serde(skip_serializing_if = "Option::is_none")]
            op: Option<SegmentTypeItemEcommStoreOp>,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<String>,
        },

        #[non_exhaustive]
        GoalActivity {
            field: SegmentTypeItemGoalActivityField,
            op: SegmentTypeItemGoalActivityOp,
            #[serde(default)]
            value: String,
        },

        #[non_exhaustive]
        GoalTimestamp {
            field: SegmentTypeItemGoalTimestampField,
            op: SegmentTypeItemGoalTimestampOp,
            #[serde(default)]
            value: String,
        },

        #[non_exhaustive]
        FuzzySegment {
            field: SegmentTypeItemFuzzySegmentField,
            op: SegmentTypeItemFuzzySegmentOp,
            value: SegmentTypeItemFuzzySegmentValue,
        },

        #[non_exhaustive]
        StaticSegment {
            field: SegmentTypeItemStaticSegmentField,
            op: SegmentTypeItemStaticSegmentOp,
            value: SegmentTypeItemStaticSegmentValue,
        },

        #[serde(rename = "IPGeoCountryState")]
        #[non_exhaustive]
        IpGeoCountryState {
            field: SegmentTypeItemIpGeoCountryStateField,
            op: SegmentTypeItemIpGeoCountryStateOp,
            #[serde(default)]
            value: String,
        },

        #[serde(rename = "IPGeoIn")]
        #[non_exhaustive]
        IpGeoIn {
            #[serde(default)]
            addr: String,
            field: SegmentTypeItemIpGeoInField,
            #[serde(default)]
            lat: String,
            #[serde(default)]
            lng: String,
            op: SegmentTypeItemIpGeoInOp,
            #[serde(default)]
            value: i64,
        },

        #[serde(rename = "IPGeoInZip")]
        #[non_exhaustive]
        IpGeoInZip {
            #[serde(default)]
            extra: i64,
            field: SegmentTypeItemIpGeoInZipField,
            op: SegmentTypeItemIpGeoInZipOp,
            #[serde(default)]
            value: i64,
        },

        #[serde(rename = "IPGeoUnknown")]
        #[non_exhaustive]
        IpGeoUnknown {
            field: SegmentTypeItemIpGeoUnknownField,
            op: SegmentTypeItemIpGeoUnknownOp,
        },

        #[serde(rename = "IPGeoZip")]
        #[non_exhaustive]
        IpGeoZip {
            field: SegmentTypeItemIpGeoZipField,
            op: SegmentTypeItemIpGeoZipOp,
            #[serde(default)]
            value: i64,
        },

        #[non_exhaustive]
        SocialAge {
            field: SegmentTypeItemSocialAgeField,
            op: SegmentTypeItemSocialAgeOp,
            value: SegmentTypeItemSocialAgeValue,
        },

        #[non_exhaustive]
        SocialGender {
            field: SegmentTypeItemSocialGenderField,
            op: SegmentTypeItemSocialGenderOp,
            value: SegmentTypeItemSocialGenderValue,
        },

        #[non_exhaustive]
        SocialInfluence {
            field: SegmentTypeItemSocialInfluenceField,
            op: SegmentTypeItemSocialInfluenceOp,
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers")]
            value: f64,
        },

        #[non_exhaustive]
        SocialNetworkMember {
            field: SegmentTypeItemSocialNetworkMemberField,
            op: SegmentTypeItemSocialNetworkMemberOp,
            value: SegmentTypeItemSocialNetworkMemberValue,
        },

        #[non_exhaustive]
        SocialNetworkFollow {
            field: SegmentTypeItemSocialNetworkFollowField,
            op: SegmentTypeItemSocialNetworkFollowOp,
            value: SegmentTypeItemSocialNetworkFollowValue,
        },

        #[non_exhaustive]
        AddressMerge {
            #[serde(default)]
            field: String,
            op: SegmentTypeItemAddressMergeOp,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<String>,
        },

        #[non_exhaustive]
        ZipMerge {
            #[serde(default)]
            extra: String,
            #[serde(default)]
            field: String,
            op: SegmentTypeItemZipMergeOp,
            #[serde(default)]
            value: String,
        },

        #[non_exhaustive]
        BirthdayMerge {
            #[serde(default)]
            field: String,
            op: SegmentTypeItemBirthdayMergeOp,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<String>,
        },

        #[non_exhaustive]
        DateMerge {
            #[serde(default)]
            field: String,
            op: SegmentTypeItemDateMergeOp,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<String>,
        },

        #[non_exhaustive]
        SelectMerge {
            #[serde(default)]
            field: String,
            op: SegmentTypeItemSelectMergeOp,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<String>,
        },

        #[non_exhaustive]
        TextMerge {
            #[serde(default)]
            field: String,
            op: SegmentTypeItemTextMergeOp,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<String>,
        },

        #[non_exhaustive]
        EmailAddress {
            field: SegmentTypeItemEmailAddressField,
            op: SegmentTypeItemEmailAddressOp,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<String>,
        },

        #[non_exhaustive]
        PredictedGender {
            field: SegmentTypeItemPredictedGenderField,
            op: SegmentTypeItemPredictedGenderOp,
            value: SegmentTypeItemPredictedGenderValue,
        },

        #[non_exhaustive]
        PredictedAge {
            field: SegmentTypeItemPredictedAgeField,
            op: SegmentTypeItemPredictedAgeOp,
            value: SegmentTypeItemPredictedAgeValue,
        },

        #[non_exhaustive]
        NewSubscribers {
            #[serde(skip_serializing_if = "Option::is_none")]
            field: Option<SegmentTypeItemNewSubscribersField>,
            #[serde(skip_serializing_if = "Option::is_none")]
            op: Option<SegmentTypeItemNewSubscribersOp>,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<String>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl SegmentTypeItem {
    pub fn aim() -> Self {
        Self::Aim { field: None, op: None, value: None }
    }

    pub fn automation(field: SegmentTypeItemAutomationField, op: SegmentTypeItemAutomationOp, value: String) -> Self {
        Self::Automation { field, op, value }
    }

    pub fn campaign_poll(field: SegmentTypeItemCampaignPollField, op: SegmentTypeItemCampaignPollOp, value: f64) -> Self {
        Self::CampaignPoll { field, op, value }
    }

    pub fn conversation(field: SegmentTypeItemConversationField, op: SegmentTypeItemConversationOp, value: String) -> Self {
        Self::Conversation { field, op, value }
    }

    pub fn date(field: SegmentTypeItemDateField, op: SegmentTypeItemDateOp, value: String) -> Self {
        Self::Date { extra: None, field, op, value }
    }

    pub fn email_client(field: SegmentTypeItemEmailClientField, op: SegmentTypeItemEmailClientOp, value: String) -> Self {
        Self::EmailClient { field, op, value }
    }

    pub fn language(field: SegmentTypeItemLanguageField, op: SegmentTypeItemLanguageOp, value: String) -> Self {
        Self::Language { field, op, value }
    }

    pub fn member_rating(field: SegmentTypeItemMemberRatingField, op: SegmentTypeItemMemberRatingOp, value: SegmentTypeItemMemberRatingValue) -> Self {
        Self::MemberRating { field, op, value }
    }

    pub fn signup_source(field: SegmentTypeItemSignupSourceField, op: SegmentTypeItemSignupSourceOp) -> Self {
        Self::SignupSource { field, op, value: None }
    }

    pub fn survey_monkey(field: SegmentTypeItemSurveyMonkeyField, op: SegmentTypeItemSurveyMonkeyOp, value: String) -> Self {
        Self::SurveyMonkey { field, op, value }
    }

    pub fn vip(field: SegmentTypeItemVipField, op: SegmentTypeItemVipOp) -> Self {
        Self::Vip { field, op }
    }

    pub fn interests() -> Self {
        Self::Interests { field: None, op: None, value: None }
    }

    pub fn ecomm_category() -> Self {
        Self::EcommCategory { field: None, op: None, value: None }
    }

    pub fn ecomm_number(field: SegmentTypeItemEcommNumberField, op: SegmentTypeItemEcommNumberOp, value: SegmentTypeItemEcommNumberValue) -> Self {
        Self::EcommNumber { field, op, value }
    }

    pub fn ecomm_purchased() -> Self {
        Self::EcommPurchased { field: None, op: None }
    }

    pub fn ecomm_spent() -> Self {
        Self::EcommSpent { field: None, op: None, value: None }
    }

    pub fn ecomm_store() -> Self {
        Self::EcommStore { field: None, op: None, value: None }
    }

    pub fn goal_activity(field: SegmentTypeItemGoalActivityField, op: SegmentTypeItemGoalActivityOp, value: String) -> Self {
        Self::GoalActivity { field, op, value }
    }

    pub fn goal_timestamp(field: SegmentTypeItemGoalTimestampField, op: SegmentTypeItemGoalTimestampOp, value: String) -> Self {
        Self::GoalTimestamp { field, op, value }
    }

    pub fn fuzzy_segment(field: SegmentTypeItemFuzzySegmentField, op: SegmentTypeItemFuzzySegmentOp, value: SegmentTypeItemFuzzySegmentValue) -> Self {
        Self::FuzzySegment { field, op, value }
    }

    pub fn static_segment(field: SegmentTypeItemStaticSegmentField, op: SegmentTypeItemStaticSegmentOp, value: SegmentTypeItemStaticSegmentValue) -> Self {
        Self::StaticSegment { field, op, value }
    }

    pub fn ip_geo_country_state(field: SegmentTypeItemIpGeoCountryStateField, op: SegmentTypeItemIpGeoCountryStateOp, value: String) -> Self {
        Self::IpGeoCountryState { field, op, value }
    }

    pub fn ip_geo_in(addr: String, field: SegmentTypeItemIpGeoInField, lat: String, lng: String, op: SegmentTypeItemIpGeoInOp, value: i64) -> Self {
        Self::IpGeoIn { addr, field, lat, lng, op, value }
    }

    pub fn ip_geo_in_zip(extra: i64, field: SegmentTypeItemIpGeoInZipField, op: SegmentTypeItemIpGeoInZipOp, value: i64) -> Self {
        Self::IpGeoInZip { extra, field, op, value }
    }

    pub fn ip_geo_unknown(field: SegmentTypeItemIpGeoUnknownField, op: SegmentTypeItemIpGeoUnknownOp) -> Self {
        Self::IpGeoUnknown { field, op }
    }

    pub fn ip_geo_zip(field: SegmentTypeItemIpGeoZipField, op: SegmentTypeItemIpGeoZipOp, value: i64) -> Self {
        Self::IpGeoZip { field, op, value }
    }

    pub fn social_age(field: SegmentTypeItemSocialAgeField, op: SegmentTypeItemSocialAgeOp, value: SegmentTypeItemSocialAgeValue) -> Self {
        Self::SocialAge { field, op, value }
    }

    pub fn social_gender(field: SegmentTypeItemSocialGenderField, op: SegmentTypeItemSocialGenderOp, value: SegmentTypeItemSocialGenderValue) -> Self {
        Self::SocialGender { field, op, value }
    }

    pub fn social_influence(field: SegmentTypeItemSocialInfluenceField, op: SegmentTypeItemSocialInfluenceOp, value: f64) -> Self {
        Self::SocialInfluence { field, op, value }
    }

    pub fn social_network_member(field: SegmentTypeItemSocialNetworkMemberField, op: SegmentTypeItemSocialNetworkMemberOp, value: SegmentTypeItemSocialNetworkMemberValue) -> Self {
        Self::SocialNetworkMember { field, op, value }
    }

    pub fn social_network_follow(field: SegmentTypeItemSocialNetworkFollowField, op: SegmentTypeItemSocialNetworkFollowOp, value: SegmentTypeItemSocialNetworkFollowValue) -> Self {
        Self::SocialNetworkFollow { field, op, value }
    }

    pub fn address_merge(field: String, op: SegmentTypeItemAddressMergeOp) -> Self {
        Self::AddressMerge { field, op, value: None }
    }

    pub fn zip_merge(extra: String, field: String, op: SegmentTypeItemZipMergeOp, value: String) -> Self {
        Self::ZipMerge { extra, field, op, value }
    }

    pub fn birthday_merge(field: String, op: SegmentTypeItemBirthdayMergeOp) -> Self {
        Self::BirthdayMerge { field, op, value: None }
    }

    pub fn date_merge(field: String, op: SegmentTypeItemDateMergeOp) -> Self {
        Self::DateMerge { field, op, value: None }
    }

    pub fn select_merge(field: String, op: SegmentTypeItemSelectMergeOp) -> Self {
        Self::SelectMerge { field, op, value: None }
    }

    pub fn text_merge(field: String, op: SegmentTypeItemTextMergeOp) -> Self {
        Self::TextMerge { field, op, value: None }
    }

    pub fn email_address(field: SegmentTypeItemEmailAddressField, op: SegmentTypeItemEmailAddressOp) -> Self {
        Self::EmailAddress { field, op, value: None }
    }

    pub fn predicted_gender(field: SegmentTypeItemPredictedGenderField, op: SegmentTypeItemPredictedGenderOp, value: SegmentTypeItemPredictedGenderValue) -> Self {
        Self::PredictedGender { field, op, value }
    }

    pub fn predicted_age(field: SegmentTypeItemPredictedAgeField, op: SegmentTypeItemPredictedAgeOp, value: SegmentTypeItemPredictedAgeValue) -> Self {
        Self::PredictedAge { field, op, value }
    }

    pub fn new_subscribers() -> Self {
        Self::NewSubscribers { field: None, op: None, value: None }
    }

    pub fn aim_with_field(field: SegmentTypeItemAimField, op: Option<SegmentTypeItemAimOp>, value: Option<String>) -> Self {
        Self::Aim { field: Some(field), op, value }
    }

    pub fn aim_with_op(field: Option<SegmentTypeItemAimField>, op: SegmentTypeItemAimOp, value: Option<String>) -> Self {
        Self::Aim { field, op: Some(op), value }
    }

    pub fn aim_with_value(field: Option<SegmentTypeItemAimField>, op: Option<SegmentTypeItemAimOp>, value: String) -> Self {
        Self::Aim { field, op, value: Some(value) }
    }

    pub fn date_with_extra(extra: String, field: SegmentTypeItemDateField, op: SegmentTypeItemDateOp, value: String) -> Self {
        Self::Date { extra: Some(extra), field, op, value }
    }

    pub fn signup_source_with_value(field: SegmentTypeItemSignupSourceField, op: SegmentTypeItemSignupSourceOp, value: String) -> Self {
        Self::SignupSource { field, op, value: Some(value) }
    }

    pub fn interests_with_field(field: String, op: Option<SegmentTypeItemInterestsOp>, value: Option<Vec<String>>) -> Self {
        Self::Interests { field: Some(field), op, value }
    }

    pub fn interests_with_op(field: Option<String>, op: SegmentTypeItemInterestsOp, value: Option<Vec<String>>) -> Self {
        Self::Interests { field, op: Some(op), value }
    }

    pub fn interests_with_value(field: Option<String>, op: Option<SegmentTypeItemInterestsOp>, value: Vec<String>) -> Self {
        Self::Interests { field, op, value: Some(value) }
    }

    pub fn ecomm_category_with_field(field: SegmentTypeItemEcommCategoryField, op: Option<SegmentTypeItemEcommCategoryOp>, value: Option<String>) -> Self {
        Self::EcommCategory { field: Some(field), op, value }
    }

    pub fn ecomm_category_with_op(field: Option<SegmentTypeItemEcommCategoryField>, op: SegmentTypeItemEcommCategoryOp, value: Option<String>) -> Self {
        Self::EcommCategory { field, op: Some(op), value }
    }

    pub fn ecomm_category_with_value(field: Option<SegmentTypeItemEcommCategoryField>, op: Option<SegmentTypeItemEcommCategoryOp>, value: String) -> Self {
        Self::EcommCategory { field, op, value: Some(value) }
    }

    pub fn ecomm_purchased_with_field(field: SegmentTypeItemEcommPurchasedField, op: Option<SegmentTypeItemEcommPurchasedOp>) -> Self {
        Self::EcommPurchased { field: Some(field), op }
    }

    pub fn ecomm_purchased_with_op(field: Option<SegmentTypeItemEcommPurchasedField>, op: SegmentTypeItemEcommPurchasedOp) -> Self {
        Self::EcommPurchased { field, op: Some(op) }
    }

    pub fn ecomm_spent_with_field(field: SegmentTypeItemEcommSpentField, op: Option<SegmentTypeItemEcommSpentOp>, value: Option<SegmentTypeItemEcommSpentValue>) -> Self {
        Self::EcommSpent { field: Some(field), op, value }
    }

    pub fn ecomm_spent_with_op(field: Option<SegmentTypeItemEcommSpentField>, op: SegmentTypeItemEcommSpentOp, value: Option<SegmentTypeItemEcommSpentValue>) -> Self {
        Self::EcommSpent { field, op: Some(op), value }
    }

    pub fn ecomm_spent_with_value(field: Option<SegmentTypeItemEcommSpentField>, op: Option<SegmentTypeItemEcommSpentOp>, value: SegmentTypeItemEcommSpentValue) -> Self {
        Self::EcommSpent { field, op, value: Some(value) }
    }

    pub fn ecomm_store_with_field(field: SegmentTypeItemEcommStoreField, op: Option<SegmentTypeItemEcommStoreOp>, value: Option<String>) -> Self {
        Self::EcommStore { field: Some(field), op, value }
    }

    pub fn ecomm_store_with_op(field: Option<SegmentTypeItemEcommStoreField>, op: SegmentTypeItemEcommStoreOp, value: Option<String>) -> Self {
        Self::EcommStore { field, op: Some(op), value }
    }

    pub fn ecomm_store_with_value(field: Option<SegmentTypeItemEcommStoreField>, op: Option<SegmentTypeItemEcommStoreOp>, value: String) -> Self {
        Self::EcommStore { field, op, value: Some(value) }
    }

    pub fn address_merge_with_value(field: String, op: SegmentTypeItemAddressMergeOp, value: String) -> Self {
        Self::AddressMerge { field, op, value: Some(value) }
    }

    pub fn birthday_merge_with_value(field: String, op: SegmentTypeItemBirthdayMergeOp, value: String) -> Self {
        Self::BirthdayMerge { field, op, value: Some(value) }
    }

    pub fn date_merge_with_value(field: String, op: SegmentTypeItemDateMergeOp, value: String) -> Self {
        Self::DateMerge { field, op, value: Some(value) }
    }

    pub fn select_merge_with_value(field: String, op: SegmentTypeItemSelectMergeOp, value: String) -> Self {
        Self::SelectMerge { field, op, value: Some(value) }
    }

    pub fn text_merge_with_value(field: String, op: SegmentTypeItemTextMergeOp, value: String) -> Self {
        Self::TextMerge { field, op, value: Some(value) }
    }

    pub fn email_address_with_value(field: SegmentTypeItemEmailAddressField, op: SegmentTypeItemEmailAddressOp, value: String) -> Self {
        Self::EmailAddress { field, op, value: Some(value) }
    }

    pub fn new_subscribers_with_field(field: SegmentTypeItemNewSubscribersField, op: Option<SegmentTypeItemNewSubscribersOp>, value: Option<String>) -> Self {
        Self::NewSubscribers { field: Some(field), op, value }
    }

    pub fn new_subscribers_with_op(field: Option<SegmentTypeItemNewSubscribersField>, op: SegmentTypeItemNewSubscribersOp, value: Option<String>) -> Self {
        Self::NewSubscribers { field, op: Some(op), value }
    }

    pub fn new_subscribers_with_value(field: Option<SegmentTypeItemNewSubscribersField>, op: Option<SegmentTypeItemNewSubscribersOp>, value: String) -> Self {
        Self::NewSubscribers { field, op, value: Some(value) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
