#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlowDestinationFlowConfigDestinationConnectorProperties {
    /// Properties that are required to query the custom Connector. See Custom Connector Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "customConnector")]
    pub r#custom_connector: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnector>>,
    /// Properties that are required to query Amazon Connect Customer Profiles. See Customer Profiles Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "customerProfiles")]
    pub r#customer_profiles: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesCustomerProfiles>>,
    /// Properties that are required to query Amazon EventBridge. See Generic Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "eventBridge")]
    pub r#event_bridge: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesEventBridge>>,
    /// Properties that are required to query Amazon Honeycode. See Generic Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "honeycode")]
    pub r#honeycode: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesHoneycode>>,
    #[builder(into)]
    #[serde(rename = "lookoutMetrics")]
    pub r#lookout_metrics: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesLookoutMetrics>>,
    /// Properties that are required to query Marketo. See Generic Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "marketo")]
    pub r#marketo: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesMarketo>>,
    /// Properties that are required to query Amazon Redshift. See Redshift Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "redshift")]
    pub r#redshift: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesRedshift>>,
    /// Properties that are required to query Amazon S3. See S3 Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "s3")]
    pub r#s_3: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesS3>>,
    /// Properties that are required to query Salesforce. See Salesforce Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "salesforce")]
    pub r#salesforce: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesSalesforce>>,
    /// Properties that are required to query SAPOData. See SAPOData Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "sapoData")]
    pub r#sapo_data: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesSapoData>>,
    /// Properties that are required to query Snowflake. See Snowflake Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "snowflake")]
    pub r#snowflake: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesSnowflake>>,
    /// Properties that are required to query Upsolver. See Upsolver Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "upsolver")]
    pub r#upsolver: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolver>>,
    /// Properties that are required to query Zendesk. See Zendesk Destination Properties for more details.
    #[builder(into)]
    #[serde(rename = "zendesk")]
    pub r#zendesk: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesZendesk>>,
}
