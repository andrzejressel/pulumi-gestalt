#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingDestination {
    /// Reference ID for the destination.
    #[builder(into)]
    #[serde(rename = "destinationRefId")]
    pub r#destination_ref_id: Box<String>,
}
