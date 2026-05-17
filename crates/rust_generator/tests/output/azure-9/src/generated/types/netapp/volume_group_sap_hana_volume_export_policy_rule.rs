#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VolumeGroupSapHanaVolumeExportPolicyRule {
    /// A comma-sperated list of allowed client IPv4 addresses.
    #[builder(into)]
    #[serde(rename = "allowedClients")]
    pub r#allowed_clients: String,
    /// Enables NFSv3. Please note that this cannot be enabled if volume has NFSv4.1 as its protocol.
    #[builder(into)]
    #[serde(rename = "nfsv3Enabled")]
    pub r#nfsv_3_enabled: bool,
    /// Enables NFSv4.1. Please note that this cannot be enabled if volume has NFSv3 as its protocol.
    #[builder(into)]
    #[serde(rename = "nfsv41Enabled")]
    pub r#nfsv_41_enabled: bool,
    /// Is root access permitted to this volume? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "rootAccessEnabled")]
    pub r#root_access_enabled: Option<bool>,
    /// The index number of the rule, must start at 1 and maximum 5.
    #[builder(into)]
    #[serde(rename = "ruleIndex")]
    pub r#rule_index: i32,
    /// Is the file system on unix read only? Defaults to `false.
    #[builder(into)]
    #[serde(rename = "unixReadOnly")]
    pub r#unix_read_only: Option<bool>,
    /// Is the file system on unix read and write? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "unixReadWrite")]
    pub r#unix_read_write: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VolumeGroupSapHanaVolumeExportPolicyRule {
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
                "allowed_clients".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_clients,
                )
                .await,
            );
            map.insert(
                "nfsv_3_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#nfsv_3_enabled,
                )
                .await,
            );
            map.insert(
                "nfsv_41_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#nfsv_41_enabled,
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
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VolumeGroupSapHanaVolumeExportPolicyRule {
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
                    r#nfsv_3_enabled: {
                        let field_value = match fields_map.get("nfsv_3_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'nfsv_3_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nfsv_41_enabled: {
                        let field_value = match fields_map.get("nfsv_41_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'nfsv_41_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
