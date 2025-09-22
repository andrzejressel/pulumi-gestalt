#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DeploymentConfigTrafficRoutingConfigTimeBasedLinear {
    /// The number of minutes between each incremental traffic shift of a `TimeBasedLinear` deployment.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Option<i32>,
    /// The percentage of traffic that is shifted at the start of each increment of a `TimeBasedLinear` deployment.
    #[builder(into)]
    #[serde(rename = "percentage")]
    pub r#percentage: Option<i32>,
}
