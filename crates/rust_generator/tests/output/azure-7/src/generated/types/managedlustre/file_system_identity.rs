#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FileSystemIdentity {
    /// A list of User Assigned Managed Identity IDs to be assigned to this Azure Managed Lustre File System. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Vec<String>,
    /// The type of Managed Service Identity that should be configured on this Azure Managed Lustre File System. Only possible value is `UserAssigned`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
