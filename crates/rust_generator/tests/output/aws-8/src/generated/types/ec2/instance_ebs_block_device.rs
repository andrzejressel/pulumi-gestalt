#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceEbsBlockDevice {
    /// Whether the volume should be destroyed on instance termination. Defaults to `true`.
    #[builder(into)]
    pub r#delete_on_termination: Option<bool>,
    /// Name of the device to mount.
    #[builder(into)]
    pub r#device_name: String,
    /// Enables [EBS encryption](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html) on the volume. Defaults to `false`. Cannot be used with `snapshot_id`. Must be configured to perform drift detection.
    #[builder(into)]
    pub r#encrypted: Option<bool>,
    /// Amount of provisioned [IOPS](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-io-characteristics.html). Only valid for volume_type of `io1`, `io2` or `gp3`.
    #[builder(into)]
    pub r#iops: Option<i32>,
    /// Amazon Resource Name (ARN) of the KMS Key to use when encrypting the volume. Must be configured to perform drift detection.
    #[builder(into)]
    pub r#kms_key_id: Option<String>,
    /// Snapshot ID to mount.
    #[builder(into)]
    pub r#snapshot_id: Option<String>,
    /// Map of tags to assign to the device.
    #[builder(into)]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
    #[builder(into)]
    pub r#tags_all: Option<std::collections::HashMap<String, String>>,
    /// Throughput to provision for a volume in mebibytes per second (MiB/s). This is only valid for `volume_type` of `gp3`.
    #[builder(into)]
    pub r#throughput: Option<i32>,
    /// ID of the volume. For example, the ID can be accessed like this, `aws_instance.web.root_block_device.0.volume_id`.
    #[builder(into)]
    pub r#volume_id: Option<String>,
    /// Size of the volume in gibibytes (GiB).
    #[builder(into)]
    pub r#volume_size: Option<i32>,
    /// Type of volume. Valid values include `standard`, `gp2`, `gp3`, `io1`, `io2`, `sc1`, or `st1`. Defaults to `gp2`.
    /// 
    /// > **NOTE:** Currently, changes to the `ebs_block_device` configuration of _existing_ resources cannot be automatically detected by this provider. To manage changes and attachments of an EBS block to an instance, use the `aws.ebs.Volume` and `aws.ec2.VolumeAttachment` resources instead. If you use `ebs_block_device` on an `aws.ec2.Instance`, this provider will assume management over the full set of non-root EBS block devices for the instance, treating additional block devices as drift. For this reason, `ebs_block_device` cannot be mixed with external `aws.ebs.Volume` and `aws.ec2.VolumeAttachment` resources for a given instance.
    #[builder(into)]
    pub r#volume_type: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceEbsBlockDevice {
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
                    "deleteOnTermination",
                    &self.r#delete_on_termination,
                ),
                to_pulumi_object_field(
                    "deviceName",
                    &self.r#device_name,
                ),
                to_pulumi_object_field(
                    "encrypted",
                    &self.r#encrypted,
                ),
                to_pulumi_object_field(
                    "iops",
                    &self.r#iops,
                ),
                to_pulumi_object_field(
                    "kmsKeyId",
                    &self.r#kms_key_id,
                ),
                to_pulumi_object_field(
                    "snapshotId",
                    &self.r#snapshot_id,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
                to_pulumi_object_field(
                    "tagsAll",
                    &self.r#tags_all,
                ),
                to_pulumi_object_field(
                    "throughput",
                    &self.r#throughput,
                ),
                to_pulumi_object_field(
                    "volumeId",
                    &self.r#volume_id,
                ),
                to_pulumi_object_field(
                    "volumeSize",
                    &self.r#volume_size,
                ),
                to_pulumi_object_field(
                    "volumeType",
                    &self.r#volume_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceEbsBlockDevice {
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
                    r#delete_on_termination: {
                        let field_value = match fields_map.get("deleteOnTermination") {
                            Some(value) => value,
                            None => bail!("Missing field 'deleteOnTermination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#device_name: {
                        let field_value = match fields_map.get("deviceName") {
                            Some(value) => value,
                            None => bail!("Missing field 'deviceName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encrypted: {
                        let field_value = match fields_map.get("encrypted") {
                            Some(value) => value,
                            None => bail!("Missing field 'encrypted' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("kmsKeyId") {
                            Some(value) => value,
                            None => bail!("Missing field 'kmsKeyId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snapshot_id: {
                        let field_value = match fields_map.get("snapshotId") {
                            Some(value) => value,
                            None => bail!("Missing field 'snapshotId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#tags_all: {
                        let field_value = match fields_map.get("tagsAll") {
                            Some(value) => value,
                            None => bail!("Missing field 'tagsAll' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#volume_id: {
                        let field_value = match fields_map.get("volumeId") {
                            Some(value) => value,
                            None => bail!("Missing field 'volumeId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_size: {
                        let field_value = match fields_map.get("volumeSize") {
                            Some(value) => value,
                            None => bail!("Missing field 'volumeSize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_type: {
                        let field_value = match fields_map.get("volumeType") {
                            Some(value) => value,
                            None => bail!("Missing field 'volumeType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
