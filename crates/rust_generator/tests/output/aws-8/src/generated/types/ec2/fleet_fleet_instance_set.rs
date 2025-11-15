#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetFleetInstanceSet {
    /// The IDs of the instances.
    #[builder(into)]
    #[serde(rename = "instanceIds")]
    pub r#instance_ids: Option<Vec<String>>,
    /// The instance type.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Option<String>,
    /// Indicates if the instance that was launched is a Spot Instance or On-Demand Instance.
    #[builder(into)]
    #[serde(rename = "lifecycle")]
    pub r#lifecycle: Option<String>,
    /// The value is `Windows` for Windows instances. Otherwise, the value is blank.
    #[builder(into)]
    #[serde(rename = "platform")]
    pub r#platform: Option<String>,
}
