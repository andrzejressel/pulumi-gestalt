#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketV2ServerSideEncryptionConfiguration {
    /// Single object for server-side encryption by default configuration. (documented below)
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Box<Vec<super::super::types::s3::BucketV2ServerSideEncryptionConfigurationRule>>,
}
