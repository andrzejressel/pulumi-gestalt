#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SearchResourceProperty {
    /// Details about this property. The content of this field is a JSON object that varies based on the resource type.
    #[builder(into)]
    #[serde(rename = "data")]
    pub r#data: String,
    /// The date and time that the information about this resource property was last updated.
    #[builder(into)]
    #[serde(rename = "lastReportedAt")]
    pub r#last_reported_at: String,
    /// Name of this property of the resource.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
