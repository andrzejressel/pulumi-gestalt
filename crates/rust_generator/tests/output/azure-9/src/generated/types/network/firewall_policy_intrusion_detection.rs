#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirewallPolicyIntrusionDetection {
    /// In which mode you want to run intrusion detection: `Off`, `Alert` or `Deny`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    /// A list of Private IP address ranges to identify traffic direction. By default, only ranges defined by IANA RFC 1918 are considered private IP addresses.
    #[builder(into)]
    #[serde(rename = "privateRanges")]
    pub r#private_ranges: Option<Vec<String>>,
    /// One or more `signature_overrides` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "signatureOverrides")]
    pub r#signature_overrides: Option<Vec<super::super::types::network::FirewallPolicyIntrusionDetectionSignatureOverride>>,
    /// One or more `traffic_bypass` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "trafficBypasses")]
    pub r#traffic_bypasses: Option<Vec<super::super::types::network::FirewallPolicyIntrusionDetectionTrafficBypass>>,
}
