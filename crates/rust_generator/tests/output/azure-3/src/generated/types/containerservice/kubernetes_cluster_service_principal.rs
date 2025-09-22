#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KubernetesClusterServicePrincipal {
    /// The Client ID for the Service Principal.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// The Client Secret for the Service Principal.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: String,
}
