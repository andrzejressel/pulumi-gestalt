#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BatchRuntimeInfoApproximateUsage {
    /// (Output)
    /// Accelerator type being used, if any.
    #[builder(into)]
    #[serde(rename = "acceleratorType")]
    pub r#accelerator_type: Option<String>,
    /// (Output)
    /// Accelerator usage in (milliAccelerator x seconds)
    #[builder(into)]
    #[serde(rename = "milliAcceleratorSeconds")]
    pub r#milli_accelerator_seconds: Option<String>,
    /// (Output)
    /// DCU (Dataproc Compute Units) usage in (milliDCU x seconds)
    #[builder(into)]
    #[serde(rename = "milliDcuSeconds")]
    pub r#milli_dcu_seconds: Option<String>,
    /// (Output)
    /// Shuffle storage usage in (GB x seconds)
    #[builder(into)]
    #[serde(rename = "shuffleStorageGbSeconds")]
    pub r#shuffle_storage_gb_seconds: Option<String>,
}
