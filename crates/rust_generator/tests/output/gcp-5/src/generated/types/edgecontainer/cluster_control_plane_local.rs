#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterControlPlaneLocal {
    /// Only machines matching this filter will be allowed to host control
    /// plane nodes. The filtering language accepts strings like "name=<name>",
    /// and is documented here: [AIP-160](https://google.aip.dev/160).
    #[builder(into)]
    #[serde(rename = "machineFilter")]
    pub r#machine_filter: Option<String>,
    /// The number of nodes to serve as replicas of the Control Plane.
    /// Only 1 and 3 are supported.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Option<i32>,
    /// Name of the Google Distributed Cloud Edge zones where this node pool
    /// will be created. For example: `us-central1-edge-customer-a`.
    #[builder(into)]
    #[serde(rename = "nodeLocation")]
    pub r#node_location: Option<String>,
    /// Policy configuration about how user applications are deployed.
    /// Possible values are: `SHARED_DEPLOYMENT_POLICY_UNSPECIFIED`, `ALLOWED`, `DISALLOWED`.
    #[builder(into)]
    #[serde(rename = "sharedDeploymentPolicy")]
    pub r#shared_deployment_policy: Option<String>,
}
