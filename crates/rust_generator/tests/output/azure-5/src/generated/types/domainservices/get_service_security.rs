#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceSecurity {
    /// (Optional) Whether the Kerberos Armoring is enabled.
    #[builder(into)]
    #[serde(rename = "kerberosArmoringEnabled")]
    pub r#kerberos_armoring_enabled: bool,
    /// (Optional) Whether the Kerberos RC4 Encryption is enabled.
    #[builder(into)]
    #[serde(rename = "kerberosRc4EncryptionEnabled")]
    pub r#kerberos_rc_4_encryption_enabled: bool,
    /// Whether legacy NTLM v1 support is enabled.
    #[builder(into)]
    #[serde(rename = "ntlmV1Enabled")]
    pub r#ntlm_v_1_enabled: bool,
    /// Whether Kerberos password hashes are synchronized to the managed domain.
    #[builder(into)]
    #[serde(rename = "syncKerberosPasswords")]
    pub r#sync_kerberos_passwords: bool,
    /// Whether NTLM password hashes are synchronized to the managed domain.
    #[builder(into)]
    #[serde(rename = "syncNtlmPasswords")]
    pub r#sync_ntlm_passwords: bool,
    /// Whether on-premises password hashes are synchronized to the managed domain.
    #[builder(into)]
    #[serde(rename = "syncOnPremPasswords")]
    pub r#sync_on_prem_passwords: bool,
    /// Whether legacy TLS v1 support is enabled.
    #[builder(into)]
    #[serde(rename = "tlsV1Enabled")]
    pub r#tls_v_1_enabled: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetServiceSecurity {
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
                "kerberos_armoring_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kerberos_armoring_enabled,
                )
                .await,
            );
            map.insert(
                "kerberos_rc_4_encryption_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kerberos_rc_4_encryption_enabled,
                )
                .await,
            );
            map.insert(
                "ntlm_v_1_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ntlm_v_1_enabled,
                )
                .await,
            );
            map.insert(
                "sync_kerberos_passwords".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sync_kerberos_passwords,
                )
                .await,
            );
            map.insert(
                "sync_ntlm_passwords".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sync_ntlm_passwords,
                )
                .await,
            );
            map.insert(
                "sync_on_prem_passwords".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sync_on_prem_passwords,
                )
                .await,
            );
            map.insert(
                "tls_v_1_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tls_v_1_enabled,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetServiceSecurity {
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
                    r#kerberos_armoring_enabled: {
                        let field_value = match fields_map.get("kerberos_armoring_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos_armoring_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_rc_4_encryption_enabled: {
                        let field_value = match fields_map.get("kerberos_rc_4_encryption_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos_rc_4_encryption_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ntlm_v_1_enabled: {
                        let field_value = match fields_map.get("ntlm_v_1_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'ntlm_v_1_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sync_kerberos_passwords: {
                        let field_value = match fields_map.get("sync_kerberos_passwords") {
                            Some(value) => value,
                            None => bail!("Missing field 'sync_kerberos_passwords' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sync_ntlm_passwords: {
                        let field_value = match fields_map.get("sync_ntlm_passwords") {
                            Some(value) => value,
                            None => bail!("Missing field 'sync_ntlm_passwords' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sync_on_prem_passwords: {
                        let field_value = match fields_map.get("sync_on_prem_passwords") {
                            Some(value) => value,
                            None => bail!("Missing field 'sync_on_prem_passwords' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_v_1_enabled: {
                        let field_value = match fields_map.get("tls_v_1_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_v_1_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
