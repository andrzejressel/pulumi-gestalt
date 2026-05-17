#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VolumeExportPolicyRule {
    /// Defines the access type for clients matching the `allowedClients` specification.
    /// Possible values are: `READ_ONLY`, `READ_WRITE`, `READ_NONE`.
    #[builder(into)]
    #[serde(rename = "accessType")]
    pub r#access_type: Option<String>,
    /// Defines the client ingress specification (allowed clients) as a comma separated list with IPv4 CIDRs or IPv4 host addresses.
    #[builder(into)]
    #[serde(rename = "allowedClients")]
    pub r#allowed_clients: Option<String>,
    /// If enabled, the root user (UID = 0) of the specified clients doesn't get mapped to nobody (UID = 65534). This is also known as no_root_squash.
    #[builder(into)]
    #[serde(rename = "hasRootAccess")]
    pub r#has_root_access: Option<String>,
    /// If enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'authentication' kerberos security mode.
    #[builder(into)]
    #[serde(rename = "kerberos5ReadOnly")]
    pub r#kerberos_5_read_only: Option<bool>,
    /// If enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'authentication' kerberos security mode. The 'kerberos5ReadOnly' value is ignored if this is enabled.
    #[builder(into)]
    #[serde(rename = "kerberos5ReadWrite")]
    pub r#kerberos_5_read_write: Option<bool>,
    /// If enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'integrity' kerberos security mode.
    #[builder(into)]
    #[serde(rename = "kerberos5iReadOnly")]
    pub r#kerberos_5_i_read_only: Option<bool>,
    /// If enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'integrity' kerberos security mode. The 'kerberos5iReadOnly' value is ignored if this is enabled.
    #[builder(into)]
    #[serde(rename = "kerberos5iReadWrite")]
    pub r#kerberos_5_i_read_write: Option<bool>,
    /// If enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'privacy' kerberos security mode.
    #[builder(into)]
    #[serde(rename = "kerberos5pReadOnly")]
    pub r#kerberos_5_p_read_only: Option<bool>,
    /// If enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'privacy' kerberos security mode. The 'kerberos5pReadOnly' value is ignored if this is enabled.
    #[builder(into)]
    #[serde(rename = "kerberos5pReadWrite")]
    pub r#kerberos_5_p_read_write: Option<bool>,
    /// Enable to apply the export rule to NFSV3 clients.
    #[builder(into)]
    #[serde(rename = "nfsv3")]
    pub r#nfsv_3: Option<bool>,
    /// Enable to apply the export rule to NFSV4.1 clients.
    #[builder(into)]
    #[serde(rename = "nfsv4")]
    pub r#nfsv_4: Option<bool>,
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
                "access_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#access_type,
                )
                .await,
            );
            map.insert(
                "allowed_clients".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_clients,
                )
                .await,
            );
            map.insert(
                "has_root_access".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#has_root_access,
                )
                .await,
            );
            map.insert(
                "kerberos_5_read_only".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kerberos_5_read_only,
                )
                .await,
            );
            map.insert(
                "kerberos_5_read_write".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kerberos_5_read_write,
                )
                .await,
            );
            map.insert(
                "kerberos_5_i_read_only".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kerberos_5_i_read_only,
                )
                .await,
            );
            map.insert(
                "kerberos_5_i_read_write".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kerberos_5_i_read_write,
                )
                .await,
            );
            map.insert(
                "kerberos_5_p_read_only".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kerberos_5_p_read_only,
                )
                .await,
            );
            map.insert(
                "kerberos_5_p_read_write".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kerberos_5_p_read_write,
                )
                .await,
            );
            map.insert(
                "nfsv_3".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#nfsv_3,
                )
                .await,
            );
            map.insert(
                "nfsv_4".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#nfsv_4,
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
                    r#access_type: {
                        let field_value = match fields_map.get("access_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_clients: {
                        let field_value = match fields_map.get("allowed_clients") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_clients' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#has_root_access: {
                        let field_value = match fields_map.get("has_root_access") {
                            Some(value) => value,
                            None => bail!("Missing field 'has_root_access' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_read_only: {
                        let field_value = match fields_map.get("kerberos_5_read_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos_5_read_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_read_write: {
                        let field_value = match fields_map.get("kerberos_5_read_write") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos_5_read_write' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_i_read_only: {
                        let field_value = match fields_map.get("kerberos_5_i_read_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos_5_i_read_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_i_read_write: {
                        let field_value = match fields_map.get("kerberos_5_i_read_write") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos_5_i_read_write' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_p_read_only: {
                        let field_value = match fields_map.get("kerberos_5_p_read_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos_5_p_read_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_5_p_read_write: {
                        let field_value = match fields_map.get("kerberos_5_p_read_write") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos_5_p_read_write' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nfsv_3: {
                        let field_value = match fields_map.get("nfsv_3") {
                            Some(value) => value,
                            None => bail!("Missing field 'nfsv_3' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nfsv_4: {
                        let field_value = match fields_map.get("nfsv_4") {
                            Some(value) => value,
                            None => bail!("Missing field 'nfsv_4' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
