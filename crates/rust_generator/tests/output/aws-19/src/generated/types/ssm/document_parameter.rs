#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DocumentParameter {
    /// If specified, the default values for the parameters. Parameters without a default value are required. Parameters with a default value are optional.
    #[builder(into)]
    #[serde(rename = "defaultValue")]
    pub r#default_value: Option<String>,
    /// A description of what the parameter does, how to use it, the default value, and whether or not the parameter is optional.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The name of the document.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The type of parameter. Valid values: `String`, `StringList`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
