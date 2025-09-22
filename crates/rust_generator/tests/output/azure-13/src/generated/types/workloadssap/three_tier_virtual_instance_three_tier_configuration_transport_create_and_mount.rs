#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ThreeTierVirtualInstanceThreeTierConfigurationTransportCreateAndMount {
    /// The ID of the Resource Group of the transport File Share. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "resourceGroupId")]
    pub r#resource_group_id: Option<String>,
    /// The name of the Storage Account of the File Share. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "storageAccountName")]
    pub r#storage_account_name: Option<String>,
}
