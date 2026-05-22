#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProviderFeaturesKeyVault {
    /// When enabled soft-deleted `azure.keyvault.KeyVault` resources will be permanently deleted (e.g purged), when destroyed
    #[builder(into)]
    #[serde(rename = "purgeSoftDeleteOnDestroy")]
    pub r#purge_soft_delete_on_destroy: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.Certificate` resources will be permanently deleted (e.g purged), when destroyed
    #[builder(into)]
    #[serde(rename = "purgeSoftDeletedCertificatesOnDestroy")]
    pub r#purge_soft_deleted_certificates_on_destroy: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.ManagedHardwareSecurityModuleKey` resources will be permanently deleted (e.g purged), when destroyed
    #[builder(into)]
    #[serde(rename = "purgeSoftDeletedHardwareSecurityModuleKeysOnDestroy")]
    pub r#purge_soft_deleted_hardware_security_module_keys_on_destroy: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.ManagedHardwareSecurityModule` resources will be permanently deleted (e.g purged), when destroyed
    #[builder(into)]
    #[serde(rename = "purgeSoftDeletedHardwareSecurityModulesOnDestroy")]
    pub r#purge_soft_deleted_hardware_security_modules_on_destroy: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.Key` resources will be permanently deleted (e.g purged), when destroyed
    #[builder(into)]
    #[serde(rename = "purgeSoftDeletedKeysOnDestroy")]
    pub r#purge_soft_deleted_keys_on_destroy: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.Secret` resources will be permanently deleted (e.g purged), when destroyed
    #[builder(into)]
    #[serde(rename = "purgeSoftDeletedSecretsOnDestroy")]
    pub r#purge_soft_deleted_secrets_on_destroy: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.Certificate` resources will be restored, instead of creating new ones
    #[builder(into)]
    #[serde(rename = "recoverSoftDeletedCertificates")]
    pub r#recover_soft_deleted_certificates: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.ManagedHardwareSecurityModuleKey` resources will be restored, instead of creating new ones
    #[builder(into)]
    #[serde(rename = "recoverSoftDeletedHardwareSecurityModuleKeys")]
    pub r#recover_soft_deleted_hardware_security_module_keys: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.KeyVault` resources will be restored, instead of creating new ones
    #[builder(into)]
    #[serde(rename = "recoverSoftDeletedKeyVaults")]
    pub r#recover_soft_deleted_key_vaults: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.Key` resources will be restored, instead of creating new ones
    #[builder(into)]
    #[serde(rename = "recoverSoftDeletedKeys")]
    pub r#recover_soft_deleted_keys: Option<bool>,
    /// When enabled soft-deleted `azure.keyvault.Secret` resources will be restored, instead of creating new ones
    #[builder(into)]
    #[serde(rename = "recoverSoftDeletedSecrets")]
    pub r#recover_soft_deleted_secrets: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProviderFeaturesKeyVault {
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
                    "purgeSoftDeleteOnDestroy",
                    &self.r#purge_soft_delete_on_destroy,
                ),
                to_pulumi_object_field(
                    "purgeSoftDeletedCertificatesOnDestroy",
                    &self.r#purge_soft_deleted_certificates_on_destroy,
                ),
                to_pulumi_object_field(
                    "purgeSoftDeletedHardwareSecurityModuleKeysOnDestroy",
                    &self.r#purge_soft_deleted_hardware_security_module_keys_on_destroy,
                ),
                to_pulumi_object_field(
                    "purgeSoftDeletedHardwareSecurityModulesOnDestroy",
                    &self.r#purge_soft_deleted_hardware_security_modules_on_destroy,
                ),
                to_pulumi_object_field(
                    "purgeSoftDeletedKeysOnDestroy",
                    &self.r#purge_soft_deleted_keys_on_destroy,
                ),
                to_pulumi_object_field(
                    "purgeSoftDeletedSecretsOnDestroy",
                    &self.r#purge_soft_deleted_secrets_on_destroy,
                ),
                to_pulumi_object_field(
                    "recoverSoftDeletedCertificates",
                    &self.r#recover_soft_deleted_certificates,
                ),
                to_pulumi_object_field(
                    "recoverSoftDeletedHardwareSecurityModuleKeys",
                    &self.r#recover_soft_deleted_hardware_security_module_keys,
                ),
                to_pulumi_object_field(
                    "recoverSoftDeletedKeyVaults",
                    &self.r#recover_soft_deleted_key_vaults,
                ),
                to_pulumi_object_field(
                    "recoverSoftDeletedKeys",
                    &self.r#recover_soft_deleted_keys,
                ),
                to_pulumi_object_field(
                    "recoverSoftDeletedSecrets",
                    &self.r#recover_soft_deleted_secrets,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProviderFeaturesKeyVault {
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
                    r#purge_soft_delete_on_destroy: {
                        let field_value = match fields_map.get("purgeSoftDeleteOnDestroy") {
                            Some(value) => value,
                            None => bail!("Missing field 'purgeSoftDeleteOnDestroy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#purge_soft_deleted_certificates_on_destroy: {
                        let field_value = match fields_map.get("purgeSoftDeletedCertificatesOnDestroy") {
                            Some(value) => value,
                            None => bail!("Missing field 'purgeSoftDeletedCertificatesOnDestroy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#purge_soft_deleted_hardware_security_module_keys_on_destroy: {
                        let field_value = match fields_map.get("purgeSoftDeletedHardwareSecurityModuleKeysOnDestroy") {
                            Some(value) => value,
                            None => bail!("Missing field 'purgeSoftDeletedHardwareSecurityModuleKeysOnDestroy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#purge_soft_deleted_hardware_security_modules_on_destroy: {
                        let field_value = match fields_map.get("purgeSoftDeletedHardwareSecurityModulesOnDestroy") {
                            Some(value) => value,
                            None => bail!("Missing field 'purgeSoftDeletedHardwareSecurityModulesOnDestroy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#purge_soft_deleted_keys_on_destroy: {
                        let field_value = match fields_map.get("purgeSoftDeletedKeysOnDestroy") {
                            Some(value) => value,
                            None => bail!("Missing field 'purgeSoftDeletedKeysOnDestroy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#purge_soft_deleted_secrets_on_destroy: {
                        let field_value = match fields_map.get("purgeSoftDeletedSecretsOnDestroy") {
                            Some(value) => value,
                            None => bail!("Missing field 'purgeSoftDeletedSecretsOnDestroy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recover_soft_deleted_certificates: {
                        let field_value = match fields_map.get("recoverSoftDeletedCertificates") {
                            Some(value) => value,
                            None => bail!("Missing field 'recoverSoftDeletedCertificates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recover_soft_deleted_hardware_security_module_keys: {
                        let field_value = match fields_map.get("recoverSoftDeletedHardwareSecurityModuleKeys") {
                            Some(value) => value,
                            None => bail!("Missing field 'recoverSoftDeletedHardwareSecurityModuleKeys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recover_soft_deleted_key_vaults: {
                        let field_value = match fields_map.get("recoverSoftDeletedKeyVaults") {
                            Some(value) => value,
                            None => bail!("Missing field 'recoverSoftDeletedKeyVaults' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recover_soft_deleted_keys: {
                        let field_value = match fields_map.get("recoverSoftDeletedKeys") {
                            Some(value) => value,
                            None => bail!("Missing field 'recoverSoftDeletedKeys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recover_soft_deleted_secrets: {
                        let field_value = match fields_map.get("recoverSoftDeletedSecrets") {
                            Some(value) => value,
                            None => bail!("Missing field 'recoverSoftDeletedSecrets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
