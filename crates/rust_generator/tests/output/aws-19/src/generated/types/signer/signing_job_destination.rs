#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SigningJobDestination {
    /// A configuration block describing the S3 Destination object: See S3 Destination below for details.
    #[builder(into)]
    #[serde(rename = "s3")]
    pub r#s_3: Box<super::super::types::signer::SigningJobDestinationS3>,
}
