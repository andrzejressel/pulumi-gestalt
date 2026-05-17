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
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "commands".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#commands,
                )
                .await,
            );
            map.insert(
                "environments".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#environments,
                )
                .await,
            );
            map.insert(
                "ephemeral_storages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ephemeral_storages,
                )
                .await,
            );
            map.insert(
                "execution_role_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#execution_role_arn,
                )
                .await,
            );
            map.insert(
                "fargate_platform_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fargate_platform_configurations,
                )
                .await,
            );
            map.insert(
                "image".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image,
                )
                .await,
            );
            map.insert(
                "instance_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_type,
                )
                .await,
            );
            map.insert(
                "job_role_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#job_role_arn,
                )
                .await,
            );
            map.insert(
                "linux_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#linux_parameters,
                )
                .await,
            );
            map.insert(
                "log_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#log_configurations,
                )
                .await,
            );
            map.insert(
                "mount_points".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mount_points,
                )
                .await,
            );
            map.insert(
                "network_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network_configurations,
                )
                .await,
            );
            map.insert(
                "privileged".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#privileged,
                )
                .await,
            );
            map.insert(
                "readonly_root_filesystem".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#readonly_root_filesystem,
                )
                .await,
            );
            map.insert(
                "resource_requirements".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_requirements,
                )
                .await,
            );
            map.insert(
                "runtime_platforms".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#runtime_platforms,
                )
                .await,
            );
            map.insert(
                "secrets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secrets,
                )
                .await,
            );
            map.insert(
                "ulimits".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ulimits,
                )
                .await,
            );
            map.insert(
                "user".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user,
                )
                .await,
            );
            map.insert(
                "volumes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#volumes,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
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
