#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigPolicyContentBundle {
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "bundle")]
    pub r#bundle: String,
    /// The set of namespaces to be exempted from the bundle.
    #[builder(into)]
    #[serde(rename = "exemptedNamespaces")]
    pub r#exempted_namespaces: Option<Vec<String>>,
}
