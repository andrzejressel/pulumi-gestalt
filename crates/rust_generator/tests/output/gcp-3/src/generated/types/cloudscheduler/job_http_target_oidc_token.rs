#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobHttpTargetOidcToken {
    /// Audience to be used when generating OIDC token. If not specified,
    /// the URI specified in target will be used.
    #[builder(into)]
    #[serde(rename = "audience")]
    pub r#audience: Option<String>,
    /// Service account email to be used for generating OAuth token.
    /// The service account must be within the same project as the job.
    #[builder(into)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: String,
}
