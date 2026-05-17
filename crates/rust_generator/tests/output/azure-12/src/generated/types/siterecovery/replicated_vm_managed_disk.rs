#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReplicatedVmManagedDisk {
    /// Id of disk that should be replicated. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "diskId")]
    pub r#disk_id: String,
    /// Storage account that should be used for caching. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "stagingStorageAccountId")]
    pub r#staging_storage_account_id: String,
    /// A `target_disk_encryption` block as defined below.
    #[builder(into)]
    #[serde(rename = "targetDiskEncryption")]
    pub r#target_disk_encryption: Option<Box<super::super::types::siterecovery::ReplicatedVmManagedDiskTargetDiskEncryption>>,
    /// The Disk Encryption Set that the Managed Disk will be associated with. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** Creating replicated vm with `target_disk_encryption_set_id` wil take more time (up to 5 hours), please extend the `timeout` for `create`.
    #[builder(into)]
    #[serde(rename = "targetDiskEncryptionSetId")]
    pub r#target_disk_encryption_set_id: Option<String>,
    /// What type should the disk be when a failover is done. Possible values are `Standard_LRS`, `Premium_LRS`, `StandardSSD_LRS` and `UltraSSD_LRS`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "targetDiskType")]
    pub r#target_disk_type: String,
    /// What type should the disk be that holds the replication data. Possible values are `Standard_LRS`, `Premium_LRS`, `StandardSSD_LRS` and `UltraSSD_LRS`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "targetReplicaDiskType")]
    pub r#target_replica_disk_type: String,
    /// Resource group disk should belong to when a failover is done. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "targetResourceGroupId")]
    pub r#target_resource_group_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ReplicatedVmManagedDisk {
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
                    "disk_id",
                    &self.r#disk_id,
                ),
                to_pulumi_object_field(
                    "staging_storage_account_id",
                    &self.r#staging_storage_account_id,
                ),
                to_pulumi_object_field(
                    "target_disk_encryption",
                    &self.r#target_disk_encryption,
                ),
                to_pulumi_object_field(
                    "target_disk_encryption_set_id",
                    &self.r#target_disk_encryption_set_id,
                ),
                to_pulumi_object_field(
                    "target_disk_type",
                    &self.r#target_disk_type,
                ),
                to_pulumi_object_field(
                    "target_replica_disk_type",
                    &self.r#target_replica_disk_type,
                ),
                to_pulumi_object_field(
                    "target_resource_group_id",
                    &self.r#target_resource_group_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ReplicatedVmManagedDisk {
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
                    r#staging_storage_account_id: {
                        let field_value = match fields_map.get("staging_storage_account_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'staging_storage_account_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_disk_encryption: {
                        let field_value = match fields_map.get("target_disk_encryption") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_disk_encryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#target_replica_disk_type: {
                        let field_value = match fields_map.get("target_replica_disk_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_replica_disk_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_resource_group_id: {
                        let field_value = match fields_map.get("target_resource_group_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_resource_group_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
