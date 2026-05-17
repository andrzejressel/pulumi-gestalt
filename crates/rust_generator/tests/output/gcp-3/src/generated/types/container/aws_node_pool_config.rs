#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AwsNodePoolConfig {
    /// Optional. Configuration related to CloudWatch metrics collection on the Auto Scaling group of the node pool. When unspecified, metrics collection is disabled.
    #[builder(into)]
    #[serde(rename = "autoscalingMetricsCollection")]
    pub r#autoscaling_metrics_collection: Option<Box<super::super::types::container::AwsNodePoolConfigAutoscalingMetricsCollection>>,
    /// The ARN of the AWS KMS key used to encrypt node pool configuration.
    #[builder(into)]
    #[serde(rename = "configEncryption")]
    pub r#config_encryption: Box<super::super::types::container::AwsNodePoolConfigConfigEncryption>,
    /// The name of the AWS IAM role assigned to nodes in the pool.
    #[builder(into)]
    #[serde(rename = "iamInstanceProfile")]
    pub r#iam_instance_profile: String,
    /// The OS image type to use on node pool instances.
    #[builder(into)]
    #[serde(rename = "imageType")]
    pub r#image_type: Option<String>,
    /// Details of placement information for an instance.
    #[builder(into)]
    #[serde(rename = "instancePlacement")]
    pub r#instance_placement: Option<Box<super::super::types::container::AwsNodePoolConfigInstancePlacement>>,
    /// Optional. The AWS instance type. When unspecified, it defaults to `m5.large`.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Option<String>,
    /// Optional. The initial labels assigned to nodes of this node pool. An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// Proxy configuration for outbound HTTP(S) traffic.
    #[builder(into)]
    #[serde(rename = "proxyConfig")]
    pub r#proxy_config: Option<Box<super::super::types::container::AwsNodePoolConfigProxyConfig>>,
    /// Optional. Template for the root volume provisioned for node pool nodes. Volumes will be provisioned in the availability zone assigned to the node pool subnet. When unspecified, it defaults to 32 GiB with the GP2 volume type.
    #[builder(into)]
    #[serde(rename = "rootVolume")]
    pub r#root_volume: Option<Box<super::super::types::container::AwsNodePoolConfigRootVolume>>,
    /// Optional. The IDs of additional security groups to add to nodes in this pool. The manager will automatically create security groups with minimum rules needed for a functioning cluster.
    #[builder(into)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Option<Vec<String>>,
    /// Optional. When specified, the node pool will provision Spot instances from the set of spot_config.instance_types. This field is mutually exclusive with `instance_type`
    #[builder(into)]
    #[serde(rename = "spotConfig")]
    pub r#spot_config: Option<Box<super::super::types::container::AwsNodePoolConfigSpotConfig>>,
    /// Optional. The SSH configuration.
    #[builder(into)]
    #[serde(rename = "sshConfig")]
    pub r#ssh_config: Option<Box<super::super::types::container::AwsNodePoolConfigSshConfig>>,
    /// Optional. Key/value metadata to assign to each underlying AWS resource. Specify at most 50 pairs containing alphanumerics, spaces, and symbols (.+-=_:@/). Keys can be up to 127 Unicode characters. Values can be up to 255 Unicode characters.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    /// Optional. The initial taints assigned to nodes of this node pool.
    #[builder(into)]
    #[serde(rename = "taints")]
    pub r#taints: Option<Vec<super::super::types::container::AwsNodePoolConfigTaint>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AwsNodePoolConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "autoscaling_metrics_collection",
                    &self.r#autoscaling_metrics_collection,
                ),
                to_pulumi_object_field(
                    "config_encryption",
                    &self.r#config_encryption,
                ),
                to_pulumi_object_field(
                    "iam_instance_profile",
                    &self.r#iam_instance_profile,
                ),
                to_pulumi_object_field(
                    "image_type",
                    &self.r#image_type,
                ),
                to_pulumi_object_field(
                    "instance_placement",
                    &self.r#instance_placement,
                ),
                to_pulumi_object_field(
                    "instance_type",
                    &self.r#instance_type,
                ),
                to_pulumi_object_field(
                    "labels",
                    &self.r#labels,
                ),
                to_pulumi_object_field(
                    "proxy_config",
                    &self.r#proxy_config,
                ),
                to_pulumi_object_field(
                    "root_volume",
                    &self.r#root_volume,
                ),
                to_pulumi_object_field(
                    "security_group_ids",
                    &self.r#security_group_ids,
                ),
                to_pulumi_object_field(
                    "spot_config",
                    &self.r#spot_config,
                ),
                to_pulumi_object_field(
                    "ssh_config",
                    &self.r#ssh_config,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
                to_pulumi_object_field(
                    "taints",
                    &self.r#taints,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AwsNodePoolConfig {
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
                    r#autoscaling_metrics_collection: {
                        let field_value = match fields_map.get("autoscaling_metrics_collection") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoscaling_metrics_collection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#config_encryption: {
                        let field_value = match fields_map.get("config_encryption") {
                            Some(value) => value,
                            None => bail!("Missing field 'config_encryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#iam_instance_profile: {
                        let field_value = match fields_map.get("iam_instance_profile") {
                            Some(value) => value,
                            None => bail!("Missing field 'iam_instance_profile' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_type: {
                        let field_value = match fields_map.get("image_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_placement: {
                        let field_value = match fields_map.get("instance_placement") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_placement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#proxy_config: {
                        let field_value = match fields_map.get("proxy_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'proxy_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_volume: {
                        let field_value = match fields_map.get("root_volume") {
                            Some(value) => value,
                            None => bail!("Missing field 'root_volume' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_group_ids: {
                        let field_value = match fields_map.get("security_group_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_group_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spot_config: {
                        let field_value = match fields_map.get("spot_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'spot_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_config: {
                        let field_value = match fields_map.get("ssh_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssh_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#taints: {
                        let field_value = match fields_map.get("taints") {
                            Some(value) => value,
                            None => bail!("Missing field 'taints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
