#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTaskSpecPlacement {
    /// An array of constraints. e.g.: `node.role==manager`
    #[builder(into)]
    #[serde(rename = "constraints")]
    pub r#constraints: Option<Vec<String>>,
    /// Maximum number of replicas for per node (default value is `0`, which is unlimited)
    #[builder(into)]
    #[serde(rename = "maxReplicas")]
    pub r#max_replicas: Option<i32>,
    /// Platforms stores all the platforms that the service's image can run on
    #[builder(into)]
    #[serde(rename = "platforms")]
    pub r#platforms: Option<Vec<super::types::ServiceTaskSpecPlacementPlatform>>,
    /// Preferences provide a way to make the scheduler aware of factors such as topology. They are provided in order from highest to lowest precedence, e.g.: `spread=node.role.manager`
    #[builder(into)]
    #[serde(rename = "prefs")]
    pub r#prefs: Option<Vec<String>>,
}
