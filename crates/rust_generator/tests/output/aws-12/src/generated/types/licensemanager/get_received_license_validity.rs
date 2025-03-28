#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetReceivedLicenseValidity {
    /// Start of the validity time range.
    #[builder(into)]
    #[serde(rename = "begin")]
    pub r#begin: Box<String>,
    /// End of the validity time range.
    #[builder(into)]
    #[serde(rename = "end")]
    pub r#end: Box<String>,
}
