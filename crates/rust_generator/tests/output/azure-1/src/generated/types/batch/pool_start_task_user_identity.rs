#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PoolStartTaskUserIdentity {
    /// A `auto_user` block that describes the user identity under which the start task runs as defined below.
    /// 
    /// > **Please Note:** `user_name` and `auto_user` blocks cannot be used both at the same time, but you need to define one or the other.
    #[builder(into)]
    #[serde(rename = "autoUser")]
    pub r#auto_user: Option<Box<super::super::types::batch::PoolStartTaskUserIdentityAutoUser>>,
    /// The username to be used by the Batch pool start task.
    #[builder(into)]
    #[serde(rename = "userName")]
    pub r#user_name: Option<String>,
}
