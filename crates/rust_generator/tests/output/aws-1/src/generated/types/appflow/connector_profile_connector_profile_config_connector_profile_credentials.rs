#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentials {
    /// The connector-specific credentials required when using Amplitude. See Amplitude Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "amplitude")]
    pub r#amplitude: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsAmplitude>>,
    /// The connector-specific profile credentials required when using the custom connector. See Custom Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "customConnector")]
    pub r#custom_connector: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnector>>,
    /// Connector-specific credentials required when using Datadog. See Datadog Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "datadog")]
    pub r#datadog: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsDatadog>>,
    /// The connector-specific credentials required when using Dynatrace. See Dynatrace Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "dynatrace")]
    pub r#dynatrace: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsDynatrace>>,
    /// The connector-specific credentials required when using Google Analytics. See Google Analytics Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "googleAnalytics")]
    pub r#google_analytics: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsGoogleAnalytics>>,
    /// The connector-specific credentials required when using Amazon Honeycode. See Honeycode Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "honeycode")]
    pub r#honeycode: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsHoneycode>>,
    /// The connector-specific credentials required when using Infor Nexus. See Infor Nexus Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "inforNexus")]
    pub r#infor_nexus: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsInforNexus>>,
    /// Connector-specific credentials required when using Marketo. See Marketo Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "marketo")]
    pub r#marketo: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsMarketo>>,
    /// Connector-specific credentials required when using Amazon Redshift. See Redshift Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "redshift")]
    pub r#redshift: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsRedshift>>,
    /// The connector-specific credentials required when using Salesforce. See Salesforce Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "salesforce")]
    pub r#salesforce: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforce>>,
    /// The connector-specific credentials required when using SAPOData. See SAPOData Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "sapoData")]
    pub r#sapo_data: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoData>>,
    /// The connector-specific credentials required when using ServiceNow. See ServiceNow Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "serviceNow")]
    pub r#service_now: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsServiceNow>>,
    /// Connector-specific credentials required when using Singular. See Singular Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "singular")]
    pub r#singular: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSingular>>,
    /// Connector-specific credentials required when using Slack. See Slack Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "slack")]
    pub r#slack: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSlack>>,
    /// The connector-specific credentials required when using Snowflake. See Snowflake Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "snowflake")]
    pub r#snowflake: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSnowflake>>,
    /// The connector-specific credentials required when using Trend Micro. See Trend Micro Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "trendmicro")]
    pub r#trendmicro: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsTrendmicro>>,
    /// Connector-specific credentials required when using Veeva. See Veeva Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "veeva")]
    pub r#veeva: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsVeeva>>,
    /// Connector-specific credentials required when using Zendesk. See Zendesk Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "zendesk")]
    pub r#zendesk: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsZendesk>>,
}
