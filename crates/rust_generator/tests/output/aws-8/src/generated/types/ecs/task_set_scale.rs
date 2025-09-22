#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TaskSetScale {
    /// The unit of measure for the scale value. Default: `PERCENT`.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: Option<String>,
    /// The value, specified as a percent total of a service's `desiredCount`, to scale the task set. Defaults to `0` if not specified. Accepted values are numbers between 0.0 and 100.0.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<f64>,
}
