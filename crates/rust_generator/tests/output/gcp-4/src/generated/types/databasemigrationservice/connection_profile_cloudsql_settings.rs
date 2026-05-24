#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionProfileCloudsqlSettings {
    /// The activation policy specifies when the instance is activated; it is applicable only when the instance state is 'RUNNABLE'.
    /// Possible values are: `ALWAYS`, `NEVER`.
    #[builder(into)]
    pub r#activation_policy: Option<String>,
    /// If you enable this setting, Cloud SQL checks your available storage every 30 seconds. If the available storage falls below a threshold size, Cloud SQL automatically adds additional storage capacity.
    /// If the available storage repeatedly falls below the threshold size, Cloud SQL continues to add storage until it reaches the maximum of 30 TB.
    #[builder(into)]
    pub r#auto_storage_increase: Option<bool>,
    /// The KMS key name used for the csql instance.
    #[builder(into)]
    pub r#cmek_key_name: Option<String>,
    /// The Cloud SQL default instance level collation.
    #[builder(into)]
    pub r#collation: Option<String>,
    /// The storage capacity available to the database, in GB. The minimum (and default) size is 10GB.
    #[builder(into)]
    pub r#data_disk_size_gb: Option<String>,
    /// The type of storage.
    /// Possible values are: `PD_SSD`, `PD_HDD`.
    #[builder(into)]
    pub r#data_disk_type: Option<String>,
    /// The database flags passed to the Cloud SQL instance at startup.
    #[builder(into)]
    pub r#database_flags: Option<std::collections::BTreeMap<String, String>>,
    /// The database engine type and version.
    /// Currently supported values located at https://cloud.google.com/database-migration/docs/reference/rest/v1/projects.locations.connectionProfiles#sqldatabaseversion
    #[builder(into)]
    pub r#database_version: Option<String>,
    /// The edition of the given Cloud SQL instance.
    /// Possible values are: `ENTERPRISE`, `ENTERPRISE_PLUS`.
    #[builder(into)]
    pub r#edition: Option<String>,
    /// The settings for IP Management. This allows to enable or disable the instance IP and manage which external networks can connect to the instance. The IPv4 address cannot be disabled.
    /// Structure is documented below.
    #[builder(into)]
    pub r#ip_config: Option<Box<super::super::types::databasemigrationservice::ConnectionProfileCloudsqlSettingsIpConfig>>,
    /// Input only. Initial root password.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    pub r#root_password: Option<String>,
    /// (Output)
    /// Output only. Indicates If this connection profile root password is stored.
    #[builder(into)]
    pub r#root_password_set: Option<bool>,
    /// The Database Migration Service source connection profile ID, in the format: projects/my_project_name/locations/us-central1/connectionProfiles/connection_profile_ID
    #[builder(into)]
    pub r#source_id: String,
    /// The maximum size to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit.
    #[builder(into)]
    pub r#storage_auto_resize_limit: Option<String>,
    /// The tier (or machine type) for this instance, for example: db-n1-standard-1 (MySQL instances) or db-custom-1-3840 (PostgreSQL instances).
    /// For more information, see https://cloud.google.com/sql/docs/mysql/instance-settings
    #[builder(into)]
    pub r#tier: Option<String>,
    /// The resource labels for a Cloud SQL instance to use to annotate any related underlying resources such as Compute Engine VMs.
    #[builder(into)]
    pub r#user_labels: Option<std::collections::BTreeMap<String, String>>,
    /// The Google Cloud Platform zone where your Cloud SQL datdabse instance is located.
    #[builder(into)]
    pub r#zone: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionProfileCloudsqlSettings {
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
                    "activationPolicy",
                    &self.r#activation_policy,
                ),
                to_pulumi_object_field(
                    "autoStorageIncrease",
                    &self.r#auto_storage_increase,
                ),
                to_pulumi_object_field(
                    "cmekKeyName",
                    &self.r#cmek_key_name,
                ),
                to_pulumi_object_field(
                    "collation",
                    &self.r#collation,
                ),
                to_pulumi_object_field(
                    "dataDiskSizeGb",
                    &self.r#data_disk_size_gb,
                ),
                to_pulumi_object_field(
                    "dataDiskType",
                    &self.r#data_disk_type,
                ),
                to_pulumi_object_field(
                    "databaseFlags",
                    &self.r#database_flags,
                ),
                to_pulumi_object_field(
                    "databaseVersion",
                    &self.r#database_version,
                ),
                to_pulumi_object_field(
                    "edition",
                    &self.r#edition,
                ),
                to_pulumi_object_field(
                    "ipConfig",
                    &self.r#ip_config,
                ),
                to_pulumi_object_field(
                    "rootPassword",
                    &self.r#root_password,
                ),
                to_pulumi_object_field(
                    "rootPasswordSet",
                    &self.r#root_password_set,
                ),
                to_pulumi_object_field(
                    "sourceId",
                    &self.r#source_id,
                ),
                to_pulumi_object_field(
                    "storageAutoResizeLimit",
                    &self.r#storage_auto_resize_limit,
                ),
                to_pulumi_object_field(
                    "tier",
                    &self.r#tier,
                ),
                to_pulumi_object_field(
                    "userLabels",
                    &self.r#user_labels,
                ),
                to_pulumi_object_field(
                    "zone",
                    &self.r#zone,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("activationPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'activationPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auto_storage_increase: {
                        let field_value = match fields_map.get("autoStorageIncrease") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoStorageIncrease' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cmek_key_name: {
                        let field_value = match fields_map.get("cmekKeyName") {
                            Some(value) => value,
                            None => bail!("Missing field 'cmekKeyName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("dataDiskSizeGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataDiskSizeGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_disk_type: {
                        let field_value = match fields_map.get("dataDiskType") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataDiskType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_flags: {
                        let field_value = match fields_map.get("databaseFlags") {
                            Some(value) => value,
                            None => bail!("Missing field 'databaseFlags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_version: {
                        let field_value = match fields_map.get("databaseVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'databaseVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("ipConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_password: {
                        let field_value = match fields_map.get("rootPassword") {
                            Some(value) => value,
                            None => bail!("Missing field 'rootPassword' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_password_set: {
                        let field_value = match fields_map.get("rootPasswordSet") {
                            Some(value) => value,
                            None => bail!("Missing field 'rootPasswordSet' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_id: {
                        let field_value = match fields_map.get("sourceId") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_auto_resize_limit: {
                        let field_value = match fields_map.get("storageAutoResizeLimit") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageAutoResizeLimit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("userLabels") {
                            Some(value) => value,
                            None => bail!("Missing field 'userLabels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
