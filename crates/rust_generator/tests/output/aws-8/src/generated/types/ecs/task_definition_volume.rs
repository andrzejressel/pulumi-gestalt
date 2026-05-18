#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TaskDefinitionVolume {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "configure_at_launch",
                    &self.r#configure_at_launch,
                ),
                to_pulumi_object_field(
                    "docker_volume_configuration",
                    &self.r#docker_volume_configuration,
                ),
                to_pulumi_object_field(
                    "efs_volume_configuration",
                    &self.r#efs_volume_configuration,
                ),
                to_pulumi_object_field(
                    "fsx_windows_file_server_volume_configuration",
                    &self.r#fsx_windows_file_server_volume_configuration,
                ),
                to_pulumi_object_field(
                    "host_path",
                    &self.r#host_path,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TaskDefinitionVolume {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#configure_at_launch: {
                        let field_value = match fields_map.get("configure_at_launch") {
                            Some(value) => value,
                            None => bail!("Missing field 'configure_at_launch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#docker_volume_configuration: {
                        let field_value = match fields_map.get("docker_volume_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'docker_volume_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#efs_volume_configuration: {
                        let field_value = match fields_map.get("efs_volume_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'efs_volume_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fsx_windows_file_server_volume_configuration: {
                        let field_value = match fields_map.get("fsx_windows_file_server_volume_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'fsx_windows_file_server_volume_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_path: {
                        let field_value = match fields_map.get("host_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
