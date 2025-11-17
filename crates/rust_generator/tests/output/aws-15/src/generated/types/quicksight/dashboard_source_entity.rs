#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DashboardSourceEntity {
    /// The source template. See source_template.
    #[builder(into)]
    #[serde(rename = "sourceTemplate")]
    pub r#source_template: Option<Box<super::super::types::quicksight::DashboardSourceEntitySourceTemplate>>,
}
