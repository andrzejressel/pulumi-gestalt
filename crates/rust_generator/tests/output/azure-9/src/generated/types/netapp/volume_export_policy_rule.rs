#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VolumeExportPolicyRule {
    /// A list of allowed clients IPv4 addresses.
    #[builder(into)]
    #[serde(rename = "allowedClients")]
    pub r#allowed_clients: Vec<String>,
    /// Is Kerberos 5 read-only access permitted to this volume?
    #[builder(into)]
    #[serde(rename = "kerberos5ReadOnlyEnabled")]
    pub r#kerberos_5_read_only_enabled: Option<bool>,
    /// Is Kerberos 5 read/write permitted to this volume?
    #[builder(into)]
    #[serde(rename = "kerberos5ReadWriteEnabled")]
    pub r#kerberos_5_read_write_enabled: Option<bool>,
    /// Is Kerberos 5i read-only permitted to this volume?
    #[builder(into)]
    #[serde(rename = "kerberos5iReadOnlyEnabled")]
    pub r#kerberos_5_i_read_only_enabled: Option<bool>,
    /// Is Kerberos 5i read/write permitted to this volume?
    #[builder(into)]
    #[serde(rename = "kerberos5iReadWriteEnabled")]
    pub r#kerberos_5_i_read_write_enabled: Option<bool>,
    /// Is Kerberos 5p read-only permitted to this volume?
    #[builder(into)]
    #[serde(rename = "kerberos5pReadOnlyEnabled")]
    pub r#kerberos_5_p_read_only_enabled: Option<bool>,
    /// Is Kerberos 5p read/write permitted to this volume?
    #[builder(into)]
    #[serde(rename = "kerberos5pReadWriteEnabled")]
    pub r#kerberos_5_p_read_write_enabled: Option<bool>,
    /// A list of allowed protocols. Valid values include `CIFS`, `NFSv3`, or `NFSv4.1`. Only one value is supported at this time. This replaces the previous arguments: `cifs_enabled`, `nfsv3_enabled` and `nfsv4_enabled`.
    #[builder(into)]
    #[serde(rename = "protocolsEnabled")]
    pub r#protocols_enabled: Option<String>,
    /// Is root access permitted to this volume?
    #[builder(into)]
    #[serde(rename = "rootAccessEnabled")]
    pub r#root_access_enabled: Option<bool>,
    /// The index number of the rule.
    #[builder(into)]
    #[serde(rename = "ruleIndex")]
    pub r#rule_index: i32,
    /// Is the file system on unix read only?
    #[builder(into)]
    #[serde(rename = "unixReadOnly")]
    pub r#unix_read_only: Option<bool>,
    /// Is the file system on unix read and write?
    #[builder(into)]
    #[serde(rename = "unixReadWrite")]
    pub r#unix_read_write: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VolumeExportPolicyRule {
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
                "allowed_clients".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_clients,
                )
                .await,
            );
            map.insert(
                "kerberos_5_read_only_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kerberos_5_read_only_enabled,
                )
                .await,
            );
            map.insert(
                "kerberos_5_read_write_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kerberos_5_read_write_enabled,
                )
                .await,
            );
            map.insert(
                "kerberos_5_i_read_only_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kerberos_5_i_read_only_enabled,
                )
                .await,
            );
            map.insert(
                "kerberos_5_i_read_write_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kerberos_5_i_read_write_enabled,
                )
                .await,
            );
            map.insert(
                "kerberos_5_p_read_only_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kerberos_5_p_read_only_enabled,
                )
                .await,
            );
            map.insert(
                "kerberos_5_p_read_write_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kerberos_5_p_read_write_enabled,
                )
                .await,
            );
            map.insert(
                "protocols_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#protocols_enabled,
                )
                .await,
            );
            map.insert(
                "root_access_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#root_access_enabled,
                )
                .await,
            );
            map.insert(
                "rule_index".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rule_index,
                )
                .await,
            );
            map.insert(
                "unix_read_only".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#unix_read_only,
                )
                .await,
            );
            map.insert(
                "unix_read_write".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#unix_read_write,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VolumeExportPolicyRule {
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
                    r#allowed_clients: {
                        let field_value = match fields_map.get("allowed_clients") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_clients' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_read_only_enabled: {
                        let field_value = match fields_map.get("kerberos_5_read_only_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos_5_read_only_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_read_write_enabled: {
                        let field_value = match fields_map.get("kerberos_5_read_write_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos_5_read_write_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_i_read_only_enabled: {
                        let field_value = match fields_map.get("kerberos_5_i_read_only_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos_5_i_read_only_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_i_read_write_enabled: {
                        let field_value = match fields_map.get("kerberos_5_i_read_write_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos_5_i_read_write_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_p_read_only_enabled: {
                        let field_value = match fields_map.get("kerberos_5_p_read_only_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos_5_p_read_only_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_p_read_write_enabled: {
                        let field_value = match fields_map.get("kerberos_5_p_read_write_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos_5_p_read_write_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocols_enabled: {
                        let field_value = match fields_map.get("protocols_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocols_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_access_enabled: {
                        let field_value = match fields_map.get("root_access_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'root_access_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_index: {
                        let field_value = match fields_map.get("rule_index") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_index' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unix_read_only: {
                        let field_value = match fields_map.get("unix_read_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'unix_read_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unix_read_write: {
                        let field_value = match fields_map.get("unix_read_write") {
                            Some(value) => value,
                            None => bail!("Missing field 'unix_read_write' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
