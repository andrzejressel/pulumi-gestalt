#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TransferJobTransferSpecAwsS3DataSourceAwsAccessKey {
    /// AWS Key ID.
    #[builder(into)]
    #[serde(rename = "accessKeyId")]
    pub r#access_key_id: String,
    /// AWS Secret Access Key.
    #[builder(into)]
    #[serde(rename = "secretAccessKey")]
    pub r#secret_access_key: String,
}
