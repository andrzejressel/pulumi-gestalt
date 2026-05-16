#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableAclAccessPolicy {
    /// The ISO8061 UTC time at which this Access Policy should be valid until.
    #[builder(into)]
    #[serde(rename = "expiry")]
    pub r#expiry: String,
    /// The permissions which should associated with this Shared Identifier.
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: String,
    /// The ISO8061 UTC time at which this Access Policy should be valid from.
    #[builder(into)]
    #[serde(rename = "start")]
    pub r#start: String,
}
