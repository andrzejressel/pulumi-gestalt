#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTaskSpecResourcesReservationGenericResources {
    /// The Integer resources
    #[builder(into)]
    #[serde(rename = "discreteResourcesSpecs")]
    pub r#discrete_resources_specs: Option<Vec<String>>,
    /// The String resources
    #[builder(into)]
    #[serde(rename = "namedResourcesSpecs")]
    pub r#named_resources_specs: Option<Vec<String>>,
}
