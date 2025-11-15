#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ManagedInstanceFailoverGroupPartnerRegion {
    /// The Azure Region where the Managed Instance Failover Group should exist. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Option<String>,
    /// The partner replication role of the Managed Instance Failover Group.
    #[builder(into)]
    #[serde(rename = "role")]
    pub r#role: Option<String>,
}
