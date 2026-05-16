#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SubscriberSubscriberIdentity {
    /// The AWS Regions where Security Lake is automatically enabled.
    #[builder(into)]
    #[serde(rename = "externalId")]
    pub r#external_id: String,
    /// Provides encryption details of Amazon Security Lake object.
    #[builder(into)]
    #[serde(rename = "principal")]
    pub r#principal: String,
}
