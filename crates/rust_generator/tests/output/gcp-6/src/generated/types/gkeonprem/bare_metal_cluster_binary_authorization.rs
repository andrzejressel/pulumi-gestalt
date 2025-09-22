#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BareMetalClusterBinaryAuthorization {
    /// Mode of operation for binauthz policy evaluation. If unspecified,
    /// defaults to DISABLED.
    /// Possible values are: `DISABLED`, `PROJECT_SINGLETON_POLICY_ENFORCE`.
    #[builder(into)]
    #[serde(rename = "evaluationMode")]
    pub r#evaluation_mode: Option<String>,
}
