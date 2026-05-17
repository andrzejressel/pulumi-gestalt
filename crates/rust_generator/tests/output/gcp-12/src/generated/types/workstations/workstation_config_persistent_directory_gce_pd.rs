#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkstationConfigPersistentDirectoryGcePd {
    /// Type of the disk to use. Defaults to `"pd-standard"`.
    #[builder(into)]
    #[serde(rename = "diskType")]
    pub r#disk_type: Option<String>,
    /// Type of file system that the disk should be formatted with. The workstation image must support this file system type. Must be empty if `sourceSnapshot` is set. Defaults to `ext4`.
    #[builder(into)]
    #[serde(rename = "fsType")]
    pub r#fs_type: Option<String>,
    /// Whether the persistent disk should be deleted when the workstation is deleted. Valid values are `DELETE` and `RETAIN`. Defaults to `DELETE`.
    /// Possible values are: `DELETE`, `RETAIN`.
    #[builder(into)]
    #[serde(rename = "reclaimPolicy")]
    pub r#reclaim_policy: Option<String>,
    /// The GB capacity of a persistent home directory for each workstation created with this configuration. Must be empty if `sourceSnapshot` is set.
    /// Valid values are `10`, `50`, `100`, `200`, `500`, or `1000`. Defaults to `200`. If less than `200` GB, the `diskType` must be `pd-balanced` or `pd-ssd`.
    #[builder(into)]
    #[serde(rename = "sizeGb")]
    pub r#size_gb: Option<i32>,
    /// Name of the snapshot to use as the source for the disk.
    /// Must be empty if `sourceImage` is set.
    /// Must be empty if `read_only` is false.
    /// Updating `source_snapshot` will update content in the ephemeral directory after the workstation is restarted.
    #[builder(into)]
    #[serde(rename = "sourceSnapshot")]
    pub r#source_snapshot: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkstationConfigPersistentDirectoryGcePd {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "disk_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disk_type,
                )
                .await,
            );
            map.insert(
                "fs_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fs_type,
                )
                .await,
            );
            map.insert(
                "reclaim_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reclaim_policy,
                )
                .await,
            );
            map.insert(
                "size_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#size_gb,
                )
                .await,
            );
            map.insert(
                "source_snapshot".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_snapshot,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkstationConfigPersistentDirectoryGcePd {
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
                    r#disk_type: {
                        let field_value = match fields_map.get("disk_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fs_type: {
                        let field_value = match fields_map.get("fs_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'fs_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reclaim_policy: {
                        let field_value = match fields_map.get("reclaim_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'reclaim_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#size_gb: {
                        let field_value = match fields_map.get("size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
