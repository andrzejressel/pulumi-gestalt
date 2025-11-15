#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScaleSetIdentity {
    /// Specifies a list of user managed identity ids to be assigned to the VMSS. Required if `type` is `UserAssigned`.
    /// 
    /// ```yaml
    /// resources:
    ///   example:
    ///     type: azure:compute:ScaleSet
    ///     properties:
    ///       name: vm-scaleset
    ///       resourceGroupName: ${exampleAzurermResourceGroup.name}
    ///       location: ${exampleAzurermResourceGroup.location}
    ///       sku:
    ///         name: ${vmSku}
    ///         tier: Standard
    ///         capacity: ${instanceCount}
    ///       identity:
    ///         type: SystemAssigned
    ///       extensions:
    ///         - name: MSILinuxExtension
    ///           publisher: Microsoft.ManagedIdentity
    ///           type: ManagedIdentityExtensionForLinux
    ///           typeHandlerVersion: '1.0'
    ///           settings: '{"port": 50342}'
    /// outputs:
    ///   principalId: ${example.identity.principalId}
    /// ```
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Option<String>,
    /// Specifies the identity type to be assigned to the scale set. Allowable values are `SystemAssigned` and `UserAssigned`. For the `SystemAssigned` identity the scale set's Service Principal ID (SPN) can be retrieved after the scale set has been created. See [documentation](https://docs.microsoft.com/azure/active-directory/managed-service-identity/overview) for more information. Possible values are `SystemAssigned`, `UserAssigned` and `SystemAssigned, UserAssigned`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
