#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomHttpsConfigurationCustomHttpsConfiguration {
    /// The name of the Key Vault secret representing the full certificate PFX.
    #[builder(into)]
    pub r#azure_key_vault_certificate_secret_name: Option<String>,
    /// The version of the Key Vault secret representing the full certificate PFX.
    /// 
    /// > **Note:** In order to enable the use of your own custom `HTTPS certificate` you must grant `Azure Front Door Service` access to your key vault. For instructions on how to configure your `Key Vault` correctly please refer to the [product documentation](https://docs.microsoft.com/azure/frontdoor/front-door-custom-domain-https#option-2-use-your-own-certificate).
    #[builder(into)]
    pub r#azure_key_vault_certificate_secret_version: Option<String>,
    /// The ID of the Key Vault containing the SSL certificate.
    #[builder(into)]
    pub r#azure_key_vault_certificate_vault_id: Option<String>,
    /// Certificate source to encrypted `HTTPS` traffic with. Allowed values are `FrontDoor` or `AzureKeyVault`. Defaults to `FrontDoor`.
    /// 
    /// The following attributes are only valid if `certificate_source` is set to `AzureKeyVault`:
    #[builder(into)]
    pub r#certificate_source: Option<String>,
    /// Minimum client TLS version supported.
    #[builder(into)]
    pub r#minimum_tls_version: Option<String>,
    #[builder(into)]
    pub r#provisioning_state: Option<String>,
    #[builder(into)]
    pub r#provisioning_substate: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CustomHttpsConfigurationCustomHttpsConfiguration {
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
                    "azureKeyVaultCertificateSecretName",
                    &self.r#azure_key_vault_certificate_secret_name,
                ),
                to_pulumi_object_field(
                    "azureKeyVaultCertificateSecretVersion",
                    &self.r#azure_key_vault_certificate_secret_version,
                ),
                to_pulumi_object_field(
                    "azureKeyVaultCertificateVaultId",
                    &self.r#azure_key_vault_certificate_vault_id,
                ),
                to_pulumi_object_field(
                    "certificateSource",
                    &self.r#certificate_source,
                ),
                to_pulumi_object_field(
                    "minimumTlsVersion",
                    &self.r#minimum_tls_version,
                ),
                to_pulumi_object_field(
                    "provisioningState",
                    &self.r#provisioning_state,
                ),
                to_pulumi_object_field(
                    "provisioningSubstate",
                    &self.r#provisioning_substate,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("azureKeyVaultCertificateSecretName") {
                            Some(value) => value,
                            None => bail!("Missing field 'azureKeyVaultCertificateSecretName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#azure_key_vault_certificate_secret_version: {
                        let field_value = match fields_map.get("azureKeyVaultCertificateSecretVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'azureKeyVaultCertificateSecretVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#azure_key_vault_certificate_vault_id: {
                        let field_value = match fields_map.get("azureKeyVaultCertificateVaultId") {
                            Some(value) => value,
                            None => bail!("Missing field 'azureKeyVaultCertificateVaultId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#certificate_source: {
                        let field_value = match fields_map.get("certificateSource") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificateSource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimum_tls_version: {
                        let field_value = match fields_map.get("minimumTlsVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimumTlsVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioning_state: {
                        let field_value = match fields_map.get("provisioningState") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioningState' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioning_substate: {
                        let field_value = match fields_map.get("provisioningSubstate") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioningSubstate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
