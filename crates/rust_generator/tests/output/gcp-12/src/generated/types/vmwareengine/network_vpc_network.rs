#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkVpcNetwork {
    /// (Output)
    /// The relative resource name of the service VPC network this VMware Engine network is attached to.
    /// For example: projects/123123/global/networks/my-network
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// VMware Engine network type.
    /// Possible values are: `LEGACY`, `STANDARD`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
