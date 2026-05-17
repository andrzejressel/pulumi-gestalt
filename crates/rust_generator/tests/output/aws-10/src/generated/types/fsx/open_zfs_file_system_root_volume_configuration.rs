#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OpenZfsFileSystemRootVolumeConfiguration {
    /// A boolean flag indicating whether tags for the file system should be copied to snapshots. The default value is false.
    #[builder(into)]
    #[serde(rename = "copyTagsToSnapshots")]
    pub r#copy_tags_to_snapshots: Option<bool>,
    /// Method used to compress the data on the volume. Valid values are `LZ4`, `NONE` or `ZSTD`. Child volumes that don't specify compression option will inherit from parent volume. This option on file system applies to the root volume.
    #[builder(into)]
    #[serde(rename = "dataCompressionType")]
    pub r#data_compression_type: Option<String>,
    /// NFS export configuration for the root volume. Exactly 1 item. See `nfs_exports` Block for details.
    #[builder(into)]
    #[serde(rename = "nfsExports")]
    pub r#nfs_exports: Option<Box<super::super::types::fsx::OpenZfsFileSystemRootVolumeConfigurationNfsExports>>,
    /// specifies whether the volume is read-only. Default is false.
    #[builder(into)]
    #[serde(rename = "readOnly")]
    pub r#read_only: Option<bool>,
    /// Specifies the record size of an OpenZFS root volume, in kibibytes (KiB). Valid values are `4`, `8`, `16`, `32`, `64`, `128`, `256`, `512`, or `1024` KiB. The default is `128` KiB.
    #[builder(into)]
    #[serde(rename = "recordSizeKib")]
    pub r#record_size_kib: Option<i32>,
    /// Specify how much storage users or groups can use on the volume. Maximum of 100 items. See `user_and_group_quotas` Block for details.
    #[builder(into)]
    #[serde(rename = "userAndGroupQuotas")]
    pub r#user_and_group_quotas: Option<Vec<super::super::types::fsx::OpenZfsFileSystemRootVolumeConfigurationUserAndGroupQuota>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OpenZfsFileSystemRootVolumeConfiguration {
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
                    "copy_tags_to_snapshots",
                    &self.r#copy_tags_to_snapshots,
                ),
                to_pulumi_object_field(
                    "data_compression_type",
                    &self.r#data_compression_type,
                ),
                to_pulumi_object_field(
                    "nfs_exports",
                    &self.r#nfs_exports,
                ),
                to_pulumi_object_field(
                    "read_only",
                    &self.r#read_only,
                ),
                to_pulumi_object_field(
                    "record_size_kib",
                    &self.r#record_size_kib,
                ),
                to_pulumi_object_field(
                    "user_and_group_quotas",
                    &self.r#user_and_group_quotas,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OpenZfsFileSystemRootVolumeConfiguration {
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
                    r#copy_tags_to_snapshots: {
                        let field_value = match fields_map.get("copy_tags_to_snapshots") {
                            Some(value) => value,
                            None => bail!("Missing field 'copy_tags_to_snapshots' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_compression_type: {
                        let field_value = match fields_map.get("data_compression_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_compression_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nfs_exports: {
                        let field_value = match fields_map.get("nfs_exports") {
                            Some(value) => value,
                            None => bail!("Missing field 'nfs_exports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#read_only: {
                        let field_value = match fields_map.get("read_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_size_kib: {
                        let field_value = match fields_map.get("record_size_kib") {
                            Some(value) => value,
                            None => bail!("Missing field 'record_size_kib' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_and_group_quotas: {
                        let field_value = match fields_map.get("user_and_group_quotas") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_and_group_quotas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
