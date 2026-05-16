#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceAccessControlAttributesAttribute {
    /// The name of the attribute associated with your identities in your identity source. This is used to map a specified attribute in your identity source with an attribute in AWS SSO.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
    /// The value used for mapping a specified attribute to an identity source. See AccessControlAttributeValue
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Vec<super::super::types::ssoadmin::InstanceAccessControlAttributesAttributeValue>,
}
