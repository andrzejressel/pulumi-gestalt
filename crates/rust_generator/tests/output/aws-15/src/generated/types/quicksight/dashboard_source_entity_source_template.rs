#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DashboardSourceEntitySourceTemplate {
    /// The Amazon Resource Name (ARN) of the resource.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    /// List of dataset references. See data_set_references.
    #[builder(into)]
    #[serde(rename = "dataSetReferences")]
    pub r#data_set_references: Vec<super::super::types::quicksight::DashboardSourceEntitySourceTemplateDataSetReference>,
}
