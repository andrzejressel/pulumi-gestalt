#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SdkvoiceGlobalSettingsVoiceConnector {
    /// The S3 bucket that stores the Voice Connector's call detail records.
    #[builder(into)]
    #[serde(rename = "cdrBucket")]
    pub r#cdr_bucket: Option<String>,
}
