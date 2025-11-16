#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LayeredType {
    /// The answer to the question
    #[builder(into)]
    #[serde(rename = "answer")]
    pub r#answer: Option<f64>,
    #[builder(into)]
    #[serde(rename = "other")]
    pub r#other: Box<super::types::HelmReleaseSettings>,
    /// Test how plain types interact
    #[builder(into)]
    #[serde(rename = "plainOther")]
    pub r#plain_other: Option<Box<super::types::HelmReleaseSettings>>,
    /// The question already answered
    #[builder(into)]
    #[serde(rename = "question")]
    pub r#question: Option<String>,
    #[builder(into)]
    #[serde(rename = "recursive")]
    pub r#recursive: Option<Box<super::types::LayeredType>>,
    /// To ask and answer
    #[builder(into)]
    #[serde(rename = "thinker")]
    pub r#thinker: String,
}
