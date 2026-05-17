#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTemplateVolume {
    /// For Cloud SQL volumes, contains the specific instances that should be mounted. Visit https://cloud.google.com/sql/docs/mysql/connect-run for more information on how to connect Cloud SQL and Cloud Run.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cloudSqlInstance")]
    pub r#cloud_sql_instance: Option<Box<super::super::types::cloudrunv2::ServiceTemplateVolumeCloudSqlInstance>>,
    /// Ephemeral storage used as a shared volume.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "emptyDir")]
    pub r#empty_dir: Option<Box<super::super::types::cloudrunv2::ServiceTemplateVolumeEmptyDir>>,
    /// Cloud Storage bucket mounted as a volume using GCSFuse. This feature is only supported in the gen2 execution environment.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "gcs")]
    pub r#gcs: Option<Box<super::super::types::cloudrunv2::ServiceTemplateVolumeGcs>>,
    /// Volume's name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Represents an NFS mount.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "nfs")]
    pub r#nfs: Option<Box<super::super::types::cloudrunv2::ServiceTemplateVolumeNfs>>,
    /// Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: Option<Box<super::super::types::cloudrunv2::ServiceTemplateVolumeSecret>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceTemplateVolume {
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
                "cloud_sql_instance".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloud_sql_instance,
                )
                .await,
            );
            map.insert(
                "empty_dir".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#empty_dir,
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
                "secret".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secret,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceTemplateVolume {
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
                    r#cloud_sql_instance: {
                        let field_value = match fields_map.get("cloud_sql_instance") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_sql_instance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#empty_dir: {
                        let field_value = match fields_map.get("empty_dir") {
                            Some(value) => value,
                            None => bail!("Missing field 'empty_dir' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#secret: {
                        let field_value = match fields_map.get("secret") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
