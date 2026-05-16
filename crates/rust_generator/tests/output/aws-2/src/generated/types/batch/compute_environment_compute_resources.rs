#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ComputeEnvironmentComputeResources {
    /// The allocation strategy to use for the compute resource in case not enough instances of the best fitting instance type can be allocated. For valid values, refer to the [AWS documentation](https://docs.aws.amazon.com/batch/latest/APIReference/API_ComputeResource.html#Batch-Type-ComputeResource-allocationStrategy). Defaults to `BEST_FIT`. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    #[serde(rename = "allocationStrategy")]
    pub r#allocation_strategy: Option<String>,
    /// Integer of maximum percentage that a Spot Instance price can be when compared with the On-Demand price for that instance type before instances are launched. For example, if your bid percentage is 20% (`20`), then the Spot price must be below 20% of the current On-Demand price for that EC2 instance. If you leave this field empty, the default value is 100% of the On-Demand price. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    #[serde(rename = "bidPercentage")]
    pub r#bid_percentage: Option<i32>,
    /// The desired number of EC2 vCPUS in the compute environment. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    #[serde(rename = "desiredVcpus")]
    pub r#desired_vcpus: Option<i32>,
    /// Provides information used to select Amazon Machine Images (AMIs) for EC2 instances in the compute environment. If Ec2Configuration isn't specified, the default is ECS_AL2. This parameter isn't applicable to jobs that are running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    #[serde(rename = "ec2Configurations")]
    pub r#ec_2_configurations: Option<Vec<super::super::types::batch::ComputeEnvironmentComputeResourcesEc2Configuration>>,
    /// The EC2 key pair that is used for instances launched in the compute environment. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    #[serde(rename = "ec2KeyPair")]
    pub r#ec_2_key_pair: Option<String>,
    /// The Amazon Machine Image (AMI) ID used for instances launched in the compute environment. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified. (Deprecated, use `ec2_configuration` `image_id_override` instead)
    #[builder(into)]
    #[serde(rename = "imageId")]
    pub r#image_id: Option<String>,
    /// The Amazon ECS instance role applied to Amazon EC2 instances in a compute environment. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    #[serde(rename = "instanceRole")]
    pub r#instance_role: Option<String>,
    /// A list of instance types that may be launched. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    #[serde(rename = "instanceTypes")]
    pub r#instance_types: Option<Vec<String>>,
    /// The launch template to use for your compute resources. See details below. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    #[serde(rename = "launchTemplate")]
    pub r#launch_template: Option<Box<super::super::types::batch::ComputeEnvironmentComputeResourcesLaunchTemplate>>,
    /// The maximum number of EC2 vCPUs that an environment can reach.
    #[builder(into)]
    #[serde(rename = "maxVcpus")]
    pub r#max_vcpus: i32,
    /// The minimum number of EC2 vCPUs that an environment should maintain. For `EC2` or `SPOT` compute environments, if the parameter is not explicitly defined, a `0` default value will be set. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    #[serde(rename = "minVcpus")]
    pub r#min_vcpus: Option<i32>,
    /// The Amazon EC2 placement group to associate with your compute resources.
    #[builder(into)]
    #[serde(rename = "placementGroup")]
    pub r#placement_group: Option<String>,
    /// A list of EC2 security group that are associated with instances launched in the compute environment. This parameter is required for Fargate compute environments.
    #[builder(into)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Option<Vec<String>>,
    /// The Amazon Resource Name (ARN) of the Amazon EC2 Spot Fleet IAM role applied to a SPOT compute environment. This parameter is required for SPOT compute environments. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    #[serde(rename = "spotIamFleetRole")]
    pub r#spot_iam_fleet_role: Option<String>,
    /// A list of VPC subnets into which the compute resources are launched.
    #[builder(into)]
    #[serde(rename = "subnets")]
    pub r#subnets: Vec<String>,
    /// Key-value pair tags to be applied to resources that are launched in the compute environment. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    /// The type of compute environment. Valid items are `EC2`, `SPOT`, `FARGATE` or `FARGATE_SPOT`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ComputeEnvironmentComputeResources {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("allocation_strategy".to_string(), self.r#allocation_strategy.to_pulumi_value().await);
            map.insert("bid_percentage".to_string(), self.r#bid_percentage.to_pulumi_value().await);
            map.insert("desired_vcpus".to_string(), self.r#desired_vcpus.to_pulumi_value().await);
            map.insert("ec_2_configurations".to_string(), self.r#ec_2_configurations.to_pulumi_value().await);
            map.insert("ec_2_key_pair".to_string(), self.r#ec_2_key_pair.to_pulumi_value().await);
            map.insert("image_id".to_string(), self.r#image_id.to_pulumi_value().await);
            map.insert("instance_role".to_string(), self.r#instance_role.to_pulumi_value().await);
            map.insert("instance_types".to_string(), self.r#instance_types.to_pulumi_value().await);
            map.insert("launch_template".to_string(), self.r#launch_template.to_pulumi_value().await);
            map.insert("max_vcpus".to_string(), self.r#max_vcpus.to_pulumi_value().await);
            map.insert("min_vcpus".to_string(), self.r#min_vcpus.to_pulumi_value().await);
            map.insert("placement_group".to_string(), self.r#placement_group.to_pulumi_value().await);
            map.insert("security_group_ids".to_string(), self.r#security_group_ids.to_pulumi_value().await);
            map.insert("spot_iam_fleet_role".to_string(), self.r#spot_iam_fleet_role.to_pulumi_value().await);
            map.insert("subnets".to_string(), self.r#subnets.to_pulumi_value().await);
            map.insert("tags".to_string(), self.r#tags.to_pulumi_value().await);
            map.insert("type_".to_string(), self.r#type_.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ComputeEnvironmentComputeResources {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#allocation_strategy: {
                        let field_value = match fields_map.get("allocation_strategy") {
                            Some(value) => value,
                            None => bail!("Missing field 'allocation_strategy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#bid_percentage: {
                        let field_value = match fields_map.get("bid_percentage") {
                            Some(value) => value,
                            None => bail!("Missing field 'bid_percentage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#desired_vcpus: {
                        let field_value = match fields_map.get("desired_vcpus") {
                            Some(value) => value,
                            None => bail!("Missing field 'desired_vcpus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ec_2_configurations: {
                        let field_value = match fields_map.get("ec_2_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'ec_2_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::batch::ComputeEnvironmentComputeResourcesEc2Configuration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ec_2_key_pair: {
                        let field_value = match fields_map.get("ec_2_key_pair") {
                            Some(value) => value,
                            None => bail!("Missing field 'ec_2_key_pair' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#image_id: {
                        let field_value = match fields_map.get("image_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#instance_role: {
                        let field_value = match fields_map.get("instance_role") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_role' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#instance_types: {
                        let field_value = match fields_map.get("instance_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#launch_template: {
                        let field_value = match fields_map.get("launch_template") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_template' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::batch::ComputeEnvironmentComputeResourcesLaunchTemplate>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#max_vcpus: {
                        let field_value = match fields_map.get("max_vcpus") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_vcpus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#min_vcpus: {
                        let field_value = match fields_map.get("min_vcpus") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_vcpus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#placement_group: {
                        let field_value = match fields_map.get("placement_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'placement_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#security_group_ids: {
                        let field_value = match fields_map.get("security_group_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_group_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#spot_iam_fleet_role: {
                        let field_value = match fields_map.get("spot_iam_fleet_role") {
                            Some(value) => value,
                            None => bail!("Missing field 'spot_iam_fleet_role' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#subnets: {
                        let field_value = match fields_map.get("subnets") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<std::collections::HashMap<String, String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
