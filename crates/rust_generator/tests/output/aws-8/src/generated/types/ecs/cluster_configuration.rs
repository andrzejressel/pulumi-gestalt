#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterConfiguration {
    /// Details of the execute command configuration. See `execute_command_configuration` Block for details.
    #[builder(into)]
    #[serde(rename = "executeCommandConfiguration")]
    pub r#execute_command_configuration: Option<Box<super::super::types::ecs::ClusterConfigurationExecuteCommandConfiguration>>,
    /// Details of the managed storage configuration. See `managed_storage_configuration` Block for details.
    #[builder(into)]
    #[serde(rename = "managedStorageConfiguration")]
    pub r#managed_storage_configuration: Option<Box<super::super::types::ecs::ClusterConfigurationManagedStorageConfiguration>>,
}
