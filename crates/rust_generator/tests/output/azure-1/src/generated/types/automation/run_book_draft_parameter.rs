#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RunBookDraftParameter {
    /// Specifies the default value of the parameter.
    #[builder(into)]
    #[serde(rename = "defaultValue")]
    pub r#default_value: Option<String>,
    /// The name of the parameter.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
    /// Whether this parameter is mandatory.
    #[builder(into)]
    #[serde(rename = "mandatory")]
    pub r#mandatory: Option<bool>,
    /// Specifies the position of the parameter.
    #[builder(into)]
    #[serde(rename = "position")]
    pub r#position: Option<i32>,
    /// Specifies the type of this parameter.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
