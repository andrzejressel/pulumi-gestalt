#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetResourcesSearchAllResult {
    /// Additional searchable attributes of this resource. Informational only. The exact set of attributes is subject to change. For example: project id, DNS name etc.
    #[builder(into)]
    #[serde(rename = "additionalAttributes")]
    pub r#additional_attributes: Vec<String>,
    /// The type of this resource.
    #[builder(into)]
    #[serde(rename = "assetType")]
    pub r#asset_type: String,
    /// One or more paragraphs of text description of this resource. Maximum length could be up to 1M bytes.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// The display name of this resource.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: String,
    /// Labels associated with this resource.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: std::collections::HashMap<String, String>,
    /// Location can be `global`, regional like `us-east1`, or zonal like `us-west1-b`.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
    /// The full resource name. See [Resource Names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Network tags associated with this resource.
    #[builder(into)]
    #[serde(rename = "networkTags")]
    pub r#network_tags: Vec<String>,
    /// The project that this resource belongs to, in the form of `projects/{project_number}`.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: String,
}
