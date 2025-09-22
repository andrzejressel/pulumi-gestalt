#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDistributionConfigurationDistributionFastLaunchConfiguration {
    /// The account ID that this configuration applies to.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: String,
    /// A Boolean that represents the current state of faster launching for the Windows AMI.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Nested list of launch templates that the fast-launch enabled Windows AMI uses when it launches Windows instances to create pre-provisioned snapshots.
    #[builder(into)]
    #[serde(rename = "launchTemplates")]
    pub r#launch_templates: Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionFastLaunchConfigurationLaunchTemplate>,
    /// The maximum number of parallel instances that are launched for creating resources.
    #[builder(into)]
    #[serde(rename = "maxParallelLaunches")]
    pub r#max_parallel_launches: i32,
    /// Nested list of configurations for managing the number of snapshots that are created from pre-provisioned instances for the Windows AMI when faster launching is enabled.
    #[builder(into)]
    #[serde(rename = "snapshotConfigurations")]
    pub r#snapshot_configurations: Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionFastLaunchConfigurationSnapshotConfiguration>,
}
