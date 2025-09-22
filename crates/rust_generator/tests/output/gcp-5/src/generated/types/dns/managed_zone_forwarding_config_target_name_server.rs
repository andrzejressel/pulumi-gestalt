#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ManagedZoneForwardingConfigTargetNameServer {
    /// Forwarding path for this TargetNameServer. If unset or `default` Cloud DNS will make forwarding
    /// decision based on address ranges, i.e. RFC1918 addresses go to the VPC, Non-RFC1918 addresses go
    /// to the Internet. When set to `private`, Cloud DNS will always send queries through VPC for this target
    /// Possible values are: `default`, `private`.
    #[builder(into)]
    #[serde(rename = "forwardingPath")]
    pub r#forwarding_path: Option<String>,
    /// IPv4 address of a target name server.
    #[builder(into)]
    #[serde(rename = "ipv4Address")]
    pub r#ipv_4_address: String,
}
