#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetMxRecordRecord {
    /// The mail server responsible for the domain covered by the MX record.
    #[builder(into)]
    #[serde(rename = "exchange")]
    pub r#exchange: String,
    /// String representing the "preference” value of the MX records. Records with lower preference value take priority.
    #[builder(into)]
    #[serde(rename = "preference")]
    pub r#preference: i32,
}
