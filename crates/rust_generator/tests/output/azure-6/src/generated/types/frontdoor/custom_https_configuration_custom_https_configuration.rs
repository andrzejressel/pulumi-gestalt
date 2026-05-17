#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CustomHttpsConfigurationCustomHttpsConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "azure_key_vault_certificate_secret_name",
                    &self.r#azure_key_vault_certificate_secret_name,
                ),
                to_pulumi_object_field(
                    "azure_key_vault_certificate_secret_version",
                    &self.r#azure_key_vault_certificate_secret_version,
                ),
                to_pulumi_object_field(
                    "azure_key_vault_certificate_vault_id",
                    &self.r#azure_key_vault_certificate_vault_id,
                ),
                to_pulumi_object_field(
                    "certificate_source",
                    &self.r#certificate_source,
                ),
                to_pulumi_object_field(
                    "minimum_tls_version",
                    &self.r#minimum_tls_version,
                ),
                to_pulumi_object_field(
                    "provisioning_state",
                    &self.r#provisioning_state,
                ),
                to_pulumi_object_field(
                    "provisioning_substate",
                    &self.r#provisioning_substate,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CustomHttpsConfigurationCustomHttpsConfiguration {
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
                    r#azure_key_vault_certificate_secret_name: {
                        let field_value = match fields_map.get("azure_key_vault_certificate_secret_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'azure_key_vault_certificate_secret_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#azure_key_vault_certificate_secret_version: {
                        let field_value = match fields_map.get("azure_key_vault_certificate_secret_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'azure_key_vault_certificate_secret_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#azure_key_vault_certificate_vault_id: {
                        let field_value = match fields_map.get("azure_key_vault_certificate_vault_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'azure_key_vault_certificate_vault_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#certificate_source: {
                        let field_value = match fields_map.get("certificate_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimum_tls_version: {
                        let field_value = match fields_map.get("minimum_tls_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_tls_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioning_state: {
                        let field_value = match fields_map.get("provisioning_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioning_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioning_substate: {
                        let field_value = match fields_map.get("provisioning_substate") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioning_substate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
