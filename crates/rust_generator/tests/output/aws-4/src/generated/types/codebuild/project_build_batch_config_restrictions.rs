#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProjectBuildBatchConfigRestrictions {
    /// An array of strings that specify the compute types that are allowed for the batch build. See [Build environment compute types](https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html) in the AWS CodeBuild User Guide for these values.
    #[builder(into)]
    #[serde(rename = "computeTypesAlloweds")]
    pub r#compute_types_alloweds: Option<Vec<String>>,
    /// Specifies the maximum number of builds allowed.
    #[builder(into)]
    #[serde(rename = "maximumBuildsAllowed")]
    pub r#maximum_builds_allowed: Option<i32>,
}
