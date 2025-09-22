#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TrafficManagerAzureEndpointSubnet {
    /// The first IP Address in this subnet.
    #[builder(into)]
    #[serde(rename = "first")]
    pub r#first: String,
    /// The last IP Address in this subnet.
    #[builder(into)]
    #[serde(rename = "last")]
    pub r#last: Option<String>,
    /// The block size (number of leading bits in the subnet mask).
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: Option<i32>,
}
