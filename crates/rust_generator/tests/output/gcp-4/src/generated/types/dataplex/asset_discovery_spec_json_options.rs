#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AssetDiscoverySpecJsonOptions {
    /// Optional. Whether to disable the inference of data type for Json data. If true, all columns will be registered as their primitive types (strings, number or boolean).
    #[builder(into)]
    #[serde(rename = "disableTypeInference")]
    pub r#disable_type_inference: Option<bool>,
    /// Optional. The character encoding of the data. The default is UTF-8.
    #[builder(into)]
    #[serde(rename = "encoding")]
    pub r#encoding: Option<String>,
}
