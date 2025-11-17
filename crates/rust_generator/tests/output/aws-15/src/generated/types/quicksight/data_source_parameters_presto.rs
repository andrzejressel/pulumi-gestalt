#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSourceParametersPresto {
    /// The catalog to which to connect.
    #[builder(into)]
    #[serde(rename = "catalog")]
    pub r#catalog: String,
    /// The host to which to connect.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: String,
    /// The port to which to connect.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
}
