#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDatabaseInstanceIpAddress {
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: String,
    #[builder(into)]
    #[serde(rename = "timeToRetire")]
    pub r#time_to_retire: String,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
