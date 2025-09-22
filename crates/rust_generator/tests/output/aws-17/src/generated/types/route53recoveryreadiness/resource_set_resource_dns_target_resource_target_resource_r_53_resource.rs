#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResourceSetResourceDnsTargetResourceTargetResourceR53Resource {
    /// Domain name that is targeted.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Option<String>,
    /// Resource record set ID that is targeted.
    #[builder(into)]
    #[serde(rename = "recordSetId")]
    pub r#record_set_id: Option<String>,
}
