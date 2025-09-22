#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetGatewayCategoriesCategorySubcategory {
    /// True if the subcategory is in beta and subject to change.
    #[builder(into)]
    #[serde(rename = "beta")]
    pub r#beta: bool,
    /// Which account types are allowed to create policies based on this subcategory.
    #[builder(into)]
    #[serde(rename = "class")]
    pub r#class: String,
    /// A short summary of domains in the subcategory.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// The identifier for this subcategory. There is only one subcategory per ID.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: i32,
    /// The name of the subcategory.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
