#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DeploymentFrontendPrivate {
    /// Specify the method for allocating the private IP. Possible values are `Static` and `Dynamic`. Changing this forces a new NGINX Deployment to be created.
    #[builder(into)]
    #[serde(rename = "allocationMethod")]
    pub r#allocation_method: String,
    /// Specify the private IP Address. Changing this forces a new NGINX Deployment to be created.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: String,
    /// Specify the Subnet Resource ID for this NGINX Deployment. Changing this forces a new NGINX Deployment to be created.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}
