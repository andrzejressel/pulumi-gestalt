#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CustomActionTypeConfigurationProperty {
    /// The description of the action configuration property.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Whether the configuration property is a key.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: bool,
    /// The name of the action configuration property.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Indicates that the property will be used in conjunction with PollForJobs.
    #[builder(into)]
    #[serde(rename = "queryable")]
    pub r#queryable: Option<bool>,
    /// Whether the configuration property is a required value.
    #[builder(into)]
    #[serde(rename = "required")]
    pub r#required: bool,
    /// Whether the configuration property is secret.
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: bool,
    /// The type of the configuration property. Valid values: `String`, `Number`, `Boolean`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
