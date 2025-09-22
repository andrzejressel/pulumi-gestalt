#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ManagedClusterAuthentication {
    /// A `active_directory` block as defined above.
    #[builder(into)]
    #[serde(rename = "activeDirectory")]
    pub r#active_directory: Option<Box<super::super::types::servicefabric::ManagedClusterAuthenticationActiveDirectory>>,
    /// One or more `certificate` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "certificates")]
    pub r#certificates: Option<Vec<super::super::types::servicefabric::ManagedClusterAuthenticationCertificate>>,
}
