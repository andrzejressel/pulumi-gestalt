#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HciDeploymentSettingScaleUnitHostNetworkIntentVirtualSwitchConfigurationOverride {
    /// Specifies the IoV enable status for Virtual Switch. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "enableIov")]
    pub r#enable_iov: Option<String>,
    /// Specifies the load balancing algorithm for Virtual Switch. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "loadBalancingAlgorithm")]
    pub r#load_balancing_algorithm: Option<String>,
}
