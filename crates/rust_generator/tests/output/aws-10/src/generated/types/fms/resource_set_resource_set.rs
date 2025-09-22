#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResourceSetResourceSet {
    /// Description of the resource set.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Unique identifier for the resource set. It's returned in the responses to create and list commands. You provide it to operations like update and delete.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Last time that the reosurce set was changed.
    #[builder(into)]
    #[serde(rename = "lastUpdateTime")]
    pub r#last_update_time: Option<String>,
    /// Descriptive name of the resource set. You can't change the name of a resource set after you create it.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Indicates whether the resource set is in or out of the admin's Region scope. Valid values are `ACTIVE` (Admin can manage and delete the resource set) or `OUT_OF_ADMIN_SCOPE` (Admin can view the resource set, but theyy can't edit or delete the resource set.)
    #[builder(into)]
    #[serde(rename = "resourceSetStatus")]
    pub r#resource_set_status: Option<String>,
    /// Determines the resources that can be associated to the resource set. Depending on your setting for max results and the number of resource sets, a single call might not return the full list.
    #[builder(into)]
    #[serde(rename = "resourceTypeLists")]
    pub r#resource_type_lists: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "updateToken")]
    pub r#update_token: Option<String>,
}
