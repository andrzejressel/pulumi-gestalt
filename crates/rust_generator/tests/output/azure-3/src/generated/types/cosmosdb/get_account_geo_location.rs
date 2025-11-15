#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAccountGeoLocation {
    #[builder(into)]
    #[serde(rename = "failoverPriority")]
    pub r#failover_priority: i32,
    /// The ID of the virtual network subnet.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The name of the Azure region hosting replicated data.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
}
