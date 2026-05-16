#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MetadataAuthor {
    /// The email address of the author contact.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Option<String>,
    /// The link for author/vendor page.
    #[builder(into)]
    #[serde(rename = "link")]
    pub r#link: Option<String>,
    /// The name of the author, company or person.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}
