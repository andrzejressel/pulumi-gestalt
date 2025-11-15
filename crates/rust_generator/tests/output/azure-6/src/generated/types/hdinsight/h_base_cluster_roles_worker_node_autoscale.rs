#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HBaseClusterRolesWorkerNodeAutoscale {
    /// A `recurrence` block as defined below.
    /// 
    /// > **NOTE:** Either a `capacity` or `recurrence` block must be specified - but not both.
    #[builder(into)]
    #[serde(rename = "recurrence")]
    pub r#recurrence: Option<Box<super::super::types::hdinsight::HBaseClusterRolesWorkerNodeAutoscaleRecurrence>>,
}
