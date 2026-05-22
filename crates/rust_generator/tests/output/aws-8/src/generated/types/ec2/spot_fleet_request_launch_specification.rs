#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpotFleetRequestLaunchSpecification {
    #[builder(into)]
    #[serde(rename = "ami")]
    pub r#ami: String,
    #[builder(into)]
    #[serde(rename = "associatePublicIpAddress")]
    pub r#associate_public_ip_address: Option<bool>,
    /// The availability zone in which to place the request.
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Option<String>,
    #[builder(into)]
    #[serde(rename = "ebsBlockDevices")]
    pub r#ebs_block_devices: Option<Vec<super::super::types::ec2::SpotFleetRequestLaunchSpecificationEbsBlockDevice>>,
    #[builder(into)]
    #[serde(rename = "ebsOptimized")]
    pub r#ebs_optimized: Option<bool>,
    #[builder(into)]
    #[serde(rename = "ephemeralBlockDevices")]
    pub r#ephemeral_block_devices: Option<Vec<super::super::types::ec2::SpotFleetRequestLaunchSpecificationEphemeralBlockDevice>>,
    #[builder(into)]
    #[serde(rename = "iamInstanceProfile")]
    pub r#iam_instance_profile: Option<String>,
    #[builder(into)]
    #[serde(rename = "iamInstanceProfileArn")]
    pub r#iam_instance_profile_arn: Option<String>,
    /// The type of instance to request.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: String,
    #[builder(into)]
    #[serde(rename = "keyName")]
    pub r#key_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "monitoring")]
    pub r#monitoring: Option<bool>,
    #[builder(into)]
    #[serde(rename = "placementGroup")]
    pub r#placement_group: Option<String>,
    #[builder(into)]
    #[serde(rename = "placementTenancy")]
    pub r#placement_tenancy: Option<String>,
    #[builder(into)]
    #[serde(rename = "rootBlockDevices")]
    pub r#root_block_devices: Option<Vec<super::super::types::ec2::SpotFleetRequestLaunchSpecificationRootBlockDevice>>,
    /// The maximum bid price per unit hour.
    #[builder(into)]
    #[serde(rename = "spotPrice")]
    pub r#spot_price: Option<String>,
    /// The subnet in which to launch the requested instance.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Option<String>,
    /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    #[builder(into)]
    #[serde(rename = "userData")]
    pub r#user_data: Option<String>,
    #[builder(into)]
    #[serde(rename = "vpcSecurityGroupIds")]
    pub r#vpc_security_group_ids: Option<Vec<String>>,
    /// The capacity added to the fleet by a fulfilled request.
    #[builder(into)]
    #[serde(rename = "weightedCapacity")]
    pub r#weighted_capacity: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpotFleetRequestLaunchSpecification {
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
                    "ami",
                    &self.r#ami,
                ),
                to_pulumi_object_field(
                    "associatePublicIpAddress",
                    &self.r#associate_public_ip_address,
                ),
                to_pulumi_object_field(
                    "availabilityZone",
                    &self.r#availability_zone,
                ),
                to_pulumi_object_field(
                    "ebsBlockDevices",
                    &self.r#ebs_block_devices,
                ),
                to_pulumi_object_field(
                    "ebsOptimized",
                    &self.r#ebs_optimized,
                ),
                to_pulumi_object_field(
                    "ephemeralBlockDevices",
                    &self.r#ephemeral_block_devices,
                ),
                to_pulumi_object_field(
                    "iamInstanceProfile",
                    &self.r#iam_instance_profile,
                ),
                to_pulumi_object_field(
                    "iamInstanceProfileArn",
                    &self.r#iam_instance_profile_arn,
                ),
                to_pulumi_object_field(
                    "instanceType",
                    &self.r#instance_type,
                ),
                to_pulumi_object_field(
                    "keyName",
                    &self.r#key_name,
                ),
                to_pulumi_object_field(
                    "monitoring",
                    &self.r#monitoring,
                ),
                to_pulumi_object_field(
                    "placementGroup",
                    &self.r#placement_group,
                ),
                to_pulumi_object_field(
                    "placementTenancy",
                    &self.r#placement_tenancy,
                ),
                to_pulumi_object_field(
                    "rootBlockDevices",
                    &self.r#root_block_devices,
                ),
                to_pulumi_object_field(
                    "spotPrice",
                    &self.r#spot_price,
                ),
                to_pulumi_object_field(
                    "subnetId",
                    &self.r#subnet_id,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
                to_pulumi_object_field(
                    "userData",
                    &self.r#user_data,
                ),
                to_pulumi_object_field(
                    "vpcSecurityGroupIds",
                    &self.r#vpc_security_group_ids,
                ),
                to_pulumi_object_field(
                    "weightedCapacity",
                    &self.r#weighted_capacity,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SpotFleetRequestLaunchSpecification {
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
                    r#ami: {
                        let field_value = match fields_map.get("ami") {
                            Some(value) => value,
                            None => bail!("Missing field 'ami' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#associate_public_ip_address: {
                        let field_value = match fields_map.get("associatePublicIpAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'associatePublicIpAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#availability_zone: {
                        let field_value = match fields_map.get("availabilityZone") {
                            Some(value) => value,
                            None => bail!("Missing field 'availabilityZone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ebs_block_devices: {
                        let field_value = match fields_map.get("ebsBlockDevices") {
                            Some(value) => value,
                            None => bail!("Missing field 'ebsBlockDevices' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ebs_optimized: {
                        let field_value = match fields_map.get("ebsOptimized") {
                            Some(value) => value,
                            None => bail!("Missing field 'ebsOptimized' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ephemeral_block_devices: {
                        let field_value = match fields_map.get("ephemeralBlockDevices") {
                            Some(value) => value,
                            None => bail!("Missing field 'ephemeralBlockDevices' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#iam_instance_profile: {
                        let field_value = match fields_map.get("iamInstanceProfile") {
                            Some(value) => value,
                            None => bail!("Missing field 'iamInstanceProfile' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#iam_instance_profile_arn: {
                        let field_value = match fields_map.get("iamInstanceProfileArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'iamInstanceProfileArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_type: {
                        let field_value = match fields_map.get("instanceType") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_name: {
                        let field_value = match fields_map.get("keyName") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monitoring: {
                        let field_value = match fields_map.get("monitoring") {
                            Some(value) => value,
                            None => bail!("Missing field 'monitoring' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#placement_tenancy: {
                        let field_value = match fields_map.get("placementTenancy") {
                            Some(value) => value,
                            None => bail!("Missing field 'placementTenancy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_block_devices: {
                        let field_value = match fields_map.get("rootBlockDevices") {
                            Some(value) => value,
                            None => bail!("Missing field 'rootBlockDevices' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spot_price: {
                        let field_value = match fields_map.get("spotPrice") {
                            Some(value) => value,
                            None => bail!("Missing field 'spotPrice' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_id: {
                        let field_value = match fields_map.get("subnetId") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#user_data: {
                        let field_value = match fields_map.get("userData") {
                            Some(value) => value,
                            None => bail!("Missing field 'userData' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_security_group_ids: {
                        let field_value = match fields_map.get("vpcSecurityGroupIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpcSecurityGroupIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weighted_capacity: {
                        let field_value = match fields_map.get("weightedCapacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'weightedCapacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
