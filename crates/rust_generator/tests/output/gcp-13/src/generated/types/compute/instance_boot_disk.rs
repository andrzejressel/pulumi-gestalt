#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceBootDisk {
    /// Whether the disk will be auto-deleted when the instance
    /// is deleted. Defaults to true.
    #[builder(into)]
    #[serde(rename = "autoDelete")]
    pub r#auto_delete: Option<bool>,
    /// Name with which attached disk will be accessible.
    /// On the instance, this device will be `/dev/disk/by-id/google-{{device_name}}`.
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Option<String>,
    /// A 256-bit [customer-supplied encryption key]
    /// (https://cloud.google.com/compute/docs/disks/customer-supplied-encryption),
    /// encoded in [RFC 4648 base64](https://tools.ietf.org/html/rfc4648#section-4)
    /// to encrypt this disk. Only one of `kms_key_self_link` and `disk_encryption_key_raw`
    /// may be set.
    #[builder(into)]
    #[serde(rename = "diskEncryptionKeyRaw")]
    pub r#disk_encryption_key_raw: Option<String>,
    /// The [RFC 4648 base64](https://tools.ietf.org/html/rfc4648#section-4)
    /// encoded SHA-256 hash of the [customer-supplied encryption key]
    /// (https://cloud.google.com/compute/docs/disks/customer-supplied-encryption) that protects this resource.
    #[builder(into)]
    #[serde(rename = "diskEncryptionKeySha256")]
    pub r#disk_encryption_key_sha_256: Option<String>,
    /// Parameters for a new disk that will be created
    /// alongside the new instance. Either `initialize_params` or `source` must be set.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "initializeParams")]
    pub r#initialize_params: Option<Box<super::super::types::compute::InstanceBootDiskInitializeParams>>,
    /// The disk interface used for attaching this disk. One of SCSI or NVME. (This field is shared with attached_disk and only used for specific cases, please don't specify this field without advice from Google.)
    #[builder(into)]
    #[serde(rename = "interface")]
    pub r#interface: Option<String>,
    /// The self_link of the encryption key that is
    /// stored in Google Cloud KMS to encrypt this disk. Only one of `kms_key_self_link`
    /// and `disk_encryption_key_raw` may be set.
    #[builder(into)]
    #[serde(rename = "kmsKeySelfLink")]
    pub r#kms_key_self_link: Option<String>,
    /// The mode in which to attach this disk, either `READ_WRITE`
    /// or `READ_ONLY`. If not specified, the default is to attach the disk in `READ_WRITE` mode.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    /// The name or self_link of the existing disk (such as those managed by
    /// `gcp.compute.Disk`) or disk image. To create an instance from a snapshot, first create a
    /// `gcp.compute.Disk` from a snapshot and reference it here.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceBootDisk {
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
                    "device_name",
                    &self.r#device_name,
                ),
                to_pulumi_object_field(
                    "disk_encryption_key_raw",
                    &self.r#disk_encryption_key_raw,
                ),
                to_pulumi_object_field(
                    "disk_encryption_key_sha_256",
                    &self.r#disk_encryption_key_sha_256,
                ),
                to_pulumi_object_field(
                    "initialize_params",
                    &self.r#initialize_params,
                ),
                to_pulumi_object_field(
                    "interface",
                    &self.r#interface,
                ),
                to_pulumi_object_field(
                    "kms_key_self_link",
                    &self.r#kms_key_self_link,
                ),
                to_pulumi_object_field(
                    "mode",
                    &self.r#mode,
                ),
                to_pulumi_object_field(
                    "source",
                    &self.r#source,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceBootDisk {
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
                    r#device_name: {
                        let field_value = match fields_map.get("device_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_encryption_key_raw: {
                        let field_value = match fields_map.get("disk_encryption_key_raw") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_encryption_key_raw' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_encryption_key_sha_256: {
                        let field_value = match fields_map.get("disk_encryption_key_sha_256") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_encryption_key_sha_256' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#initialize_params: {
                        let field_value = match fields_map.get("initialize_params") {
                            Some(value) => value,
                            None => bail!("Missing field 'initialize_params' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#kms_key_self_link: {
                        let field_value = match fields_map.get("kms_key_self_link") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key_self_link' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#source: {
                        let field_value = match fields_map.get("source") {
                            Some(value) => value,
                            None => bail!("Missing field 'source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
