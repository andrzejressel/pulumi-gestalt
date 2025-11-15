#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AttachedClusterBinaryAuthorization {
    /// Configure Binary Authorization evaluation mode.
    /// Possible values are: `DISABLED`, `PROJECT_SINGLETON_POLICY_ENFORCE`.
    #[builder(into)]
    #[serde(rename = "evaluationMode")]
    pub r#evaluation_mode: Option<String>,
}
