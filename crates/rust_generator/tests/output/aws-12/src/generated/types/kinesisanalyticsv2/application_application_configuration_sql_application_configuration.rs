#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfiguration {
    /// The input stream used by the application.
    #[builder(into, default)]
    #[serde(rename = "input")]
    pub r#input: Box<Option<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInput>>,
    /// The destination streams used by the application.
    #[builder(into, default)]
    #[serde(rename = "outputs")]
    pub r#outputs: Box<Option<Vec<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationOutput>>>,
    /// The reference data source used by the application.
    #[builder(into, default)]
    #[serde(rename = "referenceDataSource")]
    pub r#reference_data_source: Box<Option<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSource>>,
}
