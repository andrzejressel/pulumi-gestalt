#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VmwareReplicatedVmManagedDisk {
    /// The ID of the disk to be replicated.
    #[builder(into)]
    #[serde(rename = "diskId")]
    pub r#disk_id: String,
    /// The ID of the storage account that should be used for logging during replication.
    #[builder(into)]
    #[serde(rename = "logStorageAccountId")]
    pub r#log_storage_account_id: Option<String>,
    /// The ID of the Disk Encryption Set that should be used for the disks when a failover is done.
    #[builder(into)]
    #[serde(rename = "targetDiskEncryptionSetId")]
    pub r#target_disk_encryption_set_id: Option<String>,
    /// The disk type of the disk to be created when a failover is done. Possible values are `Premium_LRS`, `Standard_LRS` and `StandardSSD_LRS`.
    #[builder(into)]
    #[serde(rename = "targetDiskType")]
    pub r#target_disk_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VmwareReplicatedVmManagedDisk {
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
                    "disk_id",
                    &self.r#disk_id,
                ),
                to_pulumi_object_field(
                    "log_storage_account_id",
                    &self.r#log_storage_account_id,
                ),
                to_pulumi_object_field(
                    "target_disk_encryption_set_id",
                    &self.r#target_disk_encryption_set_id,
                ),
                to_pulumi_object_field(
                    "target_disk_type",
                    &self.r#target_disk_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VmwareReplicatedVmManagedDisk {
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
                    r#disk_id: {
                        let field_value = match fields_map.get("disk_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_storage_account_id: {
                        let field_value = match fields_map.get("log_storage_account_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_storage_account_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_disk_encryption_set_id: {
                        let field_value = match fields_map.get("target_disk_encryption_set_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_disk_encryption_set_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_disk_type: {
                        let field_value = match fields_map.get("target_disk_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_disk_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
