#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFunctionAppConnectionString {
    /// The name of the Function App resource.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The identity type of the Managed Identity assigned to the Function App.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// The value for the Connection String.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
