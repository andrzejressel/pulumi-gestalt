#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetNetworkManagerConnectivityConfigurationHub {
    /// The resource ID used as hub in Hub and Spoke topology.
    #[builder(into)]
    #[serde(rename = "resourceId")]
    pub r#resource_id: String,
    /// The resource type used as hub in Hub and Spoke topology.
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: String,
}
