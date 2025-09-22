#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VpnSiteO365Policy {
    /// A `traffic_category` block as defined above.
    #[builder(into)]
    #[serde(rename = "trafficCategory")]
    pub r#traffic_category: Option<Box<super::super::types::network::VpnSiteO365PolicyTrafficCategory>>,
}
