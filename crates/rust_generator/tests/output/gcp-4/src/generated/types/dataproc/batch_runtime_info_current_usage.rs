#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BatchRuntimeInfoCurrentUsage {
    /// (Output)
    /// Accelerator type being used, if any.
    #[builder(into)]
    #[serde(rename = "acceleratorType")]
    pub r#accelerator_type: Option<String>,
    /// (Output)
    /// Milli (one-thousandth) accelerator..
    #[builder(into)]
    #[serde(rename = "milliAccelerator")]
    pub r#milli_accelerator: Option<String>,
    /// (Output)
    /// Milli (one-thousandth) Dataproc Compute Units (DCUs).
    #[builder(into)]
    #[serde(rename = "milliDcu")]
    pub r#milli_dcu: Option<String>,
    /// (Output)
    /// Milli (one-thousandth) Dataproc Compute Units (DCUs) charged at premium tier.
    #[builder(into)]
    #[serde(rename = "milliDcuPremium")]
    pub r#milli_dcu_premium: Option<String>,
    /// (Output)
    /// Shuffle Storage in gigabytes (GB).
    #[builder(into)]
    #[serde(rename = "shuffleStorageGb")]
    pub r#shuffle_storage_gb: Option<String>,
    /// (Output)
    /// Shuffle Storage in gigabytes (GB) charged at premium tier.
    #[builder(into)]
    #[serde(rename = "shuffleStorageGbPremium")]
    pub r#shuffle_storage_gb_premium: Option<String>,
    /// (Output)
    /// The timestamp of the usage snapshot.
    #[builder(into)]
    #[serde(rename = "snapshotTime")]
    pub r#snapshot_time: Option<String>,
}
