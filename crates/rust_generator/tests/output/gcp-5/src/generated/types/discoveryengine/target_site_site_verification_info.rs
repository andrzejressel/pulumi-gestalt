#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TargetSiteSiteVerificationInfo {
    /// Site verification state indicating the ownership and validity.
    /// Possible values are: `VERIFIED`, `UNVERIFIED`, `EXEMPTED`.
    #[builder(into, default)]
    #[serde(rename = "siteVerificationState")]
    pub r#site_verification_state: Box<Option<String>>,
    /// Latest site verification time.
    #[builder(into, default)]
    #[serde(rename = "verifyTime")]
    pub r#verify_time: Box<Option<String>>,
}
