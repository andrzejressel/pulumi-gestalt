#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipelineStageAction {
    /// A category defines what kind of action can be taken in the stage, and constrains the provider type for the action. Possible values are `Approval`, `Build`, `Deploy`, `Invoke`, `Source` and `Test`.
    #[builder(into)]
    #[serde(rename = "category")]
    pub r#category: String,
    /// A map of the action declaration's configuration. Configurations options for action types and providers can be found in the [Pipeline Structure Reference](http://docs.aws.amazon.com/codepipeline/latest/userguide/reference-pipeline-structure.html#action-requirements) and [Action Structure Reference](https://docs.aws.amazon.com/codepipeline/latest/userguide/action-reference.html) documentation. Note: The `DetectChanges` parameter (optional, default value is true) in the `configuration` section causes CodePipeline to automatically start your pipeline upon new commits. Please refer to AWS Documentation for more details: https://docs.aws.amazon.com/codepipeline/latest/userguide/action-reference-CodestarConnectionSource.html#action-reference-CodestarConnectionSource-config.
    #[builder(into)]
    #[serde(rename = "configuration")]
    pub r#configuration: Option<std::collections::HashMap<String, String>>,
    /// A list of artifact names to be worked on.
    #[builder(into)]
    #[serde(rename = "inputArtifacts")]
    pub r#input_artifacts: Option<Vec<String>>,
    /// The action declaration's name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The namespace all output variables will be accessed from.
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: Option<String>,
    /// A list of artifact names to output. Output artifact names must be unique within a pipeline.
    #[builder(into)]
    #[serde(rename = "outputArtifacts")]
    pub r#output_artifacts: Option<Vec<String>>,
    /// The creator of the action being called. Possible values are `AWS`, `Custom` and `ThirdParty`.
    #[builder(into)]
    #[serde(rename = "owner")]
    pub r#owner: String,
    /// The provider of the service being called by the action. Valid providers are determined by the action category. Provider names are listed in the [Action Structure Reference](https://docs.aws.amazon.com/codepipeline/latest/userguide/action-reference.html) documentation.
    #[builder(into)]
    #[serde(rename = "provider")]
    pub r#provider: String,
    /// The region in which to run the action.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Option<String>,
    /// The ARN of the IAM service role that will perform the declared action. This is assumed through the roleArn for the pipeline.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Option<String>,
    /// The order in which actions are run.
    #[builder(into)]
    #[serde(rename = "runOrder")]
    pub r#run_order: Option<i32>,
    #[builder(into)]
    #[serde(rename = "timeoutInMinutes")]
    pub r#timeout_in_minutes: Option<i32>,
    /// A string that identifies the action type.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
}
