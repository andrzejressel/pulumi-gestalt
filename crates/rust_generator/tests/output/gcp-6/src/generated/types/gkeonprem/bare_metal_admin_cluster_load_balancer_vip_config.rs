#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BareMetalAdminClusterLoadBalancerVipConfig {
    /// The VIP which you previously set aside for the Kubernetes API of this Bare Metal Admin Cluster.
    #[builder(into)]
    #[serde(rename = "controlPlaneVip")]
    pub r#control_plane_vip: Box<String>,
}
