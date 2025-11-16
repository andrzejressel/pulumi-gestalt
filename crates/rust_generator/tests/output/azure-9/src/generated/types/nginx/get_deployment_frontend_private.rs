#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDeploymentFrontendPrivate {
    /// The method of allocating the private IP to the NGINX Deployment.
    #[builder(into)]
    #[serde(rename = "allocationMethod")]
    pub r#allocation_method: String,
    /// The list of Public IP Resource IDs for this NGINX Deployment.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: String,
    /// The subnet resource ID of the NGINX Deployment.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}
