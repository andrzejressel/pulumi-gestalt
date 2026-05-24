#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ComputeEnvironmentComputeResources {
    /// The allocation strategy to use for the compute resource in case not enough instances of the best fitting instance type can be allocated. For valid values, refer to the [AWS documentation](https://docs.aws.amazon.com/batch/latest/APIReference/API_ComputeResource.html#Batch-Type-ComputeResource-allocationStrategy). Defaults to `BEST_FIT`. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    pub r#allocation_strategy: Option<String>,
    /// Integer of maximum percentage that a Spot Instance price can be when compared with the On-Demand price for that instance type before instances are launched. For example, if your bid percentage is 20% (`20`), then the Spot price must be below 20% of the current On-Demand price for that EC2 instance. If you leave this field empty, the default value is 100% of the On-Demand price. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    pub r#bid_percentage: Option<i32>,
    /// The desired number of EC2 vCPUS in the compute environment. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    pub r#desired_vcpus: Option<i32>,
    /// Provides information used to select Amazon Machine Images (AMIs) for EC2 instances in the compute environment. If Ec2Configuration isn't specified, the default is ECS_AL2. This parameter isn't applicable to jobs that are running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    pub r#ec_2_configurations: Option<Vec<super::super::types::batch::ComputeEnvironmentComputeResourcesEc2Configuration>>,
    /// The EC2 key pair that is used for instances launched in the compute environment. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    pub r#ec_2_key_pair: Option<String>,
    /// The Amazon Machine Image (AMI) ID used for instances launched in the compute environment. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified. (Deprecated, use `ec2_configuration` `image_id_override` instead)
    #[builder(into)]
    pub r#image_id: Option<String>,
    /// The Amazon ECS instance role applied to Amazon EC2 instances in a compute environment. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    pub r#instance_role: Option<String>,
    /// A list of instance types that may be launched. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    pub r#instance_types: Option<Vec<String>>,
    /// The launch template to use for your compute resources. See details below. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    pub r#launch_template: Option<Box<super::super::types::batch::ComputeEnvironmentComputeResourcesLaunchTemplate>>,
    /// The maximum number of EC2 vCPUs that an environment can reach.
    #[builder(into)]
    pub r#max_vcpus: i32,
    /// The minimum number of EC2 vCPUs that an environment should maintain. For `EC2` or `SPOT` compute environments, if the parameter is not explicitly defined, a `0` default value will be set. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    pub r#min_vcpus: Option<i32>,
    /// The Amazon EC2 placement group to associate with your compute resources.
    #[builder(into)]
    pub r#placement_group: Option<String>,
    /// A list of EC2 security group that are associated with instances launched in the compute environment. This parameter is required for Fargate compute environments.
    #[builder(into)]
    pub r#security_group_ids: Option<Vec<String>>,
    /// The Amazon Resource Name (ARN) of the Amazon EC2 Spot Fleet IAM role applied to a SPOT compute environment. This parameter is required for SPOT compute environments. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    pub r#spot_iam_fleet_role: Option<String>,
    /// A list of VPC subnets into which the compute resources are launched.
    #[builder(into)]
    pub r#subnets: Vec<String>,
    /// Key-value pair tags to be applied to resources that are launched in the compute environment. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into)]
    pub r#tags: Option<std::collections::BTreeMap<String, String>>,
    /// The type of compute environment. Valid items are `EC2`, `SPOT`, `FARGATE` or `FARGATE_SPOT`.
    #[builder(into)]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ComputeEnvironmentComputeResources {
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
                    "allocationStrategy",
                    &self.r#allocation_strategy,
                ),
                to_pulumi_object_field(
                    "bidPercentage",
                    &self.r#bid_percentage,
                ),
                to_pulumi_object_field(
                    "desiredVcpus",
                    &self.r#desired_vcpus,
                ),
                to_pulumi_object_field(
                    "ec2Configurations",
                    &self.r#ec_2_configurations,
                ),
                to_pulumi_object_field(
                    "ec2KeyPair",
                    &self.r#ec_2_key_pair,
                ),
                to_pulumi_object_field(
                    "imageId",
                    &self.r#image_id,
                ),
                to_pulumi_object_field(
                    "instanceRole",
                    &self.r#instance_role,
                ),
                to_pulumi_object_field(
                    "instanceTypes",
                    &self.r#instance_types,
                ),
                to_pulumi_object_field(
                    "launchTemplate",
                    &self.r#launch_template,
                ),
                to_pulumi_object_field(
                    "maxVcpus",
                    &self.r#max_vcpus,
                ),
                to_pulumi_object_field(
                    "minVcpus",
                    &self.r#min_vcpus,
                ),
                to_pulumi_object_field(
                    "placementGroup",
                    &self.r#placement_group,
                ),
                to_pulumi_object_field(
                    "securityGroupIds",
                    &self.r#security_group_ids,
                ),
                to_pulumi_object_field(
                    "spotIamFleetRole",
                    &self.r#spot_iam_fleet_role,
                ),
                to_pulumi_object_field(
                    "subnets",
                    &self.r#subnets,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
                to_pulumi_object_field(
                    "type",
                    &self.r#type_,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ComputeEnvironmentComputeResources {
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
                    r#allocation_strategy: {
                        let field_value = match fields_map.get("allocationStrategy") {
                            Some(value) => value,
                            None => bail!("Missing field 'allocationStrategy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bid_percentage: {
                        let field_value = match fields_map.get("bidPercentage") {
                            Some(value) => value,
                            None => bail!("Missing field 'bidPercentage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#desired_vcpus: {
                        let field_value = match fields_map.get("desiredVcpus") {
                            Some(value) => value,
                            None => bail!("Missing field 'desiredVcpus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ec_2_configurations: {
                        let field_value = match fields_map.get("ec2Configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'ec2Configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ec_2_key_pair: {
                        let field_value = match fields_map.get("ec2KeyPair") {
                            Some(value) => value,
                            None => bail!("Missing field 'ec2KeyPair' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_id: {
                        let field_value = match fields_map.get("imageId") {
                            Some(value) => value,
                            None => bail!("Missing field 'imageId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_role: {
                        let field_value = match fields_map.get("instanceRole") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceRole' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_types: {
                        let field_value = match fields_map.get("instanceTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#launch_template: {
                        let field_value = match fields_map.get("launchTemplate") {
                            Some(value) => value,
                            None => bail!("Missing field 'launchTemplate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_vcpus: {
                        let field_value = match fields_map.get("maxVcpus") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxVcpus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_vcpus: {
                        let field_value = match fields_map.get("minVcpus") {
                            Some(value) => value,
                            None => bail!("Missing field 'minVcpus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#placement_group: {
                        let field_value = match fields_map.get("placementGroup") {
                            Some(value) => value,
                            None => bail!("Missing field 'placementGroup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_group_ids: {
                        let field_value = match fields_map.get("securityGroupIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'securityGroupIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spot_iam_fleet_role: {
                        let field_value = match fields_map.get("spotIamFleetRole") {
                            Some(value) => value,
                            None => bail!("Missing field 'spotIamFleetRole' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnets: {
                        let field_value = match fields_map.get("subnets") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type") {
                            Some(value) => value,
                            None => bail!("Missing field 'type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
