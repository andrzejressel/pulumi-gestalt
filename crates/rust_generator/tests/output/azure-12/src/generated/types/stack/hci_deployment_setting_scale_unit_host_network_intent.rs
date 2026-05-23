#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HciDeploymentSettingScaleUnitHostNetworkIntent {
    /// A `adapter_property_override` block as defined above. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    pub r#adapter_property_override: Option<Box<super::super::types::stack::HciDeploymentSettingScaleUnitHostNetworkIntentAdapterPropertyOverride>>,
    /// Whether to override adapter properties. Possible values are `true` and `false`. defaults to `false`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    pub r#adapter_property_override_enabled: Option<bool>,
    /// Specifies a list of ID of network interfaces used for the network intent. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    pub r#adapters: Vec<String>,
    /// Specifies the name of the intent. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    pub r#name: String,
    /// A `qos_policy_override` block as defined below. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    pub r#qos_policy_override: Option<Box<super::super::types::stack::HciDeploymentSettingScaleUnitHostNetworkIntentQosPolicyOverride>>,
    /// Whether to override QoS policy. Possible values are `true` and `false`. defaults to `false`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    pub r#qos_policy_override_enabled: Option<bool>,
    /// Specifies a list of network traffic types. Possible values are `Compute`, `Storage`, `Management`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    pub r#traffic_types: Vec<String>,
    /// A `virtual_switch_configuration_override` block as defined below. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    pub r#virtual_switch_configuration_override: Option<Box<super::super::types::stack::HciDeploymentSettingScaleUnitHostNetworkIntentVirtualSwitchConfigurationOverride>>,
    /// Whether to override virtual switch configuration. Possible values are `true` and `false`. defaults to `false`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    pub r#virtual_switch_configuration_override_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HciDeploymentSettingScaleUnitHostNetworkIntent {
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
                    "adapterPropertyOverride",
                    &self.r#adapter_property_override,
                ),
                to_pulumi_object_field(
                    "adapterPropertyOverrideEnabled",
                    &self.r#adapter_property_override_enabled,
                ),
                to_pulumi_object_field(
                    "adapters",
                    &self.r#adapters,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "qosPolicyOverride",
                    &self.r#qos_policy_override,
                ),
                to_pulumi_object_field(
                    "qosPolicyOverrideEnabled",
                    &self.r#qos_policy_override_enabled,
                ),
                to_pulumi_object_field(
                    "trafficTypes",
                    &self.r#traffic_types,
                ),
                to_pulumi_object_field(
                    "virtualSwitchConfigurationOverride",
                    &self.r#virtual_switch_configuration_override,
                ),
                to_pulumi_object_field(
                    "virtualSwitchConfigurationOverrideEnabled",
                    &self.r#virtual_switch_configuration_override_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("adapterPropertyOverride") {
                            Some(value) => value,
                            None => bail!("Missing field 'adapterPropertyOverride' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#adapter_property_override_enabled: {
                        let field_value = match fields_map.get("adapterPropertyOverrideEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'adapterPropertyOverrideEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("qosPolicyOverride") {
                            Some(value) => value,
                            None => bail!("Missing field 'qosPolicyOverride' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#qos_policy_override_enabled: {
                        let field_value = match fields_map.get("qosPolicyOverrideEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'qosPolicyOverrideEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#traffic_types: {
                        let field_value = match fields_map.get("trafficTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'trafficTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_switch_configuration_override: {
                        let field_value = match fields_map.get("virtualSwitchConfigurationOverride") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtualSwitchConfigurationOverride' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_switch_configuration_override_enabled: {
                        let field_value = match fields_map.get("virtualSwitchConfigurationOverrideEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtualSwitchConfigurationOverrideEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
