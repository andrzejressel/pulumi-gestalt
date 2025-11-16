#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetScriptDagNodeArg {
    /// Name of the argument or property.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Boolean if the value is used as a parameter. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "param")]
    pub r#param: Option<bool>,
    /// Value of the argument or property.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
