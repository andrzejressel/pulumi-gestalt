#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceAttachedDisk {
    /// Name with which the attached disk will be accessible
    /// under `/dev/disk/by-id/google-*`
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Option<String>,
    /// A 256-bit [customer-supplied encryption key]
    /// (https://cloud.google.com/compute/docs/disks/customer-supplied-encryption),
    /// encoded in [RFC 4648 base64](https://tools.ietf.org/html/rfc4648#section-4)
    /// to encrypt this disk. Only one of `kms_key_self_link` and `disk_encryption_key_raw` may be set.
    #[builder(into)]
    #[serde(rename = "diskEncryptionKeyRaw")]
    pub r#disk_encryption_key_raw: Option<String>,
    /// The [RFC 4648 base64](https://tools.ietf.org/html/rfc4648#section-4)
    /// encoded SHA-256 hash of the [customer-supplied encryption key]
    /// (https://cloud.google.com/compute/docs/disks/customer-supplied-encryption) that protects this resource.
    #[builder(into)]
    #[serde(rename = "diskEncryptionKeySha256")]
    pub r#disk_encryption_key_sha_256: Option<String>,
    /// The self_link of the encryption key that is
    /// stored in Google Cloud KMS to encrypt this disk. Only one of `kms_key_self_link`
    /// and `disk_encryption_key_raw` may be set.
    #[builder(into)]
    #[serde(rename = "kmsKeySelfLink")]
    pub r#kms_key_self_link: Option<String>,
    /// Either "READ_ONLY" or "READ_WRITE", defaults to "READ_WRITE"
    /// If you have a persistent disk with data that you want to share
    /// between multiple instances, detach it from any read-write instances and
    /// attach it to one or more instances in read-only mode.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    /// The name or self_link of the disk to attach to this instance.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceAttachedDisk {
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceAttachedDisk {
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
