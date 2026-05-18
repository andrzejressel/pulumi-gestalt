#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainer {
    /// The command that's passed to the container.
    #[builder(into)]
    #[serde(rename = "commands")]
    pub r#commands: Vec<String>,
    /// The environment variables to pass to a container.
    #[builder(into)]
    #[serde(rename = "environments")]
    pub r#environments: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerEnvironment>,
    /// The amount of ephemeral storage to allocate for the task. This parameter is used to expand the total amount of ephemeral storage available, beyond the default amount, for tasks hosted on AWS Fargate.
    #[builder(into)]
    #[serde(rename = "ephemeralStorages")]
    pub r#ephemeral_storages: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerEphemeralStorage>,
    /// The Amazon Resource Name (ARN) of the execution role that AWS Batch can assume. For jobs that run on Fargate resources, you must provide an execution role.
    #[builder(into)]
    #[serde(rename = "executionRoleArn")]
    pub r#execution_role_arn: String,
    /// The platform configuration for jobs that are running on Fargate resources. Jobs that are running on EC2 resources must not specify this parameter.
    #[builder(into)]
    #[serde(rename = "fargatePlatformConfigurations")]
    pub r#fargate_platform_configurations: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerFargatePlatformConfiguration>,
    /// The image used to start a container.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: String,
    /// The instance type to use for a multi-node parallel job.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: String,
    /// The Amazon Resource Name (ARN) of the IAM role that the container can assume for AWS permissions.
    #[builder(into)]
    #[serde(rename = "jobRoleArn")]
    pub r#job_role_arn: String,
    /// Linux-specific modifications that are applied to the container.
    #[builder(into)]
    #[serde(rename = "linuxParameters")]
    pub r#linux_parameters: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerLinuxParameter>,
    /// The log configuration specification for the container.
    #[builder(into)]
    #[serde(rename = "logConfigurations")]
    pub r#log_configurations: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerLogConfiguration>,
    /// The mount points for data volumes in your container.
    #[builder(into)]
    #[serde(rename = "mountPoints")]
    pub r#mount_points: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerMountPoint>,
    /// The network configuration for jobs that are running on Fargate resources.
    #[builder(into)]
    #[serde(rename = "networkConfigurations")]
    pub r#network_configurations: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerNetworkConfiguration>,
    /// When this parameter is true, the container is given elevated permissions on the host container instance (similar to the root user).
    #[builder(into)]
    #[serde(rename = "privileged")]
    pub r#privileged: bool,
    /// When this parameter is true, the container is given read-only access to its root file system.
    #[builder(into)]
    #[serde(rename = "readonlyRootFilesystem")]
    pub r#readonly_root_filesystem: bool,
    /// The type and amount of resources to assign to a container.
    #[builder(into)]
    #[serde(rename = "resourceRequirements")]
    pub r#resource_requirements: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerResourceRequirement>,
    /// An object that represents the compute environment architecture for AWS Batch jobs on Fargate.
    #[builder(into)]
    #[serde(rename = "runtimePlatforms")]
    pub r#runtime_platforms: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerRuntimePlatform>,
    /// The secrets for the container.
    #[builder(into)]
    #[serde(rename = "secrets")]
    pub r#secrets: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerSecret>,
    /// A list of ulimits to set in the container.
    #[builder(into)]
    #[serde(rename = "ulimits")]
    pub r#ulimits: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerUlimit>,
    /// The user name to use inside the container.
    #[builder(into)]
    #[serde(rename = "user")]
    pub r#user: String,
    /// A list of data volumes used in a job.
    #[builder(into)]
    #[serde(rename = "volumes")]
    pub r#volumes: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerVolume>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetJobDefinitionNodePropertyNodeRangePropertyContainer {
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
                    "commands",
                    &self.r#commands,
                ),
                to_pulumi_object_field(
                    "environments",
                    &self.r#environments,
                ),
                to_pulumi_object_field(
                    "ephemeral_storages",
                    &self.r#ephemeral_storages,
                ),
                to_pulumi_object_field(
                    "execution_role_arn",
                    &self.r#execution_role_arn,
                ),
                to_pulumi_object_field(
                    "fargate_platform_configurations",
                    &self.r#fargate_platform_configurations,
                ),
                to_pulumi_object_field(
                    "image",
                    &self.r#image,
                ),
                to_pulumi_object_field(
                    "instance_type",
                    &self.r#instance_type,
                ),
                to_pulumi_object_field(
                    "job_role_arn",
                    &self.r#job_role_arn,
                ),
                to_pulumi_object_field(
                    "linux_parameters",
                    &self.r#linux_parameters,
                ),
                to_pulumi_object_field(
                    "log_configurations",
                    &self.r#log_configurations,
                ),
                to_pulumi_object_field(
                    "mount_points",
                    &self.r#mount_points,
                ),
                to_pulumi_object_field(
                    "network_configurations",
                    &self.r#network_configurations,
                ),
                to_pulumi_object_field(
                    "privileged",
                    &self.r#privileged,
                ),
                to_pulumi_object_field(
                    "readonly_root_filesystem",
                    &self.r#readonly_root_filesystem,
                ),
                to_pulumi_object_field(
                    "resource_requirements",
                    &self.r#resource_requirements,
                ),
                to_pulumi_object_field(
                    "runtime_platforms",
                    &self.r#runtime_platforms,
                ),
                to_pulumi_object_field(
                    "secrets",
                    &self.r#secrets,
                ),
                to_pulumi_object_field(
                    "ulimits",
                    &self.r#ulimits,
                ),
                to_pulumi_object_field(
                    "user",
                    &self.r#user,
                ),
                to_pulumi_object_field(
                    "volumes",
                    &self.r#volumes,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetJobDefinitionNodePropertyNodeRangePropertyContainer {
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
                    r#commands: {
                        let field_value = match fields_map.get("commands") {
                            Some(value) => value,
                            None => bail!("Missing field 'commands' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#environments: {
                        let field_value = match fields_map.get("environments") {
                            Some(value) => value,
                            None => bail!("Missing field 'environments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ephemeral_storages: {
                        let field_value = match fields_map.get("ephemeral_storages") {
                            Some(value) => value,
                            None => bail!("Missing field 'ephemeral_storages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#execution_role_arn: {
                        let field_value = match fields_map.get("execution_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'execution_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fargate_platform_configurations: {
                        let field_value = match fields_map.get("fargate_platform_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'fargate_platform_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image: {
                        let field_value = match fields_map.get("image") {
                            Some(value) => value,
                            None => bail!("Missing field 'image' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_type: {
                        let field_value = match fields_map.get("instance_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#job_role_arn: {
                        let field_value = match fields_map.get("job_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'job_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#linux_parameters: {
                        let field_value = match fields_map.get("linux_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'linux_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_configurations: {
                        let field_value = match fields_map.get("log_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mount_points: {
                        let field_value = match fields_map.get("mount_points") {
                            Some(value) => value,
                            None => bail!("Missing field 'mount_points' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_configurations: {
                        let field_value = match fields_map.get("network_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#privileged: {
                        let field_value = match fields_map.get("privileged") {
                            Some(value) => value,
                            None => bail!("Missing field 'privileged' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#readonly_root_filesystem: {
                        let field_value = match fields_map.get("readonly_root_filesystem") {
                            Some(value) => value,
                            None => bail!("Missing field 'readonly_root_filesystem' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_requirements: {
                        let field_value = match fields_map.get("resource_requirements") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_requirements' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#runtime_platforms: {
                        let field_value = match fields_map.get("runtime_platforms") {
                            Some(value) => value,
                            None => bail!("Missing field 'runtime_platforms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secrets: {
                        let field_value = match fields_map.get("secrets") {
                            Some(value) => value,
                            None => bail!("Missing field 'secrets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ulimits: {
                        let field_value = match fields_map.get("ulimits") {
                            Some(value) => value,
                            None => bail!("Missing field 'ulimits' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user: {
                        let field_value = match fields_map.get("user") {
                            Some(value) => value,
                            None => bail!("Missing field 'user' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volumes: {
                        let field_value = match fields_map.get("volumes") {
                            Some(value) => value,
                            None => bail!("Missing field 'volumes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
