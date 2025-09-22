#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SigningJobSignedObjectS3 {
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Option<String>,
    /// Key name of the object that contains your unsigned code.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
}
