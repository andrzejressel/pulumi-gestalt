#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ProvisionedProductProvisioningParameter {
    /// Parameter key.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
    /// Whether to ignore `value` and keep the previous parameter value. Ignored when initially provisioning a product.
    #[builder(into)]
    #[serde(rename = "usePreviousValue")]
    pub r#use_previous_value: Option<bool>,
    /// Parameter value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
