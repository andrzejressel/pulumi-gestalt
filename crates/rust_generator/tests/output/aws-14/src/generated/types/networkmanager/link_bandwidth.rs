#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinkBandwidth {
    /// Download speed in Mbps.
    #[builder(into)]
    #[serde(rename = "downloadSpeed")]
    pub r#download_speed: Option<i32>,
    /// Upload speed in Mbps.
    #[builder(into)]
    #[serde(rename = "uploadSpeed")]
    pub r#upload_speed: Option<i32>,
}
