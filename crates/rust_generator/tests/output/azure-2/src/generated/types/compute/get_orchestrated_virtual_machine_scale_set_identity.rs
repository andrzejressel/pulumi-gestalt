#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetOrchestratedVirtualMachineScaleSetIdentity {
    /// The list of User Assigned Managed Identity IDs assigned to this Orchestrated Virtual Machine Scale Set.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Vec<String>>,
    /// The Principal ID of the System Assigned Managed Service Identity that is configured on this Orchestrated Virtual Machine Scale Set.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    /// The Tenant ID of the System Assigned Managed Service Identity that is configured on this Orchestrated Virtual Machine Scale Set.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
    /// The Type of IP Tag.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
