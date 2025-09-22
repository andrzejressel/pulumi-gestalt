#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ReceiptRuleAddHeaderAction {
    /// The name of the header to add
    #[builder(into)]
    #[serde(rename = "headerName")]
    pub r#header_name: String,
    /// The value of the header to add
    #[builder(into)]
    #[serde(rename = "headerValue")]
    pub r#header_value: String,
    /// The position of the action in the receipt rule
    #[builder(into)]
    #[serde(rename = "position")]
    pub r#position: i32,
}
