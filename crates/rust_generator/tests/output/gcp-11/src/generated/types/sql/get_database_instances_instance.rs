#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDatabaseInstancesInstance {
    /// Available Maintenance versions.
    #[builder(into)]
    #[serde(rename = "availableMaintenanceVersions")]
    pub r#available_maintenance_versions: Vec<String>,
    /// Configuration for creating a new instance as a clone of another instance.
    #[builder(into)]
    #[serde(rename = "clones")]
    pub r#clones: Vec<super::super::types::sql::GetDatabaseInstancesInstanceClone>,
    /// The connection name of the instance to be used in connection strings. For example, when connecting with Cloud SQL Proxy.
    #[builder(into)]
    #[serde(rename = "connectionName")]
    pub r#connection_name: String,
    /// To filter out the Cloud SQL instances which are of the specified database version.
    #[builder(into)]
    #[serde(rename = "databaseVersion")]
    pub r#database_version: String,
    #[builder(into)]
    #[serde(rename = "deletionProtection")]
    pub r#deletion_protection: bool,
    /// The dns name of the instance.
    #[builder(into)]
    #[serde(rename = "dnsName")]
    pub r#dns_name: String,
    #[builder(into)]
    #[serde(rename = "encryptionKeyName")]
    pub r#encryption_key_name: String,
    #[builder(into)]
    #[serde(rename = "firstIpAddress")]
    pub r#first_ip_address: String,
    /// The type of the instance. The valid values are:- 'SQL_INSTANCE_TYPE_UNSPECIFIED', 'CLOUD_SQL_INSTANCE', 'ON_PREMISES_INSTANCE' and 'READ_REPLICA_INSTANCE'.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: String,
    #[builder(into)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Vec<super::super::types::sql::GetDatabaseInstancesInstanceIpAddress>,
    /// Maintenance version.
    #[builder(into)]
    #[serde(rename = "maintenanceVersion")]
    pub r#maintenance_version: String,
    /// The name of the instance that will act as the master in the replication setup. Note, this requires the master to have binary_log_enabled set, as well as existing backups.
    #[builder(into)]
    #[serde(rename = "masterInstanceName")]
    pub r#master_instance_name: String,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: String,
    /// The ID of the project in which the resources belong. If it is not provided, the provider project is used.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: String,
    /// The link to service attachment of PSC instance.
    #[builder(into)]
    #[serde(rename = "pscServiceAttachmentLink")]
    pub r#psc_service_attachment_link: String,
    #[builder(into)]
    #[serde(rename = "publicIpAddress")]
    pub r#public_ip_address: String,
    /// To filter out the Cloud SQL instances which are located in the specified region.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: String,
    /// The configuration for replication.
    #[builder(into)]
    #[serde(rename = "replicaConfigurations")]
    pub r#replica_configurations: Vec<super::super::types::sql::GetDatabaseInstancesInstanceReplicaConfiguration>,
    /// The replicas of the instance.
    #[builder(into)]
    #[serde(rename = "replicaNames")]
    pub r#replica_names: Vec<String>,
    #[builder(into)]
    #[serde(rename = "restoreBackupContexts")]
    pub r#restore_backup_contexts: Vec<super::super::types::sql::GetDatabaseInstancesInstanceRestoreBackupContext>,
    /// Initial root password. Required for MS SQL Server.
    #[builder(into)]
    #[serde(rename = "rootPassword")]
    pub r#root_password: String,
    /// The URI of the created resource.
    #[builder(into)]
    #[serde(rename = "selfLink")]
    pub r#self_link: String,
    #[builder(into)]
    #[serde(rename = "serverCaCerts")]
    pub r#server_ca_certs: Vec<super::super::types::sql::GetDatabaseInstancesInstanceServerCaCert>,
    /// The service account email address assigned to the instance.
    #[builder(into)]
    #[serde(rename = "serviceAccountEmailAddress")]
    pub r#service_account_email_address: String,
    /// The settings to use for the database. The configuration is detailed below.
    #[builder(into)]
    #[serde(rename = "settings")]
    pub r#settings: Vec<super::super::types::sql::GetDatabaseInstancesInstanceSetting>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDatabaseInstancesInstance {
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
                    "availableMaintenanceVersions",
                    &self.r#available_maintenance_versions,
                ),
                to_pulumi_object_field(
                    "clones",
                    &self.r#clones,
                ),
                to_pulumi_object_field(
                    "connectionName",
                    &self.r#connection_name,
                ),
                to_pulumi_object_field(
                    "databaseVersion",
                    &self.r#database_version,
                ),
                to_pulumi_object_field(
                    "deletionProtection",
                    &self.r#deletion_protection,
                ),
                to_pulumi_object_field(
                    "dnsName",
                    &self.r#dns_name,
                ),
                to_pulumi_object_field(
                    "encryptionKeyName",
                    &self.r#encryption_key_name,
                ),
                to_pulumi_object_field(
                    "firstIpAddress",
                    &self.r#first_ip_address,
                ),
                to_pulumi_object_field(
                    "instanceType",
                    &self.r#instance_type,
                ),
                to_pulumi_object_field(
                    "ipAddresses",
                    &self.r#ip_addresses,
                ),
                to_pulumi_object_field(
                    "maintenanceVersion",
                    &self.r#maintenance_version,
                ),
                to_pulumi_object_field(
                    "masterInstanceName",
                    &self.r#master_instance_name,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "privateIpAddress",
                    &self.r#private_ip_address,
                ),
                to_pulumi_object_field(
                    "project",
                    &self.r#project,
                ),
                to_pulumi_object_field(
                    "pscServiceAttachmentLink",
                    &self.r#psc_service_attachment_link,
                ),
                to_pulumi_object_field(
                    "publicIpAddress",
                    &self.r#public_ip_address,
                ),
                to_pulumi_object_field(
                    "region",
                    &self.r#region,
                ),
                to_pulumi_object_field(
                    "replicaConfigurations",
                    &self.r#replica_configurations,
                ),
                to_pulumi_object_field(
                    "replicaNames",
                    &self.r#replica_names,
                ),
                to_pulumi_object_field(
                    "restoreBackupContexts",
                    &self.r#restore_backup_contexts,
                ),
                to_pulumi_object_field(
                    "rootPassword",
                    &self.r#root_password,
                ),
                to_pulumi_object_field(
                    "selfLink",
                    &self.r#self_link,
                ),
                to_pulumi_object_field(
                    "serverCaCerts",
                    &self.r#server_ca_certs,
                ),
                to_pulumi_object_field(
                    "serviceAccountEmailAddress",
                    &self.r#service_account_email_address,
                ),
                to_pulumi_object_field(
                    "settings",
                    &self.r#settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDatabaseInstancesInstance {
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
                    r#available_maintenance_versions: {
                        let field_value = match fields_map.get("availableMaintenanceVersions") {
                            Some(value) => value,
                            None => bail!("Missing field 'availableMaintenanceVersions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#clones: {
                        let field_value = match fields_map.get("clones") {
                            Some(value) => value,
                            None => bail!("Missing field 'clones' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connection_name: {
                        let field_value = match fields_map.get("connectionName") {
                            Some(value) => value,
                            None => bail!("Missing field 'connectionName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#deletion_protection: {
                        let field_value = match fields_map.get("deletionProtection") {
                            Some(value) => value,
                            None => bail!("Missing field 'deletionProtection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_name: {
                        let field_value = match fields_map.get("dnsName") {
                            Some(value) => value,
                            None => bail!("Missing field 'dnsName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_key_name: {
                        let field_value = match fields_map.get("encryptionKeyName") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryptionKeyName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#first_ip_address: {
                        let field_value = match fields_map.get("firstIpAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'firstIpAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_type: {
                        let field_value = match fields_map.get("instanceType") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_addresses: {
                        let field_value = match fields_map.get("ipAddresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipAddresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maintenance_version: {
                        let field_value = match fields_map.get("maintenanceVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenanceVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#master_instance_name: {
                        let field_value = match fields_map.get("masterInstanceName") {
                            Some(value) => value,
                            None => bail!("Missing field 'masterInstanceName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#private_ip_address: {
                        let field_value = match fields_map.get("privateIpAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'privateIpAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#project: {
                        let field_value = match fields_map.get("project") {
                            Some(value) => value,
                            None => bail!("Missing field 'project' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#psc_service_attachment_link: {
                        let field_value = match fields_map.get("pscServiceAttachmentLink") {
                            Some(value) => value,
                            None => bail!("Missing field 'pscServiceAttachmentLink' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip_address: {
                        let field_value = match fields_map.get("publicIpAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicIpAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#region: {
                        let field_value = match fields_map.get("region") {
                            Some(value) => value,
                            None => bail!("Missing field 'region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replica_configurations: {
                        let field_value = match fields_map.get("replicaConfigurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'replicaConfigurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replica_names: {
                        let field_value = match fields_map.get("replicaNames") {
                            Some(value) => value,
                            None => bail!("Missing field 'replicaNames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#restore_backup_contexts: {
                        let field_value = match fields_map.get("restoreBackupContexts") {
                            Some(value) => value,
                            None => bail!("Missing field 'restoreBackupContexts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#self_link: {
                        let field_value = match fields_map.get("selfLink") {
                            Some(value) => value,
                            None => bail!("Missing field 'selfLink' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_ca_certs: {
                        let field_value = match fields_map.get("serverCaCerts") {
                            Some(value) => value,
                            None => bail!("Missing field 'serverCaCerts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_email_address: {
                        let field_value = match fields_map.get("serviceAccountEmailAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceAccountEmailAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#settings: {
                        let field_value = match fields_map.get("settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
