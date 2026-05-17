#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterStorageProfile {
    /// Is the Blob CSI driver enabled? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "blobDriverEnabled")]
    pub r#blob_driver_enabled: Option<bool>,
    /// Is the Disk CSI driver enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "diskDriverEnabled")]
    pub r#disk_driver_enabled: Option<bool>,
    /// Is the File CSI driver enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "fileDriverEnabled")]
    pub r#file_driver_enabled: Option<bool>,
    /// Is the Snapshot Controller enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "snapshotControllerEnabled")]
    pub r#snapshot_controller_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterStorageProfile {
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
                "blob_driver_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#blob_driver_enabled,
                )
                .await,
            );
            map.insert(
                "disk_driver_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disk_driver_enabled,
                )
                .await,
            );
            map.insert(
                "file_driver_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file_driver_enabled,
                )
                .await,
            );
            map.insert(
                "snapshot_controller_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#snapshot_controller_enabled,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterStorageProfile {
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
                    r#blob_driver_enabled: {
                        let field_value = match fields_map.get("blob_driver_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'blob_driver_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_driver_enabled: {
                        let field_value = match fields_map.get("disk_driver_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_driver_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_driver_enabled: {
                        let field_value = match fields_map.get("file_driver_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_driver_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snapshot_controller_enabled: {
                        let field_value = match fields_map.get("snapshot_controller_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'snapshot_controller_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
