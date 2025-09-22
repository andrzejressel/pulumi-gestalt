#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SigningProfileSigningMaterial {
    /// The Amazon Resource Name (ARN) of the certificates that is used to sign your code.
    #[builder(into)]
    #[serde(rename = "certificateArn")]
    pub r#certificate_arn: String,
}
