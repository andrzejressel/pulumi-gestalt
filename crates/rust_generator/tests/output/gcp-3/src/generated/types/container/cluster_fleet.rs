#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterFleet {
    /// The resource name of the fleet Membership resource associated to this cluster with format `//gkehub.googleapis.com/projects/{{project}}/locations/{{location}}/memberships/{{name}}`. See the official doc for [fleet management](https://cloud.google.com/kubernetes-engine/docs/fleets-overview).
    #[builder(into)]
    #[serde(rename = "membership")]
    pub r#membership: Option<String>,
    /// The short name of the fleet membership, extracted from `fleet.0.membership`. You can use this field to configure `membership_id` under google_gkehub_feature_membership.
    #[builder(into)]
    #[serde(rename = "membershipId")]
    pub r#membership_id: Option<String>,
    /// The location of the fleet membership,  extracted from `fleet.0.membership`. You can use this field to configure `membership_location` under google_gkehub_feature_membership.
    #[builder(into)]
    #[serde(rename = "membershipLocation")]
    pub r#membership_location: Option<String>,
    /// Whether the cluster has been registered via the fleet API.
    #[builder(into)]
    #[serde(rename = "preRegistered")]
    pub r#pre_registered: Option<bool>,
    /// The name of the Fleet host project where this cluster will be registered.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: Option<String>,
}
