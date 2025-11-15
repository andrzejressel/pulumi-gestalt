#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AwsClusterFleet {
    /// The name of the managed Hub Membership resource associated to this cluster. Membership names are formatted as projects/<project-number>/locations/global/membership/<cluster-id>.
    #[builder(into)]
    #[serde(rename = "membership")]
    pub r#membership: Option<String>,
    /// The number of the Fleet host project where this cluster will be registered.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: Option<String>,
}
