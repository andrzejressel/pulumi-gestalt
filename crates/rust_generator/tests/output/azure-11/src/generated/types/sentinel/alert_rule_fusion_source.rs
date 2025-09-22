#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AlertRuleFusionSource {
    /// Whether this source signal is enabled or disabled in Fusion detection? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The name of the Fusion source signal. Refer to Fusion alert rule template for supported values.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// One or more `sub_type` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "subTypes")]
    pub r#sub_types: Option<Vec<super::super::types::sentinel::AlertRuleFusionSourceSubType>>,
}
