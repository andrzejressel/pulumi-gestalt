#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicySpec {
    /// (Output)
    /// An opaque tag indicating the current version of the `Policy`, used for concurrency control. This field is ignored if used in a `CreatePolicy` request. When the `Policy` is returned from either a `GetPolicy` or a `ListPolicies` request, this `etag` indicates the version of the current `Policy` to use when executing a read-modify-write loop. When the `Policy` is returned from a `GetEffectivePolicy` request, the `etag` will be unset.
    #[builder(into, default)]
    #[serde(rename = "etag")]
    pub r#etag: Box<Option<String>>,
    /// Determines the inheritance behavior for this `Policy`. If `inherit_from_parent` is true, PolicyRules set higher up in the hierarchy (up to the closest root) are inherited and present in the effective policy. If it is false, then no rules are inherited, and this Policy becomes the new root for evaluation. This field can be set only for Policies which configure list constraints.
    #[builder(into, default)]
    #[serde(rename = "inheritFromParent")]
    pub r#inherit_from_parent: Box<Option<bool>>,
    /// Ignores policies set above this resource and restores the `constraint_default` enforcement behavior of the specific `Constraint` at this resource. This field can be set in policies for either list or boolean constraints. If set, `rules` must be empty and `inherit_from_parent` must be set to false.
    #[builder(into, default)]
    #[serde(rename = "reset")]
    pub r#reset: Box<Option<bool>>,
    /// In Policies for boolean constraints, the following requirements apply: - There must be one and only one PolicyRule where condition is unset. - BooleanPolicyRules with conditions must set `enforced` to the opposite of the PolicyRule without a condition. - During policy evaluation, PolicyRules with conditions that are true for a target resource take precedence.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<Vec<super::super::types::orgpolicy::PolicySpecRule>>>,
    /// (Output)
    /// Output only. The time stamp this was previously updated. This represents the last time a call to `CreatePolicy` or `UpdatePolicy` was made for that `Policy`.
    #[builder(into, default)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Box<Option<String>>,
}
