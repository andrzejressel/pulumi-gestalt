#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetResourcePolicySnapshotSchedulePolicySnapshotProperty {
    /// Creates the new snapshot in the snapshot chain labeled with the
    /// specified name. The chain name must be 1-63 characters long and comply
    /// with RFC1035.
    #[builder(into)]
    #[serde(rename = "chainName")]
    pub r#chain_name: String,
    /// Whether to perform a 'guest aware' snapshot.
    #[builder(into)]
    #[serde(rename = "guestFlush")]
    pub r#guest_flush: bool,
    /// A set of key-value pairs.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: std::collections::HashMap<String, String>,
    /// Cloud Storage bucket location to store the auto snapshot
    /// (regional or multi-regional)
    #[builder(into)]
    #[serde(rename = "storageLocations")]
    pub r#storage_locations: Vec<String>,
}
