#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSetColumnLevelPermissionRule {
    /// An array of column names.
    #[builder(into)]
    #[serde(rename = "columnNames")]
    pub r#column_names: Option<Vec<String>>,
    /// An array of ARNs for Amazon QuickSight users or groups.
    #[builder(into)]
    #[serde(rename = "principals")]
    pub r#principals: Option<Vec<String>>,
}
