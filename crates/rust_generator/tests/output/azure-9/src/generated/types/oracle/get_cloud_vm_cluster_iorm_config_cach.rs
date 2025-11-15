#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCloudVmClusterIormConfigCach {
    /// A `db_plans` block as defined above.
    #[builder(into)]
    #[serde(rename = "dbPlans")]
    pub r#db_plans: Vec<super::super::types::oracle::GetCloudVmClusterIormConfigCachDbPlan>,
    /// Additional information about the current `lifecycleState`.
    #[builder(into)]
    #[serde(rename = "lifecycleDetails")]
    pub r#lifecycle_details: String,
    /// The current state of IORM configuration for the Exadata DB system.
    #[builder(into)]
    #[serde(rename = "lifecycleState")]
    pub r#lifecycle_state: String,
    /// The current value for the IORM objective. The default is `AUTO`.
    #[builder(into)]
    #[serde(rename = "objective")]
    pub r#objective: String,
}
