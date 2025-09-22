#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainMappingResourceRecord {
    /// Relative name of the object affected by this record. Only applicable for CNAME records. Example: 'www'.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Data for this record. Values vary by record type, as defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1).
    #[builder(into)]
    #[serde(rename = "rrdata")]
    pub r#rrdata: Option<String>,
    /// Resource record type. Example: `AAAA`.
    /// Possible values are: `A`, `AAAA`, `CNAME`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
