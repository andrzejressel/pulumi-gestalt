#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureSpec {
    /// Clusterupgrade feature spec.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "clusterupgrade")]
    pub r#clusterupgrade: Box<Option<super::super::types::gkehub::FeatureSpecClusterupgrade>>,
    /// Fleet Observability feature spec.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fleetobservability")]
    pub r#fleetobservability: Box<Option<super::super::types::gkehub::FeatureSpecFleetobservability>>,
    /// Multicluster Ingress-specific spec.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "multiclusteringress")]
    pub r#multiclusteringress: Box<Option<super::super::types::gkehub::FeatureSpecMulticlusteringress>>,
}
