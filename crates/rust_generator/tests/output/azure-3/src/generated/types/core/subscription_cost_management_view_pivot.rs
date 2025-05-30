#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SubscriptionCostManagementViewPivot {
    /// The name of the column which should be used for this sub-view in the Cost Analysis UI.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The data type to show in this sub-view. Possible values are `Dimension` and `TagKey`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
