#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsFecOutputSettings {
    /// The height of the FEC protection matrix.
    #[builder(into)]
    #[serde(rename = "columnDepth")]
    pub r#column_depth: Option<i32>,
    /// Enables column only or column and row based FEC.
    #[builder(into)]
    #[serde(rename = "includeFec")]
    pub r#include_fec: Option<String>,
    /// The width of the FEC protection matrix.
    #[builder(into)]
    #[serde(rename = "rowLength")]
    pub r#row_length: Option<i32>,
}
