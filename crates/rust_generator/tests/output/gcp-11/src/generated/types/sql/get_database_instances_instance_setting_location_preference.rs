#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDatabaseInstancesInstanceSettingLocationPreference {
    /// A Google App Engine application whose zone to remain in. Must be in the same region as this instance.
    #[builder(into)]
    #[serde(rename = "followGaeApplication")]
    pub r#follow_gae_application: Box<String>,
    /// The preferred Compute Engine zone for the secondary/failover
    #[builder(into)]
    #[serde(rename = "secondaryZone")]
    pub r#secondary_zone: Box<String>,
    /// To filter out the Cloud SQL instances which are located in the specified zone. This zone refers to the Compute Engine zone that the instance is currently serving from.
    #[builder(into)]
    #[serde(rename = "zone")]
    pub r#zone: Box<String>,
}
