#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InterconnectAttachmentPrivateInterconnectInfo {
    /// (Output)
    /// 802.1q encapsulation tag to be used for traffic between
    /// Google and the customer, going to and from this network and region.
    #[builder(into)]
    #[serde(rename = "tag8021q")]
    pub r#tag_8021_q: Option<i32>,
}
