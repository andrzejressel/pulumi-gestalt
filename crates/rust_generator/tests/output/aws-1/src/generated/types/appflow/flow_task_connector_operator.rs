#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlowTaskConnectorOperator {
    /// Operation to be performed on the provided Amplitude source fields. The only valid value is `BETWEEN`.
    #[builder(into)]
    #[serde(rename = "amplitude")]
    pub r#amplitude: Option<String>,
    /// Operators supported by the custom connector. Valid values are `PROJECTION`, `LESS_THAN`, `GREATER_THAN`, `CONTAINS`, `BETWEEN`, `LESS_THAN_OR_EQUAL_TO`, `GREATER_THAN_OR_EQUAL_TO`, `EQUAL_TO`, `NOT_EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
    #[builder(into)]
    #[serde(rename = "customConnector")]
    pub r#custom_connector: Option<String>,
    /// Operation to be performed on the provided Datadog source fields. Valid values are `PROJECTION`, `BETWEEN`, `EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
    #[builder(into)]
    #[serde(rename = "datadog")]
    pub r#datadog: Option<String>,
    /// Operation to be performed on the provided Dynatrace source fields. Valid values are `PROJECTION`, `BETWEEN`, `EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
    #[builder(into)]
    #[serde(rename = "dynatrace")]
    pub r#dynatrace: Option<String>,
    /// Operation to be performed on the provided Google Analytics source fields. Valid values are `PROJECTION` and `BETWEEN`.
    #[builder(into)]
    #[serde(rename = "googleAnalytics")]
    pub r#google_analytics: Option<String>,
    /// Operation to be performed on the provided Infor Nexus source fields. Valid values are `PROJECTION`, `BETWEEN`, `EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
    #[builder(into)]
    #[serde(rename = "inforNexus")]
    pub r#infor_nexus: Option<String>,
    /// Operation to be performed on the provided Marketo source fields. Valid values are `PROJECTION`, `BETWEEN`, `EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
    #[builder(into)]
    #[serde(rename = "marketo")]
    pub r#marketo: Option<String>,
    /// Operation to be performed on the provided Amazon S3 source fields. Valid values are `PROJECTION`, `LESS_THAN`, `GREATER_THAN`, `BETWEEN`, `LESS_THAN_OR_EQUAL_TO`, `GREATER_THAN_OR_EQUAL_TO`, `EQUAL_TO`, `NOT_EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
    #[builder(into)]
    #[serde(rename = "s3")]
    pub r#s_3: Option<String>,
    /// Operation to be performed on the provided Salesforce source fields. Valid values are `PROJECTION`, `LESS_THAN`, `GREATER_THAN`, `CONTAINS`, `BETWEEN`, `LESS_THAN_OR_EQUAL_TO`, `GREATER_THAN_OR_EQUAL_TO`, `EQUAL_TO`, `NOT_EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
    #[builder(into)]
    #[serde(rename = "salesforce")]
    pub r#salesforce: Option<String>,
    /// Operation to be performed on the provided SAPOData source fields. Valid values are `PROJECTION`, `LESS_THAN`, `GREATER_THAN`, `CONTAINS`, `BETWEEN`, `LESS_THAN_OR_EQUAL_TO`, `GREATER_THAN_OR_EQUAL_TO`, `EQUAL_TO`, `NOT_EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
    #[builder(into)]
    #[serde(rename = "sapoData")]
    pub r#sapo_data: Option<String>,
    /// Operation to be performed on the provided ServiceNow source fields. Valid values are `PROJECTION`, `LESS_THAN`, `GREATER_THAN`, `CONTAINS`, `BETWEEN`, `LESS_THAN_OR_EQUAL_TO`, `GREATER_THAN_OR_EQUAL_TO`, `EQUAL_TO`, `NOT_EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
    #[builder(into)]
    #[serde(rename = "serviceNow")]
    pub r#service_now: Option<String>,
    /// Operation to be performed on the provided Singular source fields. Valid values are `PROJECTION`, `EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
    #[builder(into)]
    #[serde(rename = "singular")]
    pub r#singular: Option<String>,
    /// Operation to be performed on the provided Slack source fields. Valid values are `PROJECTION`, `LESS_THAN`, `GREATER_THAN`, `BETWEEN`, `LESS_THAN_OR_EQUAL_TO`, `GREATER_THAN_OR_EQUAL_TO`, `EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
    #[builder(into)]
    #[serde(rename = "slack")]
    pub r#slack: Option<String>,
    /// Operation to be performed on the provided Trend Micro source fields. Valid values are `PROJECTION`, `EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
    #[builder(into)]
    #[serde(rename = "trendmicro")]
    pub r#trendmicro: Option<String>,
    /// Operation to be performed on the provided Veeva source fields. Valid values are `PROJECTION`, `LESS_THAN`, `GREATER_THAN`, `CONTAINS`, `BETWEEN`, `LESS_THAN_OR_EQUAL_TO`, `GREATER_THAN_OR_EQUAL_TO`, `EQUAL_TO`, `NOT_EQUAL_TO`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
    #[builder(into)]
    #[serde(rename = "veeva")]
    pub r#veeva: Option<String>,
    /// Operation to be performed on the provided Zendesk source fields. Valid values are `PROJECTION`, `GREATER_THAN`, `ADDITION`, `MULTIPLICATION`, `DIVISION`, `SUBTRACTION`, `MASK_ALL`, `MASK_FIRST_N`, `MASK_LAST_N`, `VALIDATE_NON_NULL`, `VALIDATE_NON_ZERO`, `VALIDATE_NON_NEGATIVE`, `VALIDATE_NUMERIC`, and `NO_OP`.
    #[builder(into)]
    #[serde(rename = "zendesk")]
    pub r#zendesk: Option<String>,
}
