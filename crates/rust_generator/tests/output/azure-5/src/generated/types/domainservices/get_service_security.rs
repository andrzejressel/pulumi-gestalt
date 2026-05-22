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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "kerberosArmoringEnabled",
                    &self.r#kerberos_armoring_enabled,
                ),
                to_pulumi_object_field(
                    "kerberosRc4EncryptionEnabled",
                    &self.r#kerberos_rc_4_encryption_enabled,
                ),
                to_pulumi_object_field(
                    "ntlmV1Enabled",
                    &self.r#ntlm_v_1_enabled,
                ),
                to_pulumi_object_field(
                    "syncKerberosPasswords",
                    &self.r#sync_kerberos_passwords,
                ),
                to_pulumi_object_field(
                    "syncNtlmPasswords",
                    &self.r#sync_ntlm_passwords,
                ),
                to_pulumi_object_field(
                    "syncOnPremPasswords",
                    &self.r#sync_on_prem_passwords,
                ),
                to_pulumi_object_field(
                    "tlsV1Enabled",
                    &self.r#tls_v_1_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("kerberosArmoringEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberosArmoringEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_rc_4_encryption_enabled: {
                        let field_value = match fields_map.get("kerberosRc4EncryptionEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberosRc4EncryptionEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ntlm_v_1_enabled: {
                        let field_value = match fields_map.get("ntlmV1Enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'ntlmV1Enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sync_kerberos_passwords: {
                        let field_value = match fields_map.get("syncKerberosPasswords") {
                            Some(value) => value,
                            None => bail!("Missing field 'syncKerberosPasswords' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sync_ntlm_passwords: {
                        let field_value = match fields_map.get("syncNtlmPasswords") {
                            Some(value) => value,
                            None => bail!("Missing field 'syncNtlmPasswords' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sync_on_prem_passwords: {
                        let field_value = match fields_map.get("syncOnPremPasswords") {
                            Some(value) => value,
                            None => bail!("Missing field 'syncOnPremPasswords' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_v_1_enabled: {
                        let field_value = match fields_map.get("tlsV1Enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'tlsV1Enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
