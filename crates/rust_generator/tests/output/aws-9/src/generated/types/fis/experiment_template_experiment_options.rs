#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ExperimentTemplateExperimentOptions {
    /// Specifies the account targeting setting for experiment options. Supports `single-account` and `multi-account`.
    #[builder(into)]
    #[serde(rename = "accountTargeting")]
    pub r#account_targeting: Option<String>,
    /// Specifies the empty target resolution mode for experiment options. Supports `fail` and `skip`.
    #[builder(into)]
    #[serde(rename = "emptyTargetResolutionMode")]
    pub r#empty_target_resolution_mode: Option<String>,
}
