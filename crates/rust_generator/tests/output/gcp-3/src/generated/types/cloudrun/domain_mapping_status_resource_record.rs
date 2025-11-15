#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainMappingStatusResourceRecord {
    /// Name should be a [verified](https://support.google.com/webmasters/answer/9008080) domain
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// (Output)
    /// Data for this record. Values vary by record type, as defined in RFC 1035
    /// (section 5) and RFC 1034 (section 3.6.1).
    #[builder(into)]
    #[serde(rename = "rrdata")]
    pub r#rrdata: Option<String>,
    /// Resource record type. Example: `AAAA`.
    /// Possible values are: `A`, `AAAA`, `CNAME`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
