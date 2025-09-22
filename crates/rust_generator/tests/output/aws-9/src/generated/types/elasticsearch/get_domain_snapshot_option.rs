#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDomainSnapshotOption {
    /// Hour during which the service takes an automated daily snapshot of the indices in the domain.
    #[builder(into)]
    #[serde(rename = "automatedSnapshotStartHour")]
    pub r#automated_snapshot_start_hour: i32,
}
