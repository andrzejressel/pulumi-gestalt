#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkSliceSingleNetworkSliceSelectionAssistanceInformation {
    /// Slice differentiator (SD). Must be a 6 digit hex string.
    #[builder(into)]
    #[serde(rename = "sliceDifferentiator")]
    pub r#slice_differentiator: Option<String>,
    /// Slice/service type (SST). Must be between `0` and `255`.
    #[builder(into)]
    #[serde(rename = "sliceServiceType")]
    pub r#slice_service_type: i32,
}
