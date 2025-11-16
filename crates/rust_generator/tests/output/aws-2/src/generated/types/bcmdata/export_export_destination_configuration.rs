#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ExportExportDestinationConfiguration {
    /// Object that describes the destination of the data exports file. See the `s3_destination` argument reference below.
    #[builder(into)]
    #[serde(rename = "s3Destinations")]
    pub r#s_3_destinations: Option<Vec<super::super::types::bcmdata::ExportExportDestinationConfigurationS3Destination>>,
}
