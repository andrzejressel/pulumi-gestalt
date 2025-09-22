#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainRuleBasedMatchingAttributeTypesSelector {
    /// The `Address` type. You can choose from `Address`, `BusinessAddress`, `MaillingAddress`, and `ShippingAddress`.
    #[builder(into)]
    #[serde(rename = "addresses")]
    pub r#addresses: Option<Vec<String>>,
    /// Configures the `AttributeMatchingModel`, you can either choose `ONE_TO_ONE` or `MANY_TO_MANY`.
    #[builder(into)]
    #[serde(rename = "attributeMatchingModel")]
    pub r#attribute_matching_model: String,
    /// The `Email` type. You can choose from `EmailAddress`, `BusinessEmailAddress` and `PersonalEmailAddress`.
    #[builder(into)]
    #[serde(rename = "emailAddresses")]
    pub r#email_addresses: Option<Vec<String>>,
    /// The `PhoneNumber` type. You can choose from `PhoneNumber`, `HomePhoneNumber`, and `MobilePhoneNumber`.
    #[builder(into)]
    #[serde(rename = "phoneNumbers")]
    pub r#phone_numbers: Option<Vec<String>>,
}
