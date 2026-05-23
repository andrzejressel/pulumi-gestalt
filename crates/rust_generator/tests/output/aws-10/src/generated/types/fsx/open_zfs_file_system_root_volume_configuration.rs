#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OpenZfsFileSystemRootVolumeConfiguration {
    /// A boolean flag indicating whether tags for the file system should be copied to snapshots. The default value is false.
    #[builder(into)]
    pub r#copy_tags_to_snapshots: Option<bool>,
    /// Method used to compress the data on the volume. Valid values are `LZ4`, `NONE` or `ZSTD`. Child volumes that don't specify compression option will inherit from parent volume. This option on file system applies to the root volume.
    #[builder(into)]
    pub r#data_compression_type: Option<String>,
    /// NFS export configuration for the root volume. Exactly 1 item. See `nfs_exports` Block for details.
    #[builder(into)]
    pub r#nfs_exports: Option<Box<super::super::types::fsx::OpenZfsFileSystemRootVolumeConfigurationNfsExports>>,
    /// specifies whether the volume is read-only. Default is false.
    #[builder(into)]
    pub r#read_only: Option<bool>,
    /// Specifies the record size of an OpenZFS root volume, in kibibytes (KiB). Valid values are `4`, `8`, `16`, `32`, `64`, `128`, `256`, `512`, or `1024` KiB. The default is `128` KiB.
    #[builder(into)]
    pub r#record_size_kib: Option<i32>,
    /// Specify how much storage users or groups can use on the volume. Maximum of 100 items. See `user_and_group_quotas` Block for details.
    #[builder(into)]
    pub r#user_and_group_quotas: Option<Vec<super::super::types::fsx::OpenZfsFileSystemRootVolumeConfigurationUserAndGroupQuota>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OpenZfsFileSystemRootVolumeConfiguration {
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
                    "copyTagsToSnapshots",
                    &self.r#copy_tags_to_snapshots,
                ),
                to_pulumi_object_field(
                    "dataCompressionType",
                    &self.r#data_compression_type,
                ),
                to_pulumi_object_field(
                    "nfsExports",
                    &self.r#nfs_exports,
                ),
                to_pulumi_object_field(
                    "readOnly",
                    &self.r#read_only,
                ),
                to_pulumi_object_field(
                    "recordSizeKib",
                    &self.r#record_size_kib,
                ),
                to_pulumi_object_field(
                    "userAndGroupQuotas",
                    &self.r#user_and_group_quotas,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("copyTagsToSnapshots") {
                            Some(value) => value,
                            None => bail!("Missing field 'copyTagsToSnapshots' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_compression_type: {
                        let field_value = match fields_map.get("dataCompressionType") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataCompressionType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nfs_exports: {
                        let field_value = match fields_map.get("nfsExports") {
                            Some(value) => value,
                            None => bail!("Missing field 'nfsExports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#read_only: {
                        let field_value = match fields_map.get("readOnly") {
                            Some(value) => value,
                            None => bail!("Missing field 'readOnly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_size_kib: {
                        let field_value = match fields_map.get("recordSizeKib") {
                            Some(value) => value,
                            None => bail!("Missing field 'recordSizeKib' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_and_group_quotas: {
                        let field_value = match fields_map.get("userAndGroupQuotas") {
                            Some(value) => value,
                            None => bail!("Missing field 'userAndGroupQuotas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
