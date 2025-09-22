#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceAccelerator {
    /// The type of an accelator for a CDF instance.
    /// Possible values are: `CDC`, `HEALTHCARE`, `CCAI_INSIGHTS`.
    #[builder(into)]
    #[serde(rename = "acceleratorType")]
    pub r#accelerator_type: String,
    /// The type of an accelator for a CDF instance.
    /// Possible values are: `ENABLED`, `DISABLED`.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: String,
}
