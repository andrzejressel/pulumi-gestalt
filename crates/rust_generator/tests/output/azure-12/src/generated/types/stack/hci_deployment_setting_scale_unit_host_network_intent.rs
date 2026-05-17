#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HciDeploymentSettingScaleUnitHostNetworkIntent {
    /// A `adapter_property_override` block as defined above. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "adapterPropertyOverride")]
    pub r#adapter_property_override: Option<Box<super::super::types::stack::HciDeploymentSettingScaleUnitHostNetworkIntentAdapterPropertyOverride>>,
    /// Whether to override adapter properties. Possible values are `true` and `false`. defaults to `false`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "adapterPropertyOverrideEnabled")]
    pub r#adapter_property_override_enabled: Option<bool>,
    /// Specifies a list of ID of network interfaces used for the network intent. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "adapters")]
    pub r#adapters: Vec<String>,
    /// Specifies the name of the intent. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A `qos_policy_override` block as defined below. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "qosPolicyOverride")]
    pub r#qos_policy_override: Option<Box<super::super::types::stack::HciDeploymentSettingScaleUnitHostNetworkIntentQosPolicyOverride>>,
    /// Whether to override QoS policy. Possible values are `true` and `false`. defaults to `false`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "qosPolicyOverrideEnabled")]
    pub r#qos_policy_override_enabled: Option<bool>,
    /// Specifies a list of network traffic types. Possible values are `Compute`, `Storage`, `Management`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "trafficTypes")]
    pub r#traffic_types: Vec<String>,
    /// A `virtual_switch_configuration_override` block as defined below. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "virtualSwitchConfigurationOverride")]
    pub r#virtual_switch_configuration_override: Option<Box<super::super::types::stack::HciDeploymentSettingScaleUnitHostNetworkIntentVirtualSwitchConfigurationOverride>>,
    /// Whether to override virtual switch configuration. Possible values are `true` and `false`. defaults to `false`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "virtualSwitchConfigurationOverrideEnabled")]
    pub r#virtual_switch_configuration_override_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HciDeploymentSettingScaleUnitHostNetworkIntent {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "adapter_property_override".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#adapter_property_override,
                )
                .await,
            );
            map.insert(
                "adapter_property_override_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#adapter_property_override_enabled,
                )
                .await,
            );
            map.insert(
                "adapters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#adapters,
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
                "qos_policy_override".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#qos_policy_override,
                )
                .await,
            );
            map.insert(
                "qos_policy_override_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#qos_policy_override_enabled,
                )
                .await,
            );
            map.insert(
                "traffic_types".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#traffic_types,
                )
                .await,
            );
            map.insert(
                "virtual_switch_configuration_override".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#virtual_switch_configuration_override,
                )
                .await,
            );
            map.insert(
                "virtual_switch_configuration_override_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#virtual_switch_configuration_override_enabled,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HciDeploymentSettingScaleUnitHostNetworkIntent {
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
                    r#adapter_property_override: {
                        let field_value = match fields_map.get("adapter_property_override") {
                            Some(value) => value,
                            None => bail!("Missing field 'adapter_property_override' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#adapter_property_override_enabled: {
                        let field_value = match fields_map.get("adapter_property_override_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'adapter_property_override_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#adapters: {
                        let field_value = match fields_map.get("adapters") {
                            Some(value) => value,
                            None => bail!("Missing field 'adapters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#qos_policy_override: {
                        let field_value = match fields_map.get("qos_policy_override") {
                            Some(value) => value,
                            None => bail!("Missing field 'qos_policy_override' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#qos_policy_override_enabled: {
                        let field_value = match fields_map.get("qos_policy_override_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'qos_policy_override_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#traffic_types: {
                        let field_value = match fields_map.get("traffic_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'traffic_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_switch_configuration_override: {
                        let field_value = match fields_map.get("virtual_switch_configuration_override") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_switch_configuration_override' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_switch_configuration_override_enabled: {
                        let field_value = match fields_map.get("virtual_switch_configuration_override_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_switch_configuration_override_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
