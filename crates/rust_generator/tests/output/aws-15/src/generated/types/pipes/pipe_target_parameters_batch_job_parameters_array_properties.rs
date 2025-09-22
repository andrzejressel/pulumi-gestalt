#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipeTargetParametersBatchJobParametersArrayProperties {
    /// The size of the array, if this is an array batch job. Minimum value of 2. Maximum value of 10,000.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Option<i32>,
}
