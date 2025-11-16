#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UrlMapTest {
    /// Description of this test case.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Host portion of the URL.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: String,
    /// Path portion of the URL.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
    /// The backend service or backend bucket link that should be matched by this test.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: String,
}
