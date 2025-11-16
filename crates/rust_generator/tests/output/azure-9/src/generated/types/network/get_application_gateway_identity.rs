#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetApplicationGatewayIdentity {
    /// The list of User Assigned Managed Identity IDs assigned to this Application Gateway.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Vec<String>,
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: String,
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: String,
    /// The type of Managed Service Identity that is configured on this Application Gateway.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
