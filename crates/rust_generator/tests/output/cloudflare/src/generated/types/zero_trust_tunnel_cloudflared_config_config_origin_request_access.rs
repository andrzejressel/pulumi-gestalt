#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ZeroTrustTunnelCloudflaredConfigConfigOriginRequestAccess {
    /// Audience tags of the access rule.
    #[builder(into)]
    #[serde(rename = "audTags")]
    pub r#aud_tags: Option<Vec<String>>,
    /// Whether the access rule is required.
    #[builder(into)]
    #[serde(rename = "required")]
    pub r#required: Option<bool>,
    /// Name of the team to which the access rule applies.
    #[builder(into)]
    #[serde(rename = "teamName")]
    pub r#team_name: Option<String>,
}
