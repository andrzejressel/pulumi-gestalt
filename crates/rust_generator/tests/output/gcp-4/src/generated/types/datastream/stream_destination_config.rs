#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamDestinationConfig {
    /// A configuration for how data should be loaded to Google BigQuery.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "bigqueryDestinationConfig")]
    pub r#bigquery_destination_config: Option<Box<super::super::types::datastream::StreamDestinationConfigBigqueryDestinationConfig>>,
    /// Destination connection profile resource. Format: projects/{project}/locations/{location}/connectionProfiles/{name}
    #[builder(into)]
    #[serde(rename = "destinationConnectionProfile")]
    pub r#destination_connection_profile: String,
    /// A configuration for how data should be loaded to Cloud Storage.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "gcsDestinationConfig")]
    pub r#gcs_destination_config: Option<Box<super::super::types::datastream::StreamDestinationConfigGcsDestinationConfig>>,
}
