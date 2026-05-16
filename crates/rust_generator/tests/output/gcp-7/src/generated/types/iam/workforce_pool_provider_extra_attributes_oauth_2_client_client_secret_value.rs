#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkforcePoolProviderExtraAttributesOauth2ClientClientSecretValue {
    /// The plain text of the client secret value.
    #[builder(into)]
    #[serde(rename = "plainText")]
    pub r#plain_text: String,
    /// (Output)
    /// A thumbprint to represent the current client secret value.
    #[builder(into)]
    #[serde(rename = "thumbprint")]
    pub r#thumbprint: Option<String>,
}
