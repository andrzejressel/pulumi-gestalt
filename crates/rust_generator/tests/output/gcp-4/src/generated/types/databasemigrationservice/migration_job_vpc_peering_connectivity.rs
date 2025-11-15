#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MigrationJobVpcPeeringConnectivity {
    /// The name of the VPC network to peer with the Cloud SQL private network.
    #[builder(into)]
    #[serde(rename = "vpc")]
    pub r#vpc: Option<String>,
}
