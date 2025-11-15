#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTargetGroupStickiness {
    #[builder(into)]
    #[serde(rename = "cookieDuration")]
    pub r#cookie_duration: i32,
    #[builder(into)]
    #[serde(rename = "cookieName")]
    pub r#cookie_name: String,
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
