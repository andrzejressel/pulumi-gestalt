#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OrchestratedVirtualMachineScaleSetExtension {
    /// Should the latest version of the Extension be used at Deployment Time, if one is available? This won't auto-update the extension on existing installation. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "autoUpgradeMinorVersionEnabled")]
    pub r#auto_upgrade_minor_version_enabled: Option<bool>,
    /// An ordered list of Extension names which Virtual Machine Scale Set should provision after VM creation.
    #[builder(into)]
    #[serde(rename = "extensionsToProvisionAfterVmCreations")]
    pub r#extensions_to_provision_after_vm_creations: Option<Vec<String>>,
    /// Should failures from the extension be suppressed? Possible values are `true` or `false`.
    /// 
    /// > **Note:** Operational failures such as not connecting to the VM will not be suppressed regardless of the `failure_suppression_enabled` value.
    #[builder(into)]
    #[serde(rename = "failureSuppressionEnabled")]
    pub r#failure_suppression_enabled: Option<bool>,
    /// A value which, when different to the previous value can be used to force-run the Extension even if the Extension Configuration hasn't changed.
    #[builder(into)]
    #[serde(rename = "forceExtensionExecutionOnChange")]
    pub r#force_extension_execution_on_change: Option<String>,
    /// The name for the Virtual Machine Scale Set Extension.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A JSON String which specifies Sensitive Settings (such as Passwords) for the Extension.
    /// 
    /// > **Note:** Keys within the `protected_settings` block are notoriously case-sensitive, where the casing required (e.g. `TitleCase` vs `snakeCase`) depends on the Extension being used. Please refer to the documentation for the specific Virtual Machine Extension you're looking to use for more information.
    #[builder(into)]
    #[serde(rename = "protectedSettings")]
    pub r#protected_settings: Option<String>,
    /// A `protected_settings_from_key_vault` block as defined below.
    /// 
    /// > **Note:** `protected_settings_from_key_vault` cannot be used with `protected_settings`
    #[builder(into)]
    #[serde(rename = "protectedSettingsFromKeyVault")]
    pub r#protected_settings_from_key_vault: Option<Box<super::super::types::compute::OrchestratedVirtualMachineScaleSetExtensionProtectedSettingsFromKeyVault>>,
    /// Specifies the Publisher of the Extension.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: String,
    /// A JSON String which specifies Settings for the Extension.
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OrchestratedVirtualMachineScaleSetExtension {
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
                    "auto_upgrade_minor_version_enabled",
                    &self.r#auto_upgrade_minor_version_enabled,
                ),
                to_pulumi_object_field(
                    "extensions_to_provision_after_vm_creations",
                    &self.r#extensions_to_provision_after_vm_creations,
                ),
                to_pulumi_object_field(
                    "failure_suppression_enabled",
                    &self.r#failure_suppression_enabled,
                ),
                to_pulumi_object_field(
                    "force_extension_execution_on_change",
                    &self.r#force_extension_execution_on_change,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "protected_settings",
                    &self.r#protected_settings,
                ),
                to_pulumi_object_field(
                    "protected_settings_from_key_vault",
                    &self.r#protected_settings_from_key_vault,
                ),
                to_pulumi_object_field(
                    "publisher",
                    &self.r#publisher,
                ),
                to_pulumi_object_field(
                    "settings",
                    &self.r#settings,
                ),
                to_pulumi_object_field(
                    "type_",
                    &self.r#type_,
                ),
                to_pulumi_object_field(
                    "type_handler_version",
                    &self.r#type_handler_version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OrchestratedVirtualMachineScaleSetExtension {
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
                    r#auto_upgrade_minor_version_enabled: {
                        let field_value = match fields_map.get("auto_upgrade_minor_version_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_upgrade_minor_version_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#extensions_to_provision_after_vm_creations: {
                        let field_value = match fields_map.get("extensions_to_provision_after_vm_creations") {
                            Some(value) => value,
                            None => bail!("Missing field 'extensions_to_provision_after_vm_creations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#failure_suppression_enabled: {
                        let field_value = match fields_map.get("failure_suppression_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'failure_suppression_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#force_extension_execution_on_change: {
                        let field_value = match fields_map.get("force_extension_execution_on_change") {
                            Some(value) => value,
                            None => bail!("Missing field 'force_extension_execution_on_change' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
