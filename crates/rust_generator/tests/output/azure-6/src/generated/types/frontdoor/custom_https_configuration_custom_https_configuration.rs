#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CustomHttpsConfigurationCustomHttpsConfiguration {
    /// The name of the Key Vault secret representing the full certificate PFX.
    #[builder(into)]
    #[serde(rename = "azureKeyVaultCertificateSecretName")]
    pub r#azure_key_vault_certificate_secret_name: Option<String>,
    /// The version of the Key Vault secret representing the full certificate PFX.
    /// 
    /// > **Note:** In order to enable the use of your own custom `HTTPS certificate` you must grant `Azure Front Door Service` access to your key vault. For instructions on how to configure your `Key Vault` correctly please refer to the [product documentation](https://docs.microsoft.com/azure/frontdoor/front-door-custom-domain-https#option-2-use-your-own-certificate).
    #[builder(into)]
    #[serde(rename = "azureKeyVaultCertificateSecretVersion")]
    pub r#azure_key_vault_certificate_secret_version: Option<String>,
    /// The ID of the Key Vault containing the SSL certificate.
    #[builder(into)]
    #[serde(rename = "azureKeyVaultCertificateVaultId")]
    pub r#azure_key_vault_certificate_vault_id: Option<String>,
    /// Certificate source to encrypted `HTTPS` traffic with. Allowed values are `FrontDoor` or `AzureKeyVault`. Defaults to `FrontDoor`.
    /// 
    /// The following attributes are only valid if `certificate_source` is set to `AzureKeyVault`:
    #[builder(into)]
    #[serde(rename = "certificateSource")]
    pub r#certificate_source: Option<String>,
    /// Minimum client TLS version supported.
    #[builder(into)]
    #[serde(rename = "minimumTlsVersion")]
    pub r#minimum_tls_version: Option<String>,
    #[builder(into)]
    #[serde(rename = "provisioningState")]
    pub r#provisioning_state: Option<String>,
    #[builder(into)]
    #[serde(rename = "provisioningSubstate")]
    pub r#provisioning_substate: Option<String>,
}
