#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorBackendPool {
    /// A `backend` block as defined below.
    #[builder(into)]
    #[serde(rename = "backends")]
    pub r#backends: Vec<super::super::types::frontdoor::FrontdoorBackendPoolBackend>,
    /// Specifies the name of the `backend_pool_health_probe` block within this resource to use for this `Backend Pool`.
    #[builder(into)]
    #[serde(rename = "healthProbeName")]
    pub r#health_probe_name: String,
    /// The ID of the FrontDoor.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Specifies the name of the `backend_pool_load_balancing` block within this resource to use for this `Backend Pool`.
    #[builder(into)]
    #[serde(rename = "loadBalancingName")]
    pub r#load_balancing_name: String,
    /// Specifies the name of the Backend Pool.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
