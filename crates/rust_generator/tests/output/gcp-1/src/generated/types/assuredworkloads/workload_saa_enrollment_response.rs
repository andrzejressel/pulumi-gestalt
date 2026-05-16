#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkloadSaaEnrollmentResponse {
    /// Indicates SAA enrollment setup error if any.
    #[builder(into)]
    #[serde(rename = "setupErrors")]
    pub r#setup_errors: Option<Vec<String>>,
    /// Indicates SAA enrollment status of a given workload. Possible values: SETUP_STATE_UNSPECIFIED, STATUS_PENDING, STATUS_COMPLETE
    #[builder(into)]
    #[serde(rename = "setupStatus")]
    pub r#setup_status: Option<String>,
}
