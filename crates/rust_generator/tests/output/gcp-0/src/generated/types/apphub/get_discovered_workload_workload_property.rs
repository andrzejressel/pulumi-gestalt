#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDiscoveredWorkloadWorkloadProperty {
    /// The service project identifier that the underlying cloud resource resides in.
    #[builder(into)]
    #[serde(rename = "gcpProject")]
    pub r#gcp_project: String,
    /// The location of the discovered workload.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
    /// The location that the underlying resource resides in if it is zonal.
    #[builder(into)]
    #[serde(rename = "zone")]
    pub r#zone: String,
}
