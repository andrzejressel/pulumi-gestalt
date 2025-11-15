#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkAttachmentConnectionEndpoint {
    /// (Output)
    /// The IPv4 address assigned to the producer instance network interface. This value will be a range in case of Serverless.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Option<String>,
    /// (Output)
    /// The project id or number of the interface to which the IP was assigned.
    #[builder(into)]
    #[serde(rename = "projectIdOrNum")]
    pub r#project_id_or_num: Option<String>,
    /// (Output)
    /// Alias IP ranges from the same subnetwork.
    #[builder(into)]
    #[serde(rename = "secondaryIpCidrRanges")]
    pub r#secondary_ip_cidr_ranges: Option<String>,
    /// (Output)
    /// The status of a connected endpoint to this network attachment.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// (Output)
    /// The subnetwork used to assign the IP to the producer instance network interface.
    #[builder(into)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Option<String>,
}
