#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceVolumeConfigurationManagedEbsVolume {
    /// Whether the volume should be encrypted. Default value is `true`.
    #[builder(into)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: Option<bool>,
    /// Linux filesystem type for the volume. For volumes created from a snapshot, same filesystem type must be specified that the volume was using when the snapshot was created. Valid values are `ext3`, `ext4`, `xfs`. Default value is `xfs`.
    #[builder(into)]
    #[serde(rename = "fileSystemType")]
    pub r#file_system_type: Option<String>,
    /// Number of I/O operations per second (IOPS).
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: Option<i32>,
    /// Amazon Resource Name (ARN) identifier of the Amazon Web Services Key Management Service key to use for Amazon EBS encryption.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
    /// Amazon ECS infrastructure IAM role that is used to manage your Amazon Web Services infrastructure. Recommended using the Amazon ECS-managed `AmazonECSInfrastructureRolePolicyForVolumes` IAM policy with this role.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// Size of the volume in GiB. You must specify either a `size_in_gb` or a `snapshot_id`. You can optionally specify a volume size greater than or equal to the snapshot size.
    #[builder(into)]
    #[serde(rename = "sizeInGb")]
    pub r#size_in_gb: Option<i32>,
    /// Snapshot that Amazon ECS uses to create the volume. You must specify either a `size_in_gb` or a `snapshot_id`.
    #[builder(into)]
    #[serde(rename = "snapshotId")]
    pub r#snapshot_id: Option<String>,
    /// The tags to apply to the volume. See below.
    #[builder(into)]
    #[serde(rename = "tagSpecifications")]
    pub r#tag_specifications: Option<Vec<super::super::types::ecs::ServiceVolumeConfigurationManagedEbsVolumeTagSpecification>>,
    /// Throughput to provision for a volume, in MiB/s, with a maximum of 1,000 MiB/s.
    #[builder(into)]
    #[serde(rename = "throughput")]
    pub r#throughput: Option<i32>,
    /// Volume type.
    #[builder(into)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceVolumeConfigurationManagedEbsVolume {
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
                    "encrypted",
                    &self.r#encrypted,
                ),
                to_pulumi_object_field(
                    "file_system_type",
                    &self.r#file_system_type,
                ),
                to_pulumi_object_field(
                    "iops",
                    &self.r#iops,
                ),
                to_pulumi_object_field(
                    "kms_key_id",
                    &self.r#kms_key_id,
                ),
                to_pulumi_object_field(
                    "role_arn",
                    &self.r#role_arn,
                ),
                to_pulumi_object_field(
                    "size_in_gb",
                    &self.r#size_in_gb,
                ),
                to_pulumi_object_field(
                    "snapshot_id",
                    &self.r#snapshot_id,
                ),
                to_pulumi_object_field(
                    "tag_specifications",
                    &self.r#tag_specifications,
                ),
                to_pulumi_object_field(
                    "throughput",
                    &self.r#throughput,
                ),
                to_pulumi_object_field(
                    "volume_type",
                    &self.r#volume_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceVolumeConfigurationManagedEbsVolume {
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
                    r#encrypted: {
                        let field_value = match fields_map.get("encrypted") {
                            Some(value) => value,
                            None => bail!("Missing field 'encrypted' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_system_type: {
                        let field_value = match fields_map.get("file_system_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_system_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#iops: {
                        let field_value = match fields_map.get("iops") {
                            Some(value) => value,
                            None => bail!("Missing field 'iops' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_key_id: {
                        let field_value = match fields_map.get("kms_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role_arn: {
                        let field_value = match fields_map.get("role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#size_in_gb: {
                        let field_value = match fields_map.get("size_in_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'size_in_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snapshot_id: {
                        let field_value = match fields_map.get("snapshot_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'snapshot_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag_specifications: {
                        let field_value = match fields_map.get("tag_specifications") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_specifications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#throughput: {
                        let field_value = match fields_map.get("throughput") {
                            Some(value) => value,
                            None => bail!("Missing field 'throughput' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_type: {
                        let field_value = match fields_map.get("volume_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'volume_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
