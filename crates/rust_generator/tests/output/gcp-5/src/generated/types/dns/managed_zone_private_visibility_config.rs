#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ManagedZonePrivateVisibilityConfig {
    /// The list of Google Kubernetes Engine clusters that can see this zone.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "gkeClusters")]
    pub r#gke_clusters: Option<Vec<super::super::types::dns::ManagedZonePrivateVisibilityConfigGkeCluster>>,
    #[builder(into)]
    #[serde(rename = "networks")]
    pub r#networks: Option<Vec<super::super::types::dns::ManagedZonePrivateVisibilityConfigNetwork>>,
}
