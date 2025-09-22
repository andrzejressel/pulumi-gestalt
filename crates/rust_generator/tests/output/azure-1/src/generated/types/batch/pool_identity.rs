#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PoolIdentity {
    /// Specifies a list of User Assigned Managed Identity IDs to be assigned to this Batch Account.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Vec<String>,
    /// Specifies the type of Managed Service Identity that should be configured on this Batch Account. Only possible value is `UserAssigned`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
