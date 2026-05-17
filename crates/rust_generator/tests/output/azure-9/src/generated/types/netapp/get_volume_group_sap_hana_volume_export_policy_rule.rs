#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVolumeGroupSapHanaVolumeExportPolicyRule {
    /// A list of allowed clients IPv4 addresses.
    #[builder(into)]
    #[serde(rename = "allowedClients")]
    pub r#allowed_clients: String,
    /// Is the NFSv3 protocol enabled?
    #[builder(into)]
    #[serde(rename = "nfsv3Enabled")]
    pub r#nfsv_3_enabled: bool,
    /// Is the NFSv4.1 enabled?
    #[builder(into)]
    #[serde(rename = "nfsv41Enabled")]
    pub r#nfsv_41_enabled: bool,
    /// Is root access permitted to this volume?
    #[builder(into)]
    #[serde(rename = "rootAccessEnabled")]
    pub r#root_access_enabled: bool,
    /// The index number of the rule.
    #[builder(into)]
    #[serde(rename = "ruleIndex")]
    pub r#rule_index: i32,
    /// Is the file system on unix read only?.
    #[builder(into)]
    #[serde(rename = "unixReadOnly")]
    pub r#unix_read_only: bool,
    /// Is the file system on unix read and write?.
    #[builder(into)]
    #[serde(rename = "unixReadWrite")]
    pub r#unix_read_write: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVolumeGroupSapHanaVolumeExportPolicyRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "allowed_clients",
                    &self.r#allowed_clients,
                ),
                to_pulumi_object_field(
                    "nfsv_3_enabled",
                    &self.r#nfsv_3_enabled,
                ),
                to_pulumi_object_field(
                    "nfsv_41_enabled",
                    &self.r#nfsv_41_enabled,
                ),
                to_pulumi_object_field(
                    "root_access_enabled",
                    &self.r#root_access_enabled,
                ),
                to_pulumi_object_field(
                    "rule_index",
                    &self.r#rule_index,
                ),
                to_pulumi_object_field(
                    "unix_read_only",
                    &self.r#unix_read_only,
                ),
                to_pulumi_object_field(
                    "unix_read_write",
                    &self.r#unix_read_write,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVolumeGroupSapHanaVolumeExportPolicyRule {
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
