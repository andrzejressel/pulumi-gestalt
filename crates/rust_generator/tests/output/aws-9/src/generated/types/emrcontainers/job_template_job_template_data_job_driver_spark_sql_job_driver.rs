#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobTemplateJobTemplateDataJobDriverSparkSqlJobDriver {
    /// The SQL file to be executed.
    #[builder(into)]
    #[serde(rename = "entryPoint")]
    pub r#entry_point: Option<String>,
    /// The Spark parameters to be included in the Spark SQL command.
    #[builder(into)]
    #[serde(rename = "sparkSqlParameters")]
    pub r#spark_sql_parameters: Option<String>,
}
