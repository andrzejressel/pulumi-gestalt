#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AuthzPolicyTarget {
    /// All gateways and forwarding rules referenced by this policy and extensions must share the same load balancing scheme.
    /// For more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service).
    /// Possible values are: `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`, `INTERNAL_SELF_MANAGED`.
    #[builder(into)]
    #[serde(rename = "loadBalancingScheme")]
    pub r#load_balancing_scheme: String,
    /// A list of references to the Forwarding Rules on which this policy will be applied.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Option<Vec<String>>,
}
