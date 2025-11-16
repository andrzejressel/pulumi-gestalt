#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MxRecordRecord {
    /// The FQDN of the exchange to MX record points to.
    #[builder(into)]
    #[serde(rename = "exchange")]
    pub r#exchange: String,
    /// The preference of the MX record.
    #[builder(into)]
    #[serde(rename = "preference")]
    pub r#preference: i32,
}
