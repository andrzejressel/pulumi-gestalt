#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxTestCaseLastTestResult {
    /// The conversation turns uttered during the test case replay in chronological order.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "conversationTurns")]
    pub r#conversation_turns: Option<Vec<super::super::types::diagflow::CxTestCaseLastTestResultConversationTurn>>,
    /// Environment where the test was run. If not set, it indicates the draft environment.
    #[builder(into)]
    #[serde(rename = "environment")]
    pub r#environment: Option<String>,
    /// The unique identifier of the page.
    /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Whether the test case passed in the agent environment.
    /// * PASSED: The test passed.
    /// * FAILED: The test did not pass.
    /// Possible values are: `PASSED`, `FAILED`.
    #[builder(into)]
    #[serde(rename = "testResult")]
    pub r#test_result: Option<String>,
    /// The time that the test was run. A timestamp in RFC3339 text format.
    #[builder(into)]
    #[serde(rename = "testTime")]
    pub r#test_time: Option<String>,
}
