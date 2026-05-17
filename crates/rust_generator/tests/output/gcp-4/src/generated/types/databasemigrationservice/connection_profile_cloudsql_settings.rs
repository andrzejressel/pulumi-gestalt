#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionProfileCloudsqlSettings {
    /// The activation policy specifies when the instance is activated; it is applicable only when the instance state is 'RUNNABLE'.
    /// Possible values are: `ALWAYS`, `NEVER`.
    #[builder(into)]
    #[serde(rename = "activationPolicy")]
    pub r#activation_policy: Option<String>,
    /// If you enable this setting, Cloud SQL checks your available storage every 30 seconds. If the available storage falls below a threshold size, Cloud SQL automatically adds additional storage capacity.
    /// If the available storage repeatedly falls below the threshold size, Cloud SQL continues to add storage until it reaches the maximum of 30 TB.
    #[builder(into)]
    #[serde(rename = "autoStorageIncrease")]
    pub r#auto_storage_increase: Option<bool>,
    /// The KMS key name used for the csql instance.
    #[builder(into)]
    #[serde(rename = "cmekKeyName")]
    pub r#cmek_key_name: Option<String>,
    /// The Cloud SQL default instance level collation.
    #[builder(into)]
    #[serde(rename = "collation")]
    pub r#collation: Option<String>,
    /// The storage capacity available to the database, in GB. The minimum (and default) size is 10GB.
    #[builder(into)]
    #[serde(rename = "dataDiskSizeGb")]
    pub r#data_disk_size_gb: Option<String>,
    /// The type of storage.
    /// Possible values are: `PD_SSD`, `PD_HDD`.
    #[builder(into)]
    #[serde(rename = "dataDiskType")]
    pub r#data_disk_type: Option<String>,
    /// The database flags passed to the Cloud SQL instance at startup.
    #[builder(into)]
    #[serde(rename = "databaseFlags")]
    pub r#database_flags: Option<std::collections::HashMap<String, String>>,
    /// The database engine type and version.
    /// Currently supported values located at https://cloud.google.com/database-migration/docs/reference/rest/v1/projects.locations.connectionProfiles#sqldatabaseversion
    #[builder(into)]
    #[serde(rename = "databaseVersion")]
    pub r#database_version: Option<String>,
    /// The edition of the given Cloud SQL instance.
    /// Possible values are: `ENTERPRISE`, `ENTERPRISE_PLUS`.
    #[builder(into)]
    #[serde(rename = "edition")]
    pub r#edition: Option<String>,
    /// The settings for IP Management. This allows to enable or disable the instance IP and manage which external networks can connect to the instance. The IPv4 address cannot be disabled.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ipConfig")]
    pub r#ip_config: Option<Box<super::super::types::databasemigrationservice::ConnectionProfileCloudsqlSettingsIpConfig>>,
    /// Input only. Initial root password.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "rootPassword")]
    pub r#root_password: Option<String>,
    /// (Output)
    /// Output only. Indicates If this connection profile root password is stored.
    #[builder(into)]
    #[serde(rename = "rootPasswordSet")]
    pub r#root_password_set: Option<bool>,
    /// The Database Migration Service source connection profile ID, in the format: projects/my_project_name/locations/us-central1/connectionProfiles/connection_profile_ID
    #[builder(into)]
    #[serde(rename = "sourceId")]
    pub r#source_id: String,
    /// The maximum size to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit.
    #[builder(into)]
    #[serde(rename = "storageAutoResizeLimit")]
    pub r#storage_auto_resize_limit: Option<String>,
    /// The tier (or machine type) for this instance, for example: db-n1-standard-1 (MySQL instances) or db-custom-1-3840 (PostgreSQL instances).
    /// For more information, see https://cloud.google.com/sql/docs/mysql/instance-settings
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: Option<String>,
    /// The resource labels for a Cloud SQL instance to use to annotate any related underlying resources such as Compute Engine VMs.
    #[builder(into)]
    #[serde(rename = "userLabels")]
    pub r#user_labels: Option<std::collections::HashMap<String, String>>,
    /// The Google Cloud Platform zone where your Cloud SQL datdabse instance is located.
    #[builder(into)]
    #[serde(rename = "zone")]
    pub r#zone: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionProfileCloudsqlSettings {
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
                "activation_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#activation_policy,
                )
                .await,
            );
            map.insert(
                "auto_storage_increase".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auto_storage_increase,
                )
                .await,
            );
            map.insert(
                "cmek_key_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cmek_key_name,
                )
                .await,
            );
            map.insert(
                "collation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#collation,
                )
                .await,
            );
            map.insert(
                "data_disk_size_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_disk_size_gb,
                )
                .await,
            );
            map.insert(
                "data_disk_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_disk_type,
                )
                .await,
            );
            map.insert(
                "database_flags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#database_flags,
                )
                .await,
            );
            map.insert(
                "database_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#database_version,
                )
                .await,
            );
            map.insert(
                "edition".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#edition,
                )
                .await,
            );
            map.insert(
                "ip_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_config,
                )
                .await,
            );
            map.insert(
                "root_password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#root_password,
                )
                .await,
            );
            map.insert(
                "root_password_set".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#root_password_set,
                )
                .await,
            );
            map.insert(
                "source_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_id,
                )
                .await,
            );
            map.insert(
                "storage_auto_resize_limit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_auto_resize_limit,
                )
                .await,
            );
            map.insert(
                "tier".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tier,
                )
                .await,
            );
            map.insert(
                "user_labels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_labels,
                )
                .await,
            );
            map.insert(
                "zone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#zone,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectionProfileCloudsqlSettings {
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
                    r#activation_policy: {
                        let field_value = match fields_map.get("activation_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'activation_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auto_storage_increase: {
                        let field_value = match fields_map.get("auto_storage_increase") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_storage_increase' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cmek_key_name: {
                        let field_value = match fields_map.get("cmek_key_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'cmek_key_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#collation: {
                        let field_value = match fields_map.get("collation") {
                            Some(value) => value,
                            None => bail!("Missing field 'collation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_disk_size_gb: {
                        let field_value = match fields_map.get("data_disk_size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_disk_size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_disk_type: {
                        let field_value = match fields_map.get("data_disk_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_disk_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_flags: {
                        let field_value = match fields_map.get("database_flags") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_flags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_version: {
                        let field_value = match fields_map.get("database_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#edition: {
                        let field_value = match fields_map.get("edition") {
                            Some(value) => value,
                            None => bail!("Missing field 'edition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_config: {
                        let field_value = match fields_map.get("ip_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_password: {
                        let field_value = match fields_map.get("root_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'root_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_password_set: {
                        let field_value = match fields_map.get("root_password_set") {
                            Some(value) => value,
                            None => bail!("Missing field 'root_password_set' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_id: {
                        let field_value = match fields_map.get("source_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_auto_resize_limit: {
                        let field_value = match fields_map.get("storage_auto_resize_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_auto_resize_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tier: {
                        let field_value = match fields_map.get("tier") {
                            Some(value) => value,
                            None => bail!("Missing field 'tier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_labels: {
                        let field_value = match fields_map.get("user_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zone: {
                        let field_value = match fields_map.get("zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
