#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetInstanceTemplateDisk {
    /// Whether or not the disk should be auto-deleted.
    /// This defaults to true.
    #[builder(into)]
    #[serde(rename = "autoDelete")]
    pub r#auto_delete: bool,
    /// Indicates that this is a boot disk.
    #[builder(into)]
    #[serde(rename = "boot")]
    pub r#boot: bool,
    /// A unique device name that is reflected into the
    /// /dev/  tree of a Linux operating system running within the instance. If not
    /// specified, the server chooses a default device name to apply to this disk.
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: String,
    /// Encrypts or decrypts a disk using a customer-supplied encryption key.
    #[builder(into)]
    #[serde(rename = "diskEncryptionKeys")]
    pub r#disk_encryption_keys: Vec<super::super::types::compute::GetInstanceTemplateDiskDiskEncryptionKey>,
    /// Name of the disk. When not provided, this defaults
    /// to the name of the instance.
    #[builder(into)]
    #[serde(rename = "diskName")]
    pub r#disk_name: String,
    /// The size of the image in gigabytes. If not
    /// specified, it will inherit the size of its base image. For SCRATCH disks,
    /// the size must be exactly 375GB.
    #[builder(into)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: i32,
    /// The GCE disk type. Such as `"pd-ssd"`, `"local-ssd"`,
    /// `"pd-balanced"` or `"pd-standard"`.
    #[builder(into)]
    #[serde(rename = "diskType")]
    pub r#disk_type: String,
    /// Specifies the disk interface to use for attaching this disk,
    /// which is either SCSI or NVME. The default is SCSI. Persistent disks must always use SCSI
    /// and the request will fail if you attempt to attach a persistent disk in any other format
    /// than SCSI. Local SSDs can use either NVME or SCSI.
    #[builder(into)]
    #[serde(rename = "interface")]
    pub r#interface: String,
    /// (Optional) A set of ket/value label pairs to assign to disk created from
    /// this template
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: std::collections::HashMap<String, String>,
    /// The mode in which to attach this disk, either READ_WRITE
    /// or READ_ONLY. If you are attaching or creating a boot disk, this must
    /// read-write mode.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: String,
    /// Indicates how many IOPS to provision for the disk. This
    /// sets the number of I/O operations per second that the disk can handle.
    /// Values must be between 10,000 and 120,000. For more details, see the
    /// [Extreme persistent disk documentation](https://cloud.google.com/compute/docs/disks/extreme-persistent-disk).
    #[builder(into)]
    #[serde(rename = "provisionedIops")]
    pub r#provisioned_iops: i32,
    /// Indicates how much throughput to provision for the disk, in MB/s. This sets the amount of data that can be read or written from the disk per second. Values must greater than or equal to 1. For more details, see the [Hyperdisk documentation](https://cloud.google.com/compute/docs/disks/hyperdisks).
    #[builder(into)]
    #[serde(rename = "provisionedThroughput")]
    pub r#provisioned_throughput: i32,
    /// A map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty.
    #[builder(into)]
    #[serde(rename = "resourceManagerTags")]
    pub r#resource_manager_tags: std::collections::HashMap<String, String>,
    /// (Optional) -- A list of short names of resource policies to attach to this disk for automatic snapshot creations. Currently a max of 1 resource policy is supported.
    #[builder(into)]
    #[serde(rename = "resourcePolicies")]
    pub r#resource_policies: Vec<String>,
    /// The name (**not self_link**)
    /// of the disk (such as those managed by `gcp.compute.Disk`) to attach.
    /// > **Note:** Either `source` or `source_image` is **required** in a disk block unless the disk type is `local-ssd`. Check the API [docs](https://cloud.google.com/compute/docs/reference/rest/v1/instanceTemplates/insert) for details.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: String,
    /// The image from which to
    /// initialize this disk. This can be one of: the image's `self_link`,
    /// `projects/{project}/global/images/{image}`,
    /// `projects/{project}/global/images/family/{family}`, `global/images/{image}`,
    /// `global/images/family/{family}`, `family/{family}`, `{project}/{family}`,
    /// `{project}/{image}`, `{family}`, or `{image}`.
    /// > **Note:** Either `source` or `source_image` is **required** in a disk block unless the disk type is `local-ssd`. Check the API [docs](https://cloud.google.com/compute/docs/reference/rest/v1/instanceTemplates/insert) for details.
    #[builder(into)]
    #[serde(rename = "sourceImage")]
    pub r#source_image: String,
    /// The customer-supplied encryption key of the source
    /// image. Required if the source image is protected by a
    /// customer-supplied encryption key.
    /// 
    /// Instance templates do not store customer-supplied
    /// encryption keys, so you cannot create disks for
    /// instances in a managed instance group if the source
    /// images are encrypted with your own keys.
    #[builder(into)]
    #[serde(rename = "sourceImageEncryptionKeys")]
    pub r#source_image_encryption_keys: Vec<super::super::types::compute::GetInstanceTemplateDiskSourceImageEncryptionKey>,
    /// The source snapshot to create this disk. When creating
    /// a new instance, one of initializeParams.sourceSnapshot,
    /// initializeParams.sourceImage, or disks.source is
    /// required except for local SSD.
    #[builder(into)]
    #[serde(rename = "sourceSnapshot")]
    pub r#source_snapshot: String,
    /// The customer-supplied encryption key of the source snapshot.
    #[builder(into)]
    #[serde(rename = "sourceSnapshotEncryptionKeys")]
    pub r#source_snapshot_encryption_keys: Vec<super::super::types::compute::GetInstanceTemplateDiskSourceSnapshotEncryptionKey>,
    /// The accelerator type resource to expose to this instance. E.g. `nvidia-tesla-k80`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetInstanceTemplateDisk {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "auto_delete",
                    &self.r#auto_delete,
                ),
                to_pulumi_object_field(
                    "boot",
                    &self.r#boot,
                ),
                to_pulumi_object_field(
                    "device_name",
                    &self.r#device_name,
                ),
                to_pulumi_object_field(
                    "disk_encryption_keys",
                    &self.r#disk_encryption_keys,
                ),
                to_pulumi_object_field(
                    "disk_name",
                    &self.r#disk_name,
                ),
                to_pulumi_object_field(
                    "disk_size_gb",
                    &self.r#disk_size_gb,
                ),
                to_pulumi_object_field(
                    "disk_type",
                    &self.r#disk_type,
                ),
                to_pulumi_object_field(
                    "interface",
                    &self.r#interface,
                ),
                to_pulumi_object_field(
                    "labels",
                    &self.r#labels,
                ),
                to_pulumi_object_field(
                    "mode",
                    &self.r#mode,
                ),
                to_pulumi_object_field(
                    "provisioned_iops",
                    &self.r#provisioned_iops,
                ),
                to_pulumi_object_field(
                    "provisioned_throughput",
                    &self.r#provisioned_throughput,
                ),
                to_pulumi_object_field(
                    "resource_manager_tags",
                    &self.r#resource_manager_tags,
                ),
                to_pulumi_object_field(
                    "resource_policies",
                    &self.r#resource_policies,
                ),
                to_pulumi_object_field(
                    "source",
                    &self.r#source,
                ),
                to_pulumi_object_field(
                    "source_image",
                    &self.r#source_image,
                ),
                to_pulumi_object_field(
                    "source_image_encryption_keys",
                    &self.r#source_image_encryption_keys,
                ),
                to_pulumi_object_field(
                    "source_snapshot",
                    &self.r#source_snapshot,
                ),
                to_pulumi_object_field(
                    "source_snapshot_encryption_keys",
                    &self.r#source_snapshot_encryption_keys,
                ),
                to_pulumi_object_field(
                    "type_",
                    &self.r#type_,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetInstanceTemplateDisk {
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
                    r#auto_delete: {
                        let field_value = match fields_map.get("auto_delete") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_delete' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#boot: {
                        let field_value = match fields_map.get("boot") {
                            Some(value) => value,
                            None => bail!("Missing field 'boot' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#device_name: {
                        let field_value = match fields_map.get("device_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_encryption_keys: {
                        let field_value = match fields_map.get("disk_encryption_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_encryption_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_name: {
                        let field_value = match fields_map.get("disk_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_size_gb: {
                        let field_value = match fields_map.get("disk_size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_type: {
                        let field_value = match fields_map.get("disk_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#interface: {
                        let field_value = match fields_map.get("interface") {
                            Some(value) => value,
                            None => bail!("Missing field 'interface' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#mode: {
                        let field_value = match fields_map.get("mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioned_iops: {
                        let field_value = match fields_map.get("provisioned_iops") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioned_iops' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioned_throughput: {
                        let field_value = match fields_map.get("provisioned_throughput") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioned_throughput' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_manager_tags: {
                        let field_value = match fields_map.get("resource_manager_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_manager_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_policies: {
                        let field_value = match fields_map.get("resource_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source: {
                        let field_value = match fields_map.get("source") {
                            Some(value) => value,
                            None => bail!("Missing field 'source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_image: {
                        let field_value = match fields_map.get("source_image") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_image' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_image_encryption_keys: {
                        let field_value = match fields_map.get("source_image_encryption_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_image_encryption_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_snapshot: {
                        let field_value = match fields_map.get("source_snapshot") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_snapshot' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_snapshot_encryption_keys: {
                        let field_value = match fields_map.get("source_snapshot_encryption_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_snapshot_encryption_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
