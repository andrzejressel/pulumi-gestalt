#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccountIdentity {
    /// The Principal ID for the Service Principal associated with the Identity of this Data Share Account.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Option<String>,
    /// The Tenant ID for the Service Principal associated with the Identity of this Data Share Account.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Option<String>,
    /// Specifies the type of Managed Service Identity that should be configured on this Data Share Account. The only possible value is `SystemAssigned`. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** The assigned `principal_id` and `tenant_id` can be retrieved after the identity `type` has been set to `SystemAssigned` and the Data Share Account has been created. More details are available below.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
