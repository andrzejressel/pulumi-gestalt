#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetNetworkServicePccRule {
    /// Specifies the name which should be used for this Mobile Network Service.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A precedence value that is used to decide between data flow policy rules when identifying the QoS values to use for a particular SIM. A lower value means a higher priority.
    #[builder(into)]
    #[serde(rename = "precedence")]
    pub r#precedence: i32,
    /// A `rule_qos_policy` block as defined below. The QoS policy to use for packets matching this rule.
    #[builder(into)]
    #[serde(rename = "qosPolicies")]
    pub r#qos_policies: Vec<super::super::types::mobile::GetNetworkServicePccRuleQosPolicy>,
    /// A `service_data_flow_template` block as defined below. The set of service data flow templates to use for this PCC rule.
    #[builder(into)]
    #[serde(rename = "serviceDataFlowTemplates")]
    pub r#service_data_flow_templates: Vec<super::super::types::mobile::GetNetworkServicePccRuleServiceDataFlowTemplate>,
    /// Determines whether flows that match this data flow policy rule are permitted.
    #[builder(into)]
    #[serde(rename = "trafficControlEnabled")]
    pub r#traffic_control_enabled: bool,
}
