#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceQuotaUsageMetricMetricDimension {
    #[builder(into)]
    #[serde(rename = "class")]
    pub r#class: Option<String>,
    #[builder(into)]
    #[serde(rename = "resource")]
    pub r#resource: Option<String>,
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Option<String>,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
