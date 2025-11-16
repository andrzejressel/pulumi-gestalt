#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterMasterInstanceFleetLaunchSpecifications {
    /// Configuration block for on demand instances launch specifications.
    #[builder(into)]
    #[serde(rename = "onDemandSpecifications")]
    pub r#on_demand_specifications: Option<Vec<super::super::types::emr::ClusterMasterInstanceFleetLaunchSpecificationsOnDemandSpecification>>,
    /// Configuration block for spot instances launch specifications.
    #[builder(into)]
    #[serde(rename = "spotSpecifications")]
    pub r#spot_specifications: Option<Vec<super::super::types::emr::ClusterMasterInstanceFleetLaunchSpecificationsSpotSpecification>>,
}
