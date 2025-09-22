#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TaskDefinitionVolume {
    /// Whether the volume should be configured at launch time. This is used to create Amazon EBS volumes for standalone tasks or tasks created as part of a service. Each task definition revision may only have one volume configured at launch in the volume configuration.
    #[builder(into)]
    #[serde(rename = "configureAtLaunch")]
    pub r#configure_at_launch: Option<bool>,
    /// Configuration block to configure a docker volume. Detailed below.
    #[builder(into)]
    #[serde(rename = "dockerVolumeConfiguration")]
    pub r#docker_volume_configuration: Option<Box<super::super::types::ecs::TaskDefinitionVolumeDockerVolumeConfiguration>>,
    /// Configuration block for an EFS volume. Detailed below.
    #[builder(into)]
    #[serde(rename = "efsVolumeConfiguration")]
    pub r#efs_volume_configuration: Option<Box<super::super::types::ecs::TaskDefinitionVolumeEfsVolumeConfiguration>>,
    /// Configuration block for an FSX Windows File Server volume. Detailed below.
    #[builder(into)]
    #[serde(rename = "fsxWindowsFileServerVolumeConfiguration")]
    pub r#fsx_windows_file_server_volume_configuration: Option<Box<super::super::types::ecs::TaskDefinitionVolumeFsxWindowsFileServerVolumeConfiguration>>,
    /// Path on the host container instance that is presented to the container. If not set, ECS will create a nonpersistent data volume that starts empty and is deleted after the task has finished.
    #[builder(into)]
    #[serde(rename = "hostPath")]
    pub r#host_path: Option<String>,
    /// Name of the volume. This name is referenced in the `sourceVolume`
    /// parameter of container definition in the `mountPoints` section.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
