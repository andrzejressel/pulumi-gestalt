#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceSecurity {
    /// Should SSL 3.0 be enabled on the backend of the gateway? Defaults to `false`.
    /// 
    /// > **info:** This maps to the `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Backend.Protocols.Ssl30` field
    #[builder(into)]
    #[serde(rename = "enableBackendSsl30")]
    pub r#enable_backend_ssl_30: Option<bool>,
    /// Should TLS 1.0 be enabled on the backend of the gateway? Defaults to `false`.
    /// 
    /// > **info:** This maps to the `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Backend.Protocols.Tls10` field
    #[builder(into)]
    #[serde(rename = "enableBackendTls10")]
    pub r#enable_backend_tls_10: Option<bool>,
    /// Should TLS 1.1 be enabled on the backend of the gateway? Defaults to `false`.
    /// 
    /// > **info:** This maps to the `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Backend.Protocols.Tls11` field
    #[builder(into)]
    #[serde(rename = "enableBackendTls11")]
    pub r#enable_backend_tls_11: Option<bool>,
    /// Should SSL 3.0 be enabled on the frontend of the gateway? Defaults to `false`.
    /// 
    /// > **info:** This maps to the `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Protocols.Ssl30` field
    #[builder(into)]
    #[serde(rename = "enableFrontendSsl30")]
    pub r#enable_frontend_ssl_30: Option<bool>,
    /// Should TLS 1.0 be enabled on the frontend of the gateway? Defaults to `false`.
    /// 
    /// > **info:** This maps to the `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Protocols.Tls10` field
    #[builder(into)]
    #[serde(rename = "enableFrontendTls10")]
    pub r#enable_frontend_tls_10: Option<bool>,
    /// Should TLS 1.1 be enabled on the frontend of the gateway? Defaults to `false`.
    /// 
    /// > **info:** This maps to the `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Protocols.Tls11` field
    #[builder(into)]
    #[serde(rename = "enableFrontendTls11")]
    pub r#enable_frontend_tls_11: Option<bool>,
    /// Should the `TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA` cipher be enabled? Defaults to `false`.
    /// 
    /// > **info:** This maps to the `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Ciphers.TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA` field
    #[builder(into)]
    #[serde(rename = "tlsEcdheEcdsaWithAes128CbcShaCiphersEnabled")]
    pub r#tls_ecdhe_ecdsa_with_aes_128_cbc_sha_ciphers_enabled: Option<bool>,
    /// Should the `TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA` cipher be enabled? Defaults to `false`.
    /// 
    /// > **info:** This maps to the `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Ciphers.TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA` field
    #[builder(into)]
    #[serde(rename = "tlsEcdheEcdsaWithAes256CbcShaCiphersEnabled")]
    pub r#tls_ecdhe_ecdsa_with_aes_256_cbc_sha_ciphers_enabled: Option<bool>,
    /// Should the `TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA` cipher be enabled? Defaults to `false`.
    /// 
    /// > **info:** This maps to the `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Ciphers.TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA` field
    #[builder(into)]
    #[serde(rename = "tlsEcdheRsaWithAes128CbcShaCiphersEnabled")]
    pub r#tls_ecdhe_rsa_with_aes_128_cbc_sha_ciphers_enabled: Option<bool>,
    /// Should the `TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA` cipher be enabled? Defaults to `false`.
    /// 
    /// > **info:** This maps to the `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Ciphers.TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA` field
    #[builder(into)]
    #[serde(rename = "tlsEcdheRsaWithAes256CbcShaCiphersEnabled")]
    pub r#tls_ecdhe_rsa_with_aes_256_cbc_sha_ciphers_enabled: Option<bool>,
    /// Should the `TLS_RSA_WITH_AES_128_CBC_SHA256` cipher be enabled? Defaults to `false`.
    /// 
    /// > **info:** This maps to the `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Ciphers.TLS_RSA_WITH_AES_128_CBC_SHA256` field
    #[builder(into)]
    #[serde(rename = "tlsRsaWithAes128CbcSha256CiphersEnabled")]
    pub r#tls_rsa_with_aes_128_cbc_sha_256_ciphers_enabled: Option<bool>,
    /// Should the `TLS_RSA_WITH_AES_128_CBC_SHA` cipher be enabled? Defaults to `false`.
    /// 
    /// > **info:** This maps to the `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Ciphers.TLS_RSA_WITH_AES_128_CBC_SHA` field
    #[builder(into)]
    #[serde(rename = "tlsRsaWithAes128CbcShaCiphersEnabled")]
    pub r#tls_rsa_with_aes_128_cbc_sha_ciphers_enabled: Option<bool>,
    /// Should the `TLS_RSA_WITH_AES_128_GCM_SHA256` cipher be enabled? Defaults to `false`.
    /// 
    /// > **info:** This maps to the `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Ciphers.TLS_RSA_WITH_AES_128_GCM_SHA256` field
    #[builder(into)]
    #[serde(rename = "tlsRsaWithAes128GcmSha256CiphersEnabled")]
    pub r#tls_rsa_with_aes_128_gcm_sha_256_ciphers_enabled: Option<bool>,
    /// Should the `TLS_RSA_WITH_AES_256_CBC_SHA256` cipher be enabled? Defaults to `false`.
    /// 
    /// > **info:** This maps to the `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Ciphers.TLS_RSA_WITH_AES_256_CBC_SHA256` field
    #[builder(into)]
    #[serde(rename = "tlsRsaWithAes256CbcSha256CiphersEnabled")]
    pub r#tls_rsa_with_aes_256_cbc_sha_256_ciphers_enabled: Option<bool>,
    /// Should the `TLS_RSA_WITH_AES_256_CBC_SHA` cipher be enabled? Defaults to `false`.
    /// 
    /// > **info:** This maps to the `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Ciphers.TLS_RSA_WITH_AES_256_CBC_SHA` field
    #[builder(into)]
    #[serde(rename = "tlsRsaWithAes256CbcShaCiphersEnabled")]
    pub r#tls_rsa_with_aes_256_cbc_sha_ciphers_enabled: Option<bool>,
    /// Should the `TLS_RSA_WITH_AES_256_GCM_SHA384` cipher be enabled? Defaults to `false`.
    /// 
    /// > **info:** This maps to the `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Ciphers.TLS_RSA_WITH_AES_256_GCM_SHA384` field
    #[builder(into)]
    #[serde(rename = "tlsRsaWithAes256GcmSha384CiphersEnabled")]
    pub r#tls_rsa_with_aes_256_gcm_sha_384_ciphers_enabled: Option<bool>,
    /// Should the `TLS_RSA_WITH_3DES_EDE_CBC_SHA` cipher be enabled for alL TLS versions (1.0, 1.1 and 1.2)? 
    /// 
    /// > **info:** This maps to the `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Ciphers.TripleDes168` field
    #[builder(into)]
    #[serde(rename = "tripleDesCiphersEnabled")]
    pub r#triple_des_ciphers_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceSecurity {
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
                    "enable_backend_ssl_30",
                    &self.r#enable_backend_ssl_30,
                ),
                to_pulumi_object_field(
                    "enable_backend_tls_10",
                    &self.r#enable_backend_tls_10,
                ),
                to_pulumi_object_field(
                    "enable_backend_tls_11",
                    &self.r#enable_backend_tls_11,
                ),
                to_pulumi_object_field(
                    "enable_frontend_ssl_30",
                    &self.r#enable_frontend_ssl_30,
                ),
                to_pulumi_object_field(
                    "enable_frontend_tls_10",
                    &self.r#enable_frontend_tls_10,
                ),
                to_pulumi_object_field(
                    "enable_frontend_tls_11",
                    &self.r#enable_frontend_tls_11,
                ),
                to_pulumi_object_field(
                    "tls_ecdhe_ecdsa_with_aes_128_cbc_sha_ciphers_enabled",
                    &self.r#tls_ecdhe_ecdsa_with_aes_128_cbc_sha_ciphers_enabled,
                ),
                to_pulumi_object_field(
                    "tls_ecdhe_ecdsa_with_aes_256_cbc_sha_ciphers_enabled",
                    &self.r#tls_ecdhe_ecdsa_with_aes_256_cbc_sha_ciphers_enabled,
                ),
                to_pulumi_object_field(
                    "tls_ecdhe_rsa_with_aes_128_cbc_sha_ciphers_enabled",
                    &self.r#tls_ecdhe_rsa_with_aes_128_cbc_sha_ciphers_enabled,
                ),
                to_pulumi_object_field(
                    "tls_ecdhe_rsa_with_aes_256_cbc_sha_ciphers_enabled",
                    &self.r#tls_ecdhe_rsa_with_aes_256_cbc_sha_ciphers_enabled,
                ),
                to_pulumi_object_field(
                    "tls_rsa_with_aes_128_cbc_sha_256_ciphers_enabled",
                    &self.r#tls_rsa_with_aes_128_cbc_sha_256_ciphers_enabled,
                ),
                to_pulumi_object_field(
                    "tls_rsa_with_aes_128_cbc_sha_ciphers_enabled",
                    &self.r#tls_rsa_with_aes_128_cbc_sha_ciphers_enabled,
                ),
                to_pulumi_object_field(
                    "tls_rsa_with_aes_128_gcm_sha_256_ciphers_enabled",
                    &self.r#tls_rsa_with_aes_128_gcm_sha_256_ciphers_enabled,
                ),
                to_pulumi_object_field(
                    "tls_rsa_with_aes_256_cbc_sha_256_ciphers_enabled",
                    &self.r#tls_rsa_with_aes_256_cbc_sha_256_ciphers_enabled,
                ),
                to_pulumi_object_field(
                    "tls_rsa_with_aes_256_cbc_sha_ciphers_enabled",
                    &self.r#tls_rsa_with_aes_256_cbc_sha_ciphers_enabled,
                ),
                to_pulumi_object_field(
                    "tls_rsa_with_aes_256_gcm_sha_384_ciphers_enabled",
                    &self.r#tls_rsa_with_aes_256_gcm_sha_384_ciphers_enabled,
                ),
                to_pulumi_object_field(
                    "triple_des_ciphers_enabled",
                    &self.r#triple_des_ciphers_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceSecurity {
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
                    r#enable_backend_ssl_30: {
                        let field_value = match fields_map.get("enable_backend_ssl_30") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_backend_ssl_30' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_backend_tls_10: {
                        let field_value = match fields_map.get("enable_backend_tls_10") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_backend_tls_10' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_backend_tls_11: {
                        let field_value = match fields_map.get("enable_backend_tls_11") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_backend_tls_11' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_frontend_ssl_30: {
                        let field_value = match fields_map.get("enable_frontend_ssl_30") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_frontend_ssl_30' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_frontend_tls_10: {
                        let field_value = match fields_map.get("enable_frontend_tls_10") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_frontend_tls_10' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_frontend_tls_11: {
                        let field_value = match fields_map.get("enable_frontend_tls_11") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_frontend_tls_11' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_ecdhe_ecdsa_with_aes_128_cbc_sha_ciphers_enabled: {
                        let field_value = match fields_map.get("tls_ecdhe_ecdsa_with_aes_128_cbc_sha_ciphers_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_ecdhe_ecdsa_with_aes_128_cbc_sha_ciphers_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_ecdhe_ecdsa_with_aes_256_cbc_sha_ciphers_enabled: {
                        let field_value = match fields_map.get("tls_ecdhe_ecdsa_with_aes_256_cbc_sha_ciphers_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_ecdhe_ecdsa_with_aes_256_cbc_sha_ciphers_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_ecdhe_rsa_with_aes_128_cbc_sha_ciphers_enabled: {
                        let field_value = match fields_map.get("tls_ecdhe_rsa_with_aes_128_cbc_sha_ciphers_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_ecdhe_rsa_with_aes_128_cbc_sha_ciphers_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_ecdhe_rsa_with_aes_256_cbc_sha_ciphers_enabled: {
                        let field_value = match fields_map.get("tls_ecdhe_rsa_with_aes_256_cbc_sha_ciphers_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_ecdhe_rsa_with_aes_256_cbc_sha_ciphers_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_rsa_with_aes_128_cbc_sha_256_ciphers_enabled: {
                        let field_value = match fields_map.get("tls_rsa_with_aes_128_cbc_sha_256_ciphers_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_rsa_with_aes_128_cbc_sha_256_ciphers_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_rsa_with_aes_128_cbc_sha_ciphers_enabled: {
                        let field_value = match fields_map.get("tls_rsa_with_aes_128_cbc_sha_ciphers_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_rsa_with_aes_128_cbc_sha_ciphers_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_rsa_with_aes_128_gcm_sha_256_ciphers_enabled: {
                        let field_value = match fields_map.get("tls_rsa_with_aes_128_gcm_sha_256_ciphers_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_rsa_with_aes_128_gcm_sha_256_ciphers_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_rsa_with_aes_256_cbc_sha_256_ciphers_enabled: {
                        let field_value = match fields_map.get("tls_rsa_with_aes_256_cbc_sha_256_ciphers_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_rsa_with_aes_256_cbc_sha_256_ciphers_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_rsa_with_aes_256_cbc_sha_ciphers_enabled: {
                        let field_value = match fields_map.get("tls_rsa_with_aes_256_cbc_sha_ciphers_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_rsa_with_aes_256_cbc_sha_ciphers_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_rsa_with_aes_256_gcm_sha_384_ciphers_enabled: {
                        let field_value = match fields_map.get("tls_rsa_with_aes_256_gcm_sha_384_ciphers_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_rsa_with_aes_256_gcm_sha_384_ciphers_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#triple_des_ciphers_enabled: {
                        let field_value = match fields_map.get("triple_des_ciphers_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'triple_des_ciphers_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
