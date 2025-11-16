#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LifecyclePolicyPolicyDetailsEventSourceParameters {
    /// The snapshot description that can trigger the policy. The description pattern is specified using a regular expression. The policy runs only if a snapshot with a description that matches the specified pattern is shared with your account.
    #[builder(into)]
    #[serde(rename = "descriptionRegex")]
    pub r#description_regex: String,
    /// The type of event. Currently, only `shareSnapshot` events are supported.
    #[builder(into)]
    #[serde(rename = "eventType")]
    pub r#event_type: String,
    /// The IDs of the AWS accounts that can trigger policy by sharing snapshots with your account. The policy only runs if one of the specified AWS accounts shares a snapshot with your account.
    #[builder(into)]
    #[serde(rename = "snapshotOwners")]
    pub r#snapshot_owners: Vec<String>,
}
