#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EventSubscriptionDeadLetterIdentity {
    /// Specifies the type of Managed Service Identity that is used for dead lettering. Allowed value is `SystemAssigned`, `UserAssigned`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// The user identity associated with the resource.
    #[builder(into)]
    #[serde(rename = "userAssignedIdentity")]
    pub r#user_assigned_identity: Option<String>,
}
