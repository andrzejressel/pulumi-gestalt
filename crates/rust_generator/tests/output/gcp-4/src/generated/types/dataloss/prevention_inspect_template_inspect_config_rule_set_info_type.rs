#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionInspectTemplateInspectConfigRuleSetInfoType {
    /// Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed
    /// at https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Optional custom sensitivity for this InfoType. This only applies to data profiling.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "sensitivityScore")]
    pub r#sensitivity_score: Box<Option<super::super::types::dataloss::PreventionInspectTemplateInspectConfigRuleSetInfoTypeSensitivityScore>>,
    /// Version name for this InfoType.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
