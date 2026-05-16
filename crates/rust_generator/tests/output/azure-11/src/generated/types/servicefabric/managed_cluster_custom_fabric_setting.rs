#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ManagedClusterCustomFabricSetting {
    /// Parameter name.
    #[builder(into)]
    #[serde(rename = "parameter")]
    pub r#parameter: String,
    /// Section name.
    #[builder(into)]
    #[serde(rename = "section")]
    pub r#section: String,
    /// Parameter value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
