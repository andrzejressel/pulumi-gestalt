#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OrganizationsPolicyBindingTarget {
    /// Required. Immutable. The resource name of the policy to be bound.
    /// The binding parent and policy must belong to the same Organization (or Project).
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "principalSet")]
    pub r#principal_set: Option<String>,
}
