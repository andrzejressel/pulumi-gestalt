#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetGatewayCategoriesCategory {
    /// True if the category is in beta and subject to change.
    #[builder(into)]
    #[serde(rename = "beta")]
    pub r#beta: bool,
    /// Which account types are allowed to create policies based on this category.
    #[builder(into)]
    #[serde(rename = "class")]
    pub r#class: String,
    /// A short summary of domains in the category.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// The identifier for this category. There is only one category per ID.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: i32,
    /// The name of the category.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A list of subcategories.
    #[builder(into)]
    #[serde(rename = "subcategories")]
    pub r#subcategories: Vec<super::types::GetGatewayCategoriesCategorySubcategory>,
}
