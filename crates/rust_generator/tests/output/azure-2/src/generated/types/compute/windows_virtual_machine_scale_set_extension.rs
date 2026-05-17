#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsVirtualMachineScaleSetExtension {
    /// Should the latest version of the Extension be used at Deployment Time, if one is available? This won't auto-update the extension on existing installation. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "autoUpgradeMinorVersion")]
    pub r#auto_upgrade_minor_version: Option<bool>,
    /// Should the Extension be automatically updated whenever the Publisher releases a new version of this VM Extension?
    #[builder(into)]
    #[serde(rename = "automaticUpgradeEnabled")]
    pub r#automatic_upgrade_enabled: Option<bool>,
    /// A value which, when different to the previous value can be used to force-run the Extension even if the Extension Configuration hasn't changed.
    #[builder(into)]
    #[serde(rename = "forceUpdateTag")]
    pub r#force_update_tag: Option<String>,
    /// The name for the Virtual Machine Scale Set Extension.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A JSON String which specifies Sensitive Settings (such as Passwords) for the Extension.
    /// 
    /// > **Note:** Keys within the `protected_settings` block are notoriously case-sensitive, where the casing required (e.g. TitleCase vs snakeCase) depends on the Extension being used. Please refer to the documentation for the specific Virtual Machine Extension you're looking to use for more information.
    #[builder(into)]
    #[serde(rename = "protectedSettings")]
    pub r#protected_settings: Option<String>,
    /// A `protected_settings_from_key_vault` block as defined below.
    /// 
    /// > **Note:** `protected_settings_from_key_vault` cannot be used with `protected_settings`
    #[builder(into)]
    #[serde(rename = "protectedSettingsFromKeyVault")]
    pub r#protected_settings_from_key_vault: Option<Box<super::super::types::compute::WindowsVirtualMachineScaleSetExtensionProtectedSettingsFromKeyVault>>,
    /// An ordered list of Extension names which this should be provisioned after.
    #[builder(into)]
    #[serde(rename = "provisionAfterExtensions")]
    pub r#provision_after_extensions: Option<Vec<String>>,
    /// Specifies the Publisher of the Extension.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: String,
    /// A JSON String which specifies Settings for the Extension.
    /// 
    /// > **Note:** Keys within the `settings` block are notoriously case-sensitive, where the casing required (e.g. TitleCase vs snakeCase) depends on the Extension being used. Please refer to the documentation for the specific Virtual Machine Extension you're looking to use for more information.
    #[builder(into)]
    #[serde(rename = "settings")]
    pub r#settings: Option<String>,
    /// Specifies the Type of the Extension.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// Specifies the version of the extension to use, available versions can be found using the Azure CLI.
    #[builder(into)]
    #[serde(rename = "typeHandlerVersion")]
    pub r#type_handler_version: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WindowsVirtualMachineScaleSetExtension {
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
                "auto_upgrade_minor_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auto_upgrade_minor_version,
                )
                .await,
            );
            map.insert(
                "automatic_upgrade_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#automatic_upgrade_enabled,
                )
                .await,
            );
            map.insert(
                "force_update_tag".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#force_update_tag,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "protected_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#protected_settings,
                )
                .await,
            );
            map.insert(
                "protected_settings_from_key_vault".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#protected_settings_from_key_vault,
                )
                .await,
            );
            map.insert(
                "provision_after_extensions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#provision_after_extensions,
                )
                .await,
            );
            map.insert(
                "publisher".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#publisher,
                )
                .await,
            );
            map.insert(
                "settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#settings,
                )
                .await,
            );
            map.insert(
                "type_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_,
                )
                .await,
            );
            map.insert(
                "type_handler_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_handler_version,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WindowsVirtualMachineScaleSetExtension {
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
                    r#auto_upgrade_minor_version: {
                        let field_value = match fields_map.get("auto_upgrade_minor_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_upgrade_minor_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#automatic_upgrade_enabled: {
                        let field_value = match fields_map.get("automatic_upgrade_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'automatic_upgrade_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#force_update_tag: {
                        let field_value = match fields_map.get("force_update_tag") {
                            Some(value) => value,
                            None => bail!("Missing field 'force_update_tag' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protected_settings: {
                        let field_value = match fields_map.get("protected_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'protected_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protected_settings_from_key_vault: {
                        let field_value = match fields_map.get("protected_settings_from_key_vault") {
                            Some(value) => value,
                            None => bail!("Missing field 'protected_settings_from_key_vault' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provision_after_extensions: {
                        let field_value = match fields_map.get("provision_after_extensions") {
                            Some(value) => value,
                            None => bail!("Missing field 'provision_after_extensions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#publisher: {
                        let field_value = match fields_map.get("publisher") {
                            Some(value) => value,
                            None => bail!("Missing field 'publisher' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#settings: {
                        let field_value = match fields_map.get("settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_handler_version: {
                        let field_value = match fields_map.get("type_handler_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_handler_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
