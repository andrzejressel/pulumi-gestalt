#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxTestCaseLastTestResultConversationTurnUserInputInputDtmf {
    /// The dtmf digits.
    #[builder(into)]
    #[serde(rename = "digits")]
    pub r#digits: Option<String>,
    /// The finish digit (if any).
    #[builder(into)]
    #[serde(rename = "finishDigit")]
    pub r#finish_digit: Option<String>,
}
