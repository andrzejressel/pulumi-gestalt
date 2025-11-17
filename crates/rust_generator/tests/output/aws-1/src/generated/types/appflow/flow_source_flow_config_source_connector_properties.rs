#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowSourceFlowConfigSourceConnectorProperties {
    /// Information that is required for querying Amplitude. See Generic Source Properties for more details.
    #[builder(into)]
    #[serde(rename = "amplitude")]
    pub r#amplitude: Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesAmplitude>>,
    /// Properties that are applied when the custom connector is being used as a source. See Custom Connector Source Properties.
    #[builder(into)]
    #[serde(rename = "customConnector")]
    pub r#custom_connector: Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesCustomConnector>>,
    /// Information that is required for querying Datadog. See Generic Source Properties for more details.
    #[builder(into)]
    #[serde(rename = "datadog")]
    pub r#datadog: Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesDatadog>>,
    /// Operation to be performed on the provided Dynatrace source fields. Valid values are `PROJECTION`, `BETWEEN`, `EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
    #[builder(into)]
    #[serde(rename = "dynatrace")]
    pub r#dynatrace: Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesDynatrace>>,
    /// Operation to be performed on the provided Google Analytics source fields. Valid values are `PROJECTION` and `BETWEEN`.
    #[builder(into)]
    #[serde(rename = "googleAnalytics")]
    pub r#google_analytics: Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesGoogleAnalytics>>,
    /// Information that is required for querying Infor Nexus. See Generic Source Properties for more details.
    #[builder(into)]
    #[serde(rename = "inforNexus")]
    pub r#infor_nexus: Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesInforNexus>>,
    /// Information that is required for querying Marketo. See Generic Source Properties for more details.
    #[builder(into)]
    #[serde(rename = "marketo")]
    pub r#marketo: Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesMarketo>>,
    /// Information that is required for querying Amazon S3. See S3 Source Properties for more details.
    #[builder(into)]
    #[serde(rename = "s3")]
    pub r#s_3: Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesS3>>,
    /// Information that is required for querying Salesforce. See Salesforce Source Properties for more details.
    #[builder(into)]
    #[serde(rename = "salesforce")]
    pub r#salesforce: Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesSalesforce>>,
    /// Information that is required for querying SAPOData as a flow source. See SAPO Source Properties for more details.
    #[builder(into)]
    #[serde(rename = "sapoData")]
    pub r#sapo_data: Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesSapoData>>,
    /// Information that is required for querying ServiceNow. See Generic Source Properties for more details.
    #[builder(into)]
    #[serde(rename = "serviceNow")]
    pub r#service_now: Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesServiceNow>>,
    /// Information that is required for querying Singular. See Generic Source Properties for more details.
    #[builder(into)]
    #[serde(rename = "singular")]
    pub r#singular: Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesSingular>>,
    /// Information that is required for querying Slack. See Generic Source Properties for more details.
    #[builder(into)]
    #[serde(rename = "slack")]
    pub r#slack: Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesSlack>>,
    /// Operation to be performed on the provided Trend Micro source fields. Valid values are `PROJECTION`, `EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
    #[builder(into)]
    #[serde(rename = "trendmicro")]
    pub r#trendmicro: Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesTrendmicro>>,
    /// Information that is required for querying Veeva. See Veeva Source Properties for more details.
    #[builder(into)]
    #[serde(rename = "veeva")]
    pub r#veeva: Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesVeeva>>,
    /// Information that is required for querying Zendesk. See Generic Source Properties for more details.
    #[builder(into)]
    #[serde(rename = "zendesk")]
    pub r#zendesk: Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesZendesk>>,
}
