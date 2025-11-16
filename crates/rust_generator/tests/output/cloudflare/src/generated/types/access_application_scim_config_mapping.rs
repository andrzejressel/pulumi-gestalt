#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessApplicationScimConfigMapping {
    /// Whether or not this mapping is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// A [SCIM filter expression](https://datatracker.ietf.org/doc/html/rfc7644#section-3.4.2.2) that matches resources that should be provisioned to this application.
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: Option<String>,
    /// Whether or not this mapping applies to creates, updates, or deletes.
    #[builder(into)]
    #[serde(rename = "operations")]
    pub r#operations: Option<Box<super::types::AccessApplicationScimConfigMappingOperations>>,
    /// Which SCIM resource type this mapping applies to.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: String,
    /// A [JSONata](https://jsonata.org/) expression that transforms the resource before provisioning it in the application.
    #[builder(into)]
    #[serde(rename = "transformJsonata")]
    pub r#transform_jsonata: Option<String>,
}
