#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobTemplateTemplateVolume {
    /// For Cloud SQL volumes, contains the specific instances that should be mounted. Visit https://cloud.google.com/sql/docs/mysql/connect-run for more information on how to connect Cloud SQL and Cloud Run.
    #[builder(into)]
    #[serde(rename = "cloudSqlInstances")]
    pub r#cloud_sql_instances: Vec<super::super::types::cloudrunv2::GetJobTemplateTemplateVolumeCloudSqlInstance>,
    /// Ephemeral storage used as a shared volume.
    #[builder(into)]
    #[serde(rename = "emptyDirs")]
    pub r#empty_dirs: Vec<super::super::types::cloudrunv2::GetJobTemplateTemplateVolumeEmptyDir>,
    /// Cloud Storage bucket mounted as a volume using GCSFuse.
    #[builder(into)]
    #[serde(rename = "gcs")]
    pub r#gcs: Vec<super::super::types::cloudrunv2::GetJobTemplateTemplateVolumeGc>,
    /// The name of the Cloud Run v2 Job.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// NFS share mounted as a volume.
    #[builder(into)]
    #[serde(rename = "nfs")]
    pub r#nfs: Vec<super::super::types::cloudrunv2::GetJobTemplateTemplateVolumeNf>,
    /// Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
    #[builder(into)]
    #[serde(rename = "secrets")]
    pub r#secrets: Vec<super::super::types::cloudrunv2::GetJobTemplateTemplateVolumeSecret>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetJobTemplateTemplateVolume {
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
                "cloud_sql_instances".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloud_sql_instances,
                )
                .await,
            );
            map.insert(
                "empty_dirs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#empty_dirs,
                )
                .await,
            );
            map.insert(
                "gcs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gcs,
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
                "nfs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#nfs,
                )
                .await,
            );
            map.insert(
                "secrets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secrets,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetJobTemplateTemplateVolume {
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
                    r#cloud_sql_instances: {
                        let field_value = match fields_map.get("cloud_sql_instances") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_sql_instances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#empty_dirs: {
                        let field_value = match fields_map.get("empty_dirs") {
                            Some(value) => value,
                            None => bail!("Missing field 'empty_dirs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcs: {
                        let field_value = match fields_map.get("gcs") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#nfs: {
                        let field_value = match fields_map.get("nfs") {
                            Some(value) => value,
                            None => bail!("Missing field 'nfs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secrets: {
                        let field_value = match fields_map.get("secrets") {
                            Some(value) => value,
                            None => bail!("Missing field 'secrets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
