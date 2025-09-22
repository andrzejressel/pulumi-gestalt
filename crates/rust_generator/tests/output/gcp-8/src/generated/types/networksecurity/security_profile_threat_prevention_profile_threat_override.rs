#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SecurityProfileThreatPreventionProfileThreatOverride {
    /// Threat action.
    /// Possible values are: `ALERT`, `ALLOW`, `DEFAULT_ACTION`, `DENY`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// Vendor-specific ID of a threat to override.
    #[builder(into)]
    #[serde(rename = "threatId")]
    pub r#threat_id: String,
    /// (Output)
    /// Type of threat.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
