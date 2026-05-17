#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PoolExtension {
    /// Indicates whether the extension should use a newer minor version if one is available at deployment time. Once deployed, however, the extension will not upgrade minor versions unless redeployed, even with this property set to true.
    #[builder(into)]
    #[serde(rename = "autoUpgradeMinorVersion")]
    pub r#auto_upgrade_minor_version: Option<bool>,
    /// Indicates whether the extension should be automatically upgraded by the platform if there is a newer version available. Supported values are `true` and `false`.
    /// 
    /// > **NOTE:** When `automatic_upgrade_enabled` is set to `true`, the `type_handler_version` is automatically updated by the Azure platform when a new version is available and any change in `type_handler_version` should be manually ignored by user.
    #[builder(into)]
    #[serde(rename = "automaticUpgradeEnabled")]
    pub r#automatic_upgrade_enabled: Option<bool>,
    /// The name of the virtual machine extension.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// JSON formatted protected settings for the extension, the value should be encoded with `jsonencode` function. The extension can contain either `protected_settings` or `provision_after_extensions` or no protected settings at all.
    #[builder(into)]
    #[serde(rename = "protectedSettings")]
    pub r#protected_settings: Option<String>,
    /// The collection of extension names. Collection of extension names after which this extension needs to be provisioned.
    #[builder(into)]
    #[serde(rename = "provisionAfterExtensions")]
    pub r#provision_after_extensions: Option<Vec<String>>,
    /// The name of the extension handler publisher.The name of the extension handler publisher.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: String,
    /// JSON formatted public settings for the extension, the value should be encoded with `jsonencode` function.
    #[builder(into)]
    #[serde(rename = "settingsJson")]
    pub r#settings_json: Option<String>,
    /// The type of the extensions.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// The version of script handler.
    #[builder(into)]
    #[serde(rename = "typeHandlerVersion")]
    pub r#type_handler_version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PoolExtension {
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
                "settings_json".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#settings_json,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PoolExtension {
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
                    r#settings_json: {
                        let field_value = match fields_map.get("settings_json") {
                            Some(value) => value,
                            None => bail!("Missing field 'settings_json' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
