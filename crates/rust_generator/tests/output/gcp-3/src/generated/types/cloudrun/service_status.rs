#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceStatus {
    /// (Output)
    /// Array of observed Service Conditions, indicating the current ready state of the service.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "conditions")]
    pub r#conditions: Option<Vec<super::super::types::cloudrun::ServiceStatusCondition>>,
    /// (Output)
    /// From ConfigurationStatus. LatestCreatedRevisionName is the last revision that was created
    /// from this Service's Configuration. It might not be ready yet, for that use
    /// LatestReadyRevisionName.
    #[builder(into)]
    #[serde(rename = "latestCreatedRevisionName")]
    pub r#latest_created_revision_name: Option<String>,
    /// (Output)
    /// From ConfigurationStatus. LatestReadyRevisionName holds the name of the latest Revision
    /// stamped out from this Service's Configuration that has had its "Ready" condition become
    /// "True".
    #[builder(into)]
    #[serde(rename = "latestReadyRevisionName")]
    pub r#latest_ready_revision_name: Option<String>,
    /// (Output)
    /// ObservedGeneration is the 'Generation' of the Route that was last processed by the
    /// controller.
    /// Clients polling for completed reconciliation should poll until observedGeneration =
    /// metadata.generation and the Ready condition's status is True or False.
    #[builder(into)]
    #[serde(rename = "observedGeneration")]
    pub r#observed_generation: Option<i32>,
    /// (Output)
    /// Traffic specifies how to distribute traffic over a collection of Knative Revisions
    /// and Configurations
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "traffics")]
    pub r#traffics: Option<Vec<super::super::types::cloudrun::ServiceStatusTraffic>>,
    /// (Output)
    /// URL displays the URL for accessing tagged traffic targets. URL is displayed in status,
    /// and is disallowed on spec. URL must contain a scheme (e.g. http://) and a hostname,
    /// but may not contain anything else (e.g. basic auth, url path, etc.)
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}
