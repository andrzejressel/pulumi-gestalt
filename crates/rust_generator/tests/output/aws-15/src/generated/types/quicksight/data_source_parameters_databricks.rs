#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceParametersDatabricks {
    /// The host name of the Databricks data source.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: String,
    /// The port for the Databricks data source.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
    /// The HTTP path of the Databricks data source.
    #[builder(into)]
    #[serde(rename = "sqlEndpointPath")]
    pub r#sql_endpoint_path: String,
}
