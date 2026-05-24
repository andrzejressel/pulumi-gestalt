#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceTemplateDisk {
    /// Whether or not the disk should be auto-deleted.
    /// This defaults to true.
    #[builder(into)]
    pub r#auto_delete: Option<bool>,
    /// Indicates that this is a boot disk.
    #[builder(into)]
    pub r#boot: Option<bool>,
    /// A unique device name that is reflected into the
    /// /dev/  tree of a Linux operating system running within the instance. If not
    /// specified, the server chooses a default device name to apply to this disk.
    #[builder(into)]
    pub r#device_name: Option<String>,
    /// Encrypts or decrypts a disk using a customer-supplied encryption key.
    /// 
    /// If you are creating a new disk, this field encrypts the new disk using an encryption key that you provide. If you are attaching an existing disk that is already encrypted, this field decrypts the disk using the customer-supplied encryption key.
    /// 
    /// If you encrypt a disk using a customer-supplied key, you must provide the same key again when you attempt to use this resource at a later time. For example, you must provide the key when you create a snapshot or an image from the disk or when you attach the disk to a virtual machine instance.
    /// 
    /// If you do not provide an encryption key, then the disk will be encrypted using an automatically generated key and you do not need to provide a key to use the disk later.
    /// 
    /// Instance templates do not store customer-supplied encryption keys, so you cannot use your own keys to encrypt disks in a managed instance group. Structure documented below.
    #[builder(into)]
    pub r#disk_encryption_key: Option<Box<super::super::types::compute::InstanceTemplateDiskDiskEncryptionKey>>,
    /// Name of the disk. When not provided, this defaults
    /// to the name of the instance.
    #[builder(into)]
    pub r#disk_name: Option<String>,
    /// The size of the image in gigabytes. If not
    /// specified, it will inherit the size of its base image. For SCRATCH disks,
    /// the size must be exactly 375GB.
    #[builder(into)]
    pub r#disk_size_gb: Option<i32>,
    /// The GCE disk type. Such as `"pd-ssd"`, `"local-ssd"`,
    /// `"pd-balanced"` or `"pd-standard"`, `"hyperdisk-balanced"`, `"hyperdisk-throughput"` or `"hyperdisk-extreme"`.
    #[builder(into)]
    pub r#disk_type: Option<String>,
    /// Specifies the disk interface to use for attaching this disk,
    /// which is either SCSI or NVME. The default is SCSI. Persistent disks must always use SCSI
    /// and the request will fail if you attempt to attach a persistent disk in any other format
    /// than SCSI. Local SSDs can use either NVME or SCSI.
    #[builder(into)]
    pub r#interface: Option<String>,
    /// A set of ket/value label pairs to assign to disk created from
    /// this template
    #[builder(into)]
    pub r#labels: Option<std::collections::BTreeMap<String, String>>,
    /// The mode in which to attach this disk, either READ_WRITE
    /// or READ_ONLY. If you are attaching or creating a boot disk, this must
    /// read-write mode.
    #[builder(into)]
    pub r#mode: Option<String>,
    /// Indicates how many IOPS to provision for the disk. This
    /// sets the number of I/O operations per second that the disk can handle.
    /// Values must be between 10,000 and 120,000. For more details, see the
    /// [Extreme persistent disk documentation](https://cloud.google.com/compute/docs/disks/extreme-persistent-disk).
    #[builder(into)]
    pub r#provisioned_iops: Option<i32>,
    /// Indicates how much throughput to provision for the disk, in MB/s. This sets the amount of data that can be read or written from the disk per second. Values must greater than or equal to 1. For more details, see the [Hyperdisk documentation](https://cloud.google.com/compute/docs/disks/hyperdisks).
    #[builder(into)]
    pub r#provisioned_throughput: Option<i32>,
    /// A set of key/value resource manager tag pairs to bind to this disk. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456.
    #[builder(into)]
    pub r#resource_manager_tags: Option<std::collections::BTreeMap<String, String>>,
    /// - A list (short name or id) of resource policies to attach to this disk for automatic snapshot creations. Currently a max of 1 resource policy is supported.
    #[builder(into)]
    pub r#resource_policies: Option<String>,
    /// The name (**not self_link**)
    /// of the disk (such as those managed by `gcp.compute.Disk`) to attach.
    /// > **Note:** Either `source`, `source_image`, or `source_snapshot` is **required** in a disk block unless the disk type is `local-ssd`. Check the API [docs](https://cloud.google.com/compute/docs/reference/rest/v1/instanceTemplates/insert) for details.
    #[builder(into)]
    pub r#source: Option<String>,
    /// The image from which to
    /// initialize this disk. This can be one of: the image's `self_link`,
    /// `projects/{project}/global/images/{image}`,
    /// `projects/{project}/global/images/family/{family}`, `global/images/{image}`,
    /// `global/images/family/{family}`, `family/{family}`, `{project}/{family}`,
    /// `{project}/{image}`, `{family}`, or `{image}`.
    /// > **Note:** Either `source`, `source_image`, or `source_snapshot` is **required** in a disk block unless the disk type is `local-ssd`. Check the API [docs](https://cloud.google.com/compute/docs/reference/rest/v1/instanceTemplates/insert) for details.
    #[builder(into)]
    pub r#source_image: Option<String>,
    /// The customer-supplied encryption
    /// key of the source image. Required if the source image is protected by a
    /// customer-supplied encryption key.
    /// 
    /// Instance templates do not store customer-supplied encryption keys, so you
    /// cannot create disks for instances in a managed instance group if the source
    /// images are encrypted with your own keys. Structure
    /// documented below.
    #[builder(into)]
    pub r#source_image_encryption_key: Option<Box<super::super::types::compute::InstanceTemplateDiskSourceImageEncryptionKey>>,
    /// The source snapshot to create this disk.
    /// > **Note:** Either `source`, `source_image`, or `source_snapshot` is **required** in a disk block unless the disk type is `local-ssd`. Check the API [docs](https://cloud.google.com/compute/docs/reference/rest/v1/instanceTemplates/insert) for details.
    #[builder(into)]
    pub r#source_snapshot: Option<String>,
    /// The customer-supplied encryption
    /// key of the source snapshot. Structure
    /// documented below.
    #[builder(into)]
    pub r#source_snapshot_encryption_key: Option<Box<super::super::types::compute::InstanceTemplateDiskSourceSnapshotEncryptionKey>>,
    /// The type of GCE disk, can be either `"SCRATCH"` or
    /// `"PERSISTENT"`.
    #[builder(into)]
    pub r#type_: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceTemplateDisk {
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
                    "autoDelete",
                    &self.r#auto_delete,
                ),
                to_pulumi_object_field(
                    "boot",
                    &self.r#boot,
                ),
                to_pulumi_object_field(
                    "deviceName",
                    &self.r#device_name,
                ),
                to_pulumi_object_field(
                    "diskEncryptionKey",
                    &self.r#disk_encryption_key,
                ),
                to_pulumi_object_field(
                    "diskName",
                    &self.r#disk_name,
                ),
                to_pulumi_object_field(
                    "diskSizeGb",
                    &self.r#disk_size_gb,
                ),
                to_pulumi_object_field(
                    "diskType",
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
                    "provisionedIops",
                    &self.r#provisioned_iops,
                ),
                to_pulumi_object_field(
                    "provisionedThroughput",
                    &self.r#provisioned_throughput,
                ),
                to_pulumi_object_field(
                    "resourceManagerTags",
                    &self.r#resource_manager_tags,
                ),
                to_pulumi_object_field(
                    "resourcePolicies",
                    &self.r#resource_policies,
                ),
                to_pulumi_object_field(
                    "source",
                    &self.r#source,
                ),
                to_pulumi_object_field(
                    "sourceImage",
                    &self.r#source_image,
                ),
                to_pulumi_object_field(
                    "sourceImageEncryptionKey",
                    &self.r#source_image_encryption_key,
                ),
                to_pulumi_object_field(
                    "sourceSnapshot",
                    &self.r#source_snapshot,
                ),
                to_pulumi_object_field(
                    "sourceSnapshotEncryptionKey",
                    &self.r#source_snapshot_encryption_key,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceTemplateDisk {
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
                        let field_value = match fields_map.get("autoDelete") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoDelete' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("deviceName") {
                            Some(value) => value,
                            None => bail!("Missing field 'deviceName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_encryption_key: {
                        let field_value = match fields_map.get("diskEncryptionKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskEncryptionKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_name: {
                        let field_value = match fields_map.get("diskName") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_size_gb: {
                        let field_value = match fields_map.get("diskSizeGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskSizeGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_type: {
                        let field_value = match fields_map.get("diskType") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("provisionedIops") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisionedIops' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioned_throughput: {
                        let field_value = match fields_map.get("provisionedThroughput") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisionedThroughput' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_manager_tags: {
                        let field_value = match fields_map.get("resourceManagerTags") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceManagerTags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_policies: {
                        let field_value = match fields_map.get("resourcePolicies") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourcePolicies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("sourceImage") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceImage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_image_encryption_key: {
                        let field_value = match fields_map.get("sourceImageEncryptionKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceImageEncryptionKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_snapshot: {
                        let field_value = match fields_map.get("sourceSnapshot") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceSnapshot' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_snapshot_encryption_key: {
                        let field_value = match fields_map.get("sourceSnapshotEncryptionKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceSnapshotEncryptionKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
