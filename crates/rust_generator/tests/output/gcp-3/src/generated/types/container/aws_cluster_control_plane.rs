#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AwsClusterControlPlane {
    /// Authentication configuration for management of AWS resources.
    #[builder(into)]
    #[serde(rename = "awsServicesAuthentication")]
    pub r#aws_services_authentication: Box<super::super::types::container::AwsClusterControlPlaneAwsServicesAuthentication>,
    /// The ARN of the AWS KMS key used to encrypt cluster configuration.
    #[builder(into)]
    #[serde(rename = "configEncryption")]
    pub r#config_encryption: Box<super::super::types::container::AwsClusterControlPlaneConfigEncryption>,
    /// The ARN of the AWS KMS key used to encrypt cluster secrets.
    #[builder(into)]
    #[serde(rename = "databaseEncryption")]
    pub r#database_encryption: Box<super::super::types::container::AwsClusterControlPlaneDatabaseEncryption>,
    /// The name of the AWS IAM instance pofile to assign to each control plane replica.
    #[builder(into)]
    #[serde(rename = "iamInstanceProfile")]
    pub r#iam_instance_profile: String,
    /// Details of placement information for an instance.
    #[builder(into)]
    #[serde(rename = "instancePlacement")]
    pub r#instance_placement: Option<Box<super::super::types::container::AwsClusterControlPlaneInstancePlacement>>,
    /// Optional. The AWS instance type. When unspecified, it defaults to `m5.large`.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Option<String>,
    /// Optional. Configuration related to the main volume provisioned for each control plane replica. The main volume is in charge of storing all of the cluster's etcd state. Volumes will be provisioned in the availability zone associated with the corresponding subnet. When unspecified, it defaults to 8 GiB with the GP2 volume type.
    #[builder(into)]
    #[serde(rename = "mainVolume")]
    pub r#main_volume: Option<Box<super::super::types::container::AwsClusterControlPlaneMainVolume>>,
    /// Proxy configuration for outbound HTTP(S) traffic.
    #[builder(into)]
    #[serde(rename = "proxyConfig")]
    pub r#proxy_config: Option<Box<super::super::types::container::AwsClusterControlPlaneProxyConfig>>,
    /// Optional. Configuration related to the root volume provisioned for each control plane replica. Volumes will be provisioned in the availability zone associated with the corresponding subnet. When unspecified, it defaults to 32 GiB with the GP2 volume type.
    #[builder(into)]
    #[serde(rename = "rootVolume")]
    pub r#root_volume: Option<Box<super::super::types::container::AwsClusterControlPlaneRootVolume>>,
    /// Optional. The IDs of additional security groups to add to control plane replicas. The Anthos Multi-Cloud API will automatically create and manage security groups with the minimum rules needed for a functioning cluster.
    #[builder(into)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Option<Vec<String>>,
    /// Optional. SSH configuration for how to access the underlying control plane machines.
    #[builder(into)]
    #[serde(rename = "sshConfig")]
    pub r#ssh_config: Option<Box<super::super::types::container::AwsClusterControlPlaneSshConfig>>,
    /// The list of subnets where control plane replicas will run. A replica will be provisioned on each subnet and up to three values can be provided. Each subnet must be in a different AWS Availability Zone (AZ).
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Vec<String>,
    /// Optional. A set of AWS resource tags to propagate to all underlying managed AWS resources. Specify at most 50 pairs containing alphanumerics, spaces, and symbols (.+-=_:@/). Keys can be up to 127 Unicode characters. Values can be up to 255 Unicode characters.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    /// The Kubernetes version to run on control plane replicas (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling .
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AwsClusterControlPlane {
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
                    "aws_services_authentication",
                    &self.r#aws_services_authentication,
                ),
                to_pulumi_object_field(
                    "config_encryption",
                    &self.r#config_encryption,
                ),
                to_pulumi_object_field(
                    "database_encryption",
                    &self.r#database_encryption,
                ),
                to_pulumi_object_field(
                    "iam_instance_profile",
                    &self.r#iam_instance_profile,
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
                    "main_volume",
                    &self.r#main_volume,
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
                    "ssh_config",
                    &self.r#ssh_config,
                ),
                to_pulumi_object_field(
                    "subnet_ids",
                    &self.r#subnet_ids,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AwsClusterControlPlane {
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
                    r#aws_services_authentication: {
                        let field_value = match fields_map.get("aws_services_authentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_services_authentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#database_encryption: {
                        let field_value = match fields_map.get("database_encryption") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_encryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#main_volume: {
                        let field_value = match fields_map.get("main_volume") {
                            Some(value) => value,
                            None => bail!("Missing field 'main_volume' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#ssh_config: {
                        let field_value = match fields_map.get("ssh_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssh_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_ids: {
                        let field_value = match fields_map.get("subnet_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
