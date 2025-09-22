#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ListItemValue {
    #[builder(into)]
    #[serde(rename = "asn")]
    pub r#asn: Option<i32>,
    #[builder(into)]
    #[serde(rename = "hostnames")]
    pub r#hostnames: Option<Vec<super::types::ListItemValueHostname>>,
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Option<String>,
    #[builder(into)]
    #[serde(rename = "redirects")]
    pub r#redirects: Option<Vec<super::types::ListItemValueRedirect>>,
}
