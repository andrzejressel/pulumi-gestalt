#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceFileShares {
    /// File share capacity in GiB. This must be at least 1024 GiB
    /// for the standard tier, or 2560 GiB for the premium tier.
    #[builder(into)]
    #[serde(rename = "capacityGb")]
    pub r#capacity_gb: i32,
    /// The name of the fileshare (16 characters or less)
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Nfs Export Options. There is a limit of 10 export options per file share.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "nfsExportOptions")]
    pub r#nfs_export_options: Option<Vec<super::super::types::filestore::InstanceFileSharesNfsExportOption>>,
    /// The resource name of the backup, in the format
    /// projects/{projectId}/locations/{locationId}/backups/{backupId},
    /// that this file share has been restored from.
    #[builder(into)]
    #[serde(rename = "sourceBackup")]
    pub r#source_backup: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceFileShares {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "capacity_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#capacity_gb,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "nfs_export_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#nfs_export_options,
                )
                .await,
            );
            map.insert(
                "source_backup".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_backup,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceFileShares {
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
                    r#capacity_gb: {
                        let field_value = match fields_map.get("capacity_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'capacity_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nfs_export_options: {
                        let field_value = match fields_map.get("nfs_export_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'nfs_export_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_backup: {
                        let field_value = match fields_map.get("source_backup") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_backup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
