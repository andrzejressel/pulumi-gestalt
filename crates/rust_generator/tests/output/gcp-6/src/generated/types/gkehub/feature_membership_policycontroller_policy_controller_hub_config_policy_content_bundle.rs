#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureMembershipPolicycontrollerPolicyControllerHubConfigPolicyContentBundle {
    /// The name of the bundle.
    #[builder(into)]
    #[serde(rename = "bundleName")]
    pub r#bundle_name: String,
    /// The set of namespaces to be exempted from the bundle.
    #[builder(into)]
    #[serde(rename = "exemptedNamespaces")]
    pub r#exempted_namespaces: Option<Vec<String>>,
}
