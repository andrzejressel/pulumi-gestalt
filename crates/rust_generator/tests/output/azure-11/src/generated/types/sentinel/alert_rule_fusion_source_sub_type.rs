#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AlertRuleFusionSourceSubType {
    /// Whether this source subtype under source signal is enabled or disabled in Fusion detection. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The Name of the source subtype under a given source signal in Fusion detection. Refer to Fusion alert rule template for supported values.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A list of severities that are enabled for this source subtype consumed in Fusion detection. Possible values for each element are `High`, `Medium`, `Low`, `Informational`.
    #[builder(into)]
    #[serde(rename = "severitiesAlloweds")]
    pub r#severities_alloweds: Vec<String>,
}
