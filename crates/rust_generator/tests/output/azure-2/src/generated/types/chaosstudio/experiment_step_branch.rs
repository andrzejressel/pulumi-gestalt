#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ExperimentStepBranch {
    /// One or more `actions` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Vec<super::super::types::chaosstudio::ExperimentStepBranchAction>,
    /// The name of the branch.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
