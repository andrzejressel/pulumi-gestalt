#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LoggerApplicationInsights {
    /// The connection string of Application Insights.
    #[builder(into)]
    #[serde(rename = "connectionString")]
    pub r#connection_string: Option<String>,
    /// The instrumentation key used to push data to Application Insights.
    /// 
    /// > **Note:** Either `connection_string` or `instrumentation_key` have to be specified.
    #[builder(into)]
    #[serde(rename = "instrumentationKey")]
    pub r#instrumentation_key: Option<String>,
}
