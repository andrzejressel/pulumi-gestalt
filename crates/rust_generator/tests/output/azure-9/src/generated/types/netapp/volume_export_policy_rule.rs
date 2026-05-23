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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "allowedClients",
                    &self.r#allowed_clients,
                ),
                to_pulumi_object_field(
                    "kerberos5ReadOnlyEnabled",
                    &self.r#kerberos_5_read_only_enabled,
                ),
                to_pulumi_object_field(
                    "kerberos5ReadWriteEnabled",
                    &self.r#kerberos_5_read_write_enabled,
                ),
                to_pulumi_object_field(
                    "kerberos5iReadOnlyEnabled",
                    &self.r#kerberos_5_i_read_only_enabled,
                ),
                to_pulumi_object_field(
                    "kerberos5iReadWriteEnabled",
                    &self.r#kerberos_5_i_read_write_enabled,
                ),
                to_pulumi_object_field(
                    "kerberos5pReadOnlyEnabled",
                    &self.r#kerberos_5_p_read_only_enabled,
                ),
                to_pulumi_object_field(
                    "kerberos5pReadWriteEnabled",
                    &self.r#kerberos_5_p_read_write_enabled,
                ),
                to_pulumi_object_field(
                    "protocolsEnabled",
                    &self.r#protocols_enabled,
                ),
                to_pulumi_object_field(
                    "rootAccessEnabled",
                    &self.r#root_access_enabled,
                ),
                to_pulumi_object_field(
                    "ruleIndex",
                    &self.r#rule_index,
                ),
                to_pulumi_object_field(
                    "unixReadOnly",
                    &self.r#unix_read_only,
                ),
                to_pulumi_object_field(
                    "unixReadWrite",
                    &self.r#unix_read_write,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("allowedClients") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowedClients' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_read_only_enabled: {
                        let field_value = match fields_map.get("kerberos5ReadOnlyEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos5ReadOnlyEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_read_write_enabled: {
                        let field_value = match fields_map.get("kerberos5ReadWriteEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos5ReadWriteEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_i_read_only_enabled: {
                        let field_value = match fields_map.get("kerberos5iReadOnlyEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos5iReadOnlyEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_i_read_write_enabled: {
                        let field_value = match fields_map.get("kerberos5iReadWriteEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos5iReadWriteEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_p_read_only_enabled: {
                        let field_value = match fields_map.get("kerberos5pReadOnlyEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos5pReadOnlyEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_p_read_write_enabled: {
                        let field_value = match fields_map.get("kerberos5pReadWriteEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos5pReadWriteEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocols_enabled: {
                        let field_value = match fields_map.get("protocolsEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocolsEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_access_enabled: {
                        let field_value = match fields_map.get("rootAccessEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'rootAccessEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_index: {
                        let field_value = match fields_map.get("ruleIndex") {
                            Some(value) => value,
                            None => bail!("Missing field 'ruleIndex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unix_read_only: {
                        let field_value = match fields_map.get("unixReadOnly") {
                            Some(value) => value,
                            None => bail!("Missing field 'unixReadOnly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unix_read_write: {
                        let field_value = match fields_map.get("unixReadWrite") {
                            Some(value) => value,
                            None => bail!("Missing field 'unixReadWrite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
