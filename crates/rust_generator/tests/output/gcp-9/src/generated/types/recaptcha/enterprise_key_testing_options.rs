#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EnterpriseKeyTestingOptions {
    /// For challenge-based keys only (CHECKBOX, INVISIBLE), all challenge requests for this site will return nocaptcha if NOCAPTCHA, or an unsolvable challenge if UNSOLVABLE_CHALLENGE. Possible values: TESTING_CHALLENGE_UNSPECIFIED, NOCAPTCHA, UNSOLVABLE_CHALLENGE
    #[builder(into)]
    #[serde(rename = "testingChallenge")]
    pub r#testing_challenge: Option<String>,
    /// All assessments for this Key will return this score. Must be between 0 (likely not legitimate) and 1 (likely legitimate) inclusive.
    #[builder(into)]
    #[serde(rename = "testingScore")]
    pub r#testing_score: Option<f64>,
}
