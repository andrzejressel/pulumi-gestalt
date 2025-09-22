#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ProvisionedProductOutput {
    /// The description of the output.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The output key.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// The output value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
