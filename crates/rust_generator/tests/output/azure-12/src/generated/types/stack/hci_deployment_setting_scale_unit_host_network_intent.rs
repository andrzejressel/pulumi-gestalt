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
