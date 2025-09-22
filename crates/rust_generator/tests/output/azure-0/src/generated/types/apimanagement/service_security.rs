#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
