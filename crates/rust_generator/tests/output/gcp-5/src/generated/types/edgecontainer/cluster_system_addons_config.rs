#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterSystemAddonsConfig {
    /// Config for the Ingress add-on which allows customers to create an Ingress
    /// object to manage external access to the servers in a cluster. The add-on
    /// consists of istiod and istio-ingress.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ingress")]
    pub r#ingress: Option<Box<super::super::types::edgecontainer::ClusterSystemAddonsConfigIngress>>,
}
