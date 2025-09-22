#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointGlobalDeliveryRuleModifyResponseHeaderAction {
    /// Action to be executed on a header value. Valid values are `Append`, `Delete` and `Overwrite`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// The header name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The value of the header. Only needed when `action` is set to `Append` or `overwrite`.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
