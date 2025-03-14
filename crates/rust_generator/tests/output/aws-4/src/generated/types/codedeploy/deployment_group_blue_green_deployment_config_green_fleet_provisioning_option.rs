#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DeploymentGroupBlueGreenDeploymentConfigGreenFleetProvisioningOption {
    /// The method used to add instances to a replacement environment.
    /// * `DISCOVER_EXISTING`: Use instances that already exist or will be created manually.
    /// * `COPY_AUTO_SCALING_GROUP`: Use settings from a specified **Auto Scaling** group to define and create instances in a new Auto Scaling group. _Exactly one Auto Scaling group must be specified_ when selecting `COPY_AUTO_SCALING_GROUP`. Use `autoscaling_groups` to specify the Auto Scaling group.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
}
