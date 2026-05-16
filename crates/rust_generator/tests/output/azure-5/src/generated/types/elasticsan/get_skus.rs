#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSkus {
    /// The name of this Elastic SAN.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The SKU tier.
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: String,
}
