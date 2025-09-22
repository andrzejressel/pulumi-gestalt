#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MonitorIdentity {
    /// The Principal ID for the Service Principal associated with the Identity of this Datadog Monitor.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Option<String>,
    /// The Tenant ID for the Service Principal associated with the Identity of this Datadog Monitor.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Option<String>,
    /// Specifies the identity type of the Datadog Monitor. At this time the only allowed value is `SystemAssigned`.
    /// 
    /// > **NOTE:** The assigned `principal_id` and `tenant_id` can be retrieved after the identity `type` has been set to `SystemAssigned` and the Datadog Monitor has been created. More details are available below.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
