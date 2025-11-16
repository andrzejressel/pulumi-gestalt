#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetInstanceTypeOfferingsBrokerInstanceOption {
    /// List of available AZs. See Availability Zones. below
    #[builder(into)]
    #[serde(rename = "availabilityZones")]
    pub r#availability_zones: Vec<super::super::types::mq::GetInstanceTypeOfferingsBrokerInstanceOptionAvailabilityZone>,
    /// Filter response by engine type.
    #[builder(into)]
    #[serde(rename = "engineType")]
    pub r#engine_type: String,
    /// Filter response by host instance type.
    #[builder(into)]
    #[serde(rename = "hostInstanceType")]
    pub r#host_instance_type: String,
    /// Filter response by storage type.
    #[builder(into)]
    #[serde(rename = "storageType")]
    pub r#storage_type: String,
    /// The list of supported deployment modes.
    #[builder(into)]
    #[serde(rename = "supportedDeploymentModes")]
    pub r#supported_deployment_modes: Vec<String>,
    /// The list of supported engine versions.
    #[builder(into)]
    #[serde(rename = "supportedEngineVersions")]
    pub r#supported_engine_versions: Vec<String>,
}
