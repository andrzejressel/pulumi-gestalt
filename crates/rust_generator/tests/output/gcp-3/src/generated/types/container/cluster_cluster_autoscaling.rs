#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterClusterAutoscaling {
    /// Contains defaults for a node pool created by NAP. A subset of fields also apply to
    /// GKE Autopilot clusters.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "autoProvisioningDefaults")]
    pub r#auto_provisioning_defaults: Box<Option<super::super::types::container::ClusterClusterAutoscalingAutoProvisioningDefaults>>,
    /// The list of Google Compute Engine 
    /// [zones](https://cloud.google.com/compute/docs/zones#available) in which the
    /// NodePool's nodes can be created by NAP.
    #[builder(into)]
    #[serde(rename = "autoProvisioningLocations")]
    pub r#auto_provisioning_locations: Option<Vec<String>>,
    /// Configuration
    /// options for the [Autoscaling profile](https://cloud.google.com/kubernetes-engine/docs/concepts/cluster-autoscaler#autoscaling_profiles)
    /// feature, which lets you choose whether the cluster autoscaler should optimize for resource utilization or resource availability
    /// when deciding to remove nodes from a cluster. Can be `BALANCED` or `OPTIMIZE_UTILIZATION`. Defaults to `BALANCED`.
    #[builder(into)]
    #[serde(rename = "autoscalingProfile")]
    pub r#autoscaling_profile: Option<String>,
    /// Whether node auto-provisioning is enabled. Must be supplied for GKE Standard clusters, `true` is implied
    /// for autopilot clusters. Resource limits for `cpu` and `memory` must be defined to enable node auto-provisioning for GKE Standard.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Global constraints for machine resources in the
    /// cluster. Configuring the `cpu` and `memory` types is required if node
    /// auto-provisioning is enabled. These limits will apply to node pool autoscaling
    /// in addition to node auto-provisioning. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "resourceLimits")]
    pub r#resource_limits: Option<Vec<super::super::types::container::ClusterClusterAutoscalingResourceLimit>>,
}
