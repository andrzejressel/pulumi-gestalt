#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationGatewaySslProfileSslPolicy {
    #[builder(into)]
    #[serde(rename = "cipherSuites")]
    pub r#cipher_suites: Option<Vec<String>>,
    /// A list of SSL Protocols which should be disabled on this Application Gateway. Possible values are `TLSv1_0`, `TLSv1_1`, `TLSv1_2` and `TLSv1_3`.
    /// 
    /// > **NOTE:** `disabled_protocols` cannot be set when `policy_name` or `policy_type` are set.
    #[builder(into)]
    #[serde(rename = "disabledProtocols")]
    pub r#disabled_protocols: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "minProtocolVersion")]
    pub r#min_protocol_version: Option<String>,
    #[builder(into)]
    #[serde(rename = "policyName")]
    pub r#policy_name: Option<String>,
    /// The Type of the Policy. Possible values are `Predefined`, `Custom` and `CustomV2`.
    /// 
    /// > **NOTE:** `policy_type` is Required when `policy_name` is set - cannot be set if `disabled_protocols` is set.
    #[builder(into)]
    #[serde(rename = "policyType")]
    pub r#policy_type: Option<String>,
}
