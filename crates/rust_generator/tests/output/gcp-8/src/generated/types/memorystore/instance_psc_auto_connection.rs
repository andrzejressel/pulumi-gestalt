#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstancePscAutoConnection {
    /// (Output)
    /// Output Only. Type of a PSC Connection.
    /// Possible values:
    /// CONNECTION_TYPE_DISCOVERY
    /// CONNECTION_TYPE_PRIMARY
    /// CONNECTION_TYPE_READER
    #[builder(into)]
    #[serde(rename = "connectionType")]
    pub r#connection_type: Option<String>,
    /// (Output)
    /// Output only. The URI of the consumer side forwarding rule.
    /// Format:
    /// projects/{project}/regions/{region}/forwardingRules/{forwarding_rule}
    #[builder(into)]
    #[serde(rename = "forwardingRule")]
    pub r#forwarding_rule: Option<String>,
    /// (Output)
    /// Output only. The IP allocated on the consumer network for the PSC forwarding rule.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Option<String>,
    /// (Output)
    /// Output only. The consumer network where the IP address resides, in the form of
    /// projects/{project_id}/global/networks/{network_id}.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// (Output)
    /// Output only. Ports of the exposed endpoint.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// (Output)
    /// Output only. The consumer project_id where the forwarding rule is created from.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: Option<String>,
    /// (Output)
    /// Output only. The PSC connection id of the forwarding rule connected to the
    /// service attachment.
    #[builder(into)]
    #[serde(rename = "pscConnectionId")]
    pub r#psc_connection_id: Option<String>,
    /// (Output)
    /// Output Only. The status of the PSC connection: whether a connection exists and ACTIVE or it no longer exists.
    /// Possible values:
    /// ACTIVE
    /// NOT_FOUND
    #[builder(into)]
    #[serde(rename = "pscConnectionStatus")]
    pub r#psc_connection_status: Option<String>,
    /// (Output)
    /// Output only. The service attachment which is the target of the PSC connection, in the form of projects/{project-id}/regions/{region}/serviceAttachments/{service-attachment-id}.
    #[builder(into)]
    #[serde(rename = "serviceAttachment")]
    pub r#service_attachment: Option<String>,
}
