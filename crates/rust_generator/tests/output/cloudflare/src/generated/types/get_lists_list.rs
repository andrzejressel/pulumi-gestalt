#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetListsList {
    /// List description.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// List identifier.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// List kind.
    #[builder(into)]
    #[serde(rename = "kind")]
    pub r#kind: Option<String>,
    /// The list name to target for the resource.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Number of items in list.
    #[builder(into)]
    #[serde(rename = "numitems")]
    pub r#numitems: Option<i32>,
}
