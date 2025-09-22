#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceRelationalDatabaseConfig {
    /// Amazon RDS HTTP endpoint configuration. See `http_endpoint_config` Block for details.
    #[builder(into)]
    #[serde(rename = "httpEndpointConfig")]
    pub r#http_endpoint_config: Box<Option<super::super::types::appsync::DataSourceRelationalDatabaseConfigHttpEndpointConfig>>,
    /// Source type for the relational database. Valid values: `RDS_HTTP_ENDPOINT`.
    #[builder(into)]
    #[serde(rename = "sourceType")]
    pub r#source_type: Option<String>,
}
