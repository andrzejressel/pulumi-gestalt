#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationInteractiveConfiguration {
    /// Enables an Apache Livy endpoint that you can connect to and run interactive jobs.
    #[builder(into)]
    #[serde(rename = "livyEndpointEnabled")]
    pub r#livy_endpoint_enabled: Option<bool>,
    /// Enables you to connect an application to Amazon EMR Studio to run interactive workloads in a notebook.
    #[builder(into)]
    #[serde(rename = "studioEnabled")]
    pub r#studio_enabled: Option<bool>,
}
