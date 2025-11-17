#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkloadWorkloadOptions {
    /// Indicates type of KAJ enrollment for the workload. Currently, only specifiying KEY_ACCESS_TRANSPARENCY_OFF is implemented to not enroll in KAT-level KAJ enrollment for Regional Controls workloads. Possible values: KAJ_ENROLLMENT_TYPE_UNSPECIFIED, FULL_KAJ, EKM_ONLY, KEY_ACCESS_TRANSPARENCY_OFF
    #[builder(into)]
    #[serde(rename = "kajEnrollmentType")]
    pub r#kaj_enrollment_type: Option<String>,
}
