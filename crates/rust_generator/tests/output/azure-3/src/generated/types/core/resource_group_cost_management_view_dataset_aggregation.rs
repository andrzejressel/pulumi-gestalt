#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResourceGroupCostManagementViewDatasetAggregation {
    /// The name of the column to aggregate. Changing this forces a new Cost Management View for a Resource Group to be created.
    #[builder(into)]
    #[serde(rename = "columnName")]
    pub r#column_name: String,
    /// The name which should be used for this aggregation. Changing this forces a new Cost Management View for a Resource Group to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
