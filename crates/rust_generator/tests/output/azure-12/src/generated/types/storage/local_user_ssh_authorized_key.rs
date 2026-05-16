#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LocalUserSshAuthorizedKey {
    /// The description of this SSH authorized key.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The public key value of this SSH authorized key.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
}
