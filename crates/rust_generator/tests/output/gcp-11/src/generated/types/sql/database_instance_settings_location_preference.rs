#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatabaseInstanceSettingsLocationPreference {
    /// A GAE application whose zone to remain
    /// in. Must be in the same region as this instance.
    #[builder(into)]
    #[serde(rename = "followGaeApplication")]
    pub r#follow_gae_application: Option<String>,
    /// The preferred Compute Engine zone for the secondary/failover.
    #[builder(into)]
    #[serde(rename = "secondaryZone")]
    pub r#secondary_zone: Option<String>,
    /// The preferred compute engine
    /// [zone](https://cloud.google.com/compute/docs/zones?hl=en).
    #[builder(into)]
    #[serde(rename = "zone")]
    pub r#zone: Option<String>,
}
