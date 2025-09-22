#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AiIndexEndpointDeployedIndexDeployedIndexAuthConfigAuthProvider {
    /// A list of allowed JWT issuers. Each entry must be a valid Google service account, in the following format: service-account-name@project-id.iam.gserviceaccount.com
    #[builder(into)]
    #[serde(rename = "allowedIssuers")]
    pub r#allowed_issuers: Option<Vec<String>>,
    /// The list of JWT audiences. that are allowed to access. A JWT containing any of these audiences will be accepted.
    #[builder(into)]
    #[serde(rename = "audiences")]
    pub r#audiences: Option<Vec<String>>,
}
