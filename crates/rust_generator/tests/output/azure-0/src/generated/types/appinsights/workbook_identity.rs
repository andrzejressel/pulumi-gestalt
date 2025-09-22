#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkbookIdentity {
    /// The list of User Assigned Managed Identity IDs assigned to this Workbook. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Option<Vec<String>>,
    /// The Principal ID of the System Assigned Managed Service Identity that is configured on this Workbook.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Option<String>,
    /// The Tenant ID of the System Assigned Managed Service Identity that is configured on this Workbook.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Option<String>,
    /// The type of Managed Service Identity that is configured on this Workbook. Possible values are `UserAssigned`, `SystemAssigned` and `SystemAssigned, UserAssigned`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
