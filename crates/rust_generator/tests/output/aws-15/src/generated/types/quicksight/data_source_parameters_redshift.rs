#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceParametersRedshift {
    /// The ID of the cluster to which to connect.
    #[builder(into)]
    #[serde(rename = "clusterId")]
    pub r#cluster_id: Option<String>,
    /// The database to which to connect.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: String,
    /// The host to which to connect.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Option<String>,
    /// The port to which to connect.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
}
