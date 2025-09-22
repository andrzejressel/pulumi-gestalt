#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterAzureActiveDirectory {
    /// The Azure Active Directory Client ID which should be used for the Client Application.
    #[builder(into)]
    #[serde(rename = "clientApplicationId")]
    pub r#client_application_id: String,
    /// The Azure Active Directory Cluster Application ID.
    #[builder(into)]
    #[serde(rename = "clusterApplicationId")]
    pub r#cluster_application_id: String,
    /// The Azure Active Directory Tenant ID.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: String,
}
