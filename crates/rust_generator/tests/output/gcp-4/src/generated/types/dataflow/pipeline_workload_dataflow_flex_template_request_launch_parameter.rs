#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipelineWorkloadDataflowFlexTemplateRequestLaunchParameter {
    /// Cloud Storage path to a file with a JSON-serialized ContainerSpec as content.
    #[builder(into)]
    #[serde(rename = "containerSpecGcsPath")]
    pub r#container_spec_gcs_path: Option<String>,
    /// The runtime environment for the Flex Template job.
    /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#FlexTemplateRuntimeEnvironment
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "environment")]
    pub r#environment: Option<Box<super::super::types::dataflow::PipelineWorkloadDataflowFlexTemplateRequestLaunchParameterEnvironment>>,
    /// The job name to use for the created job. For an update job request, the job name should be the same as the existing running job.
    #[builder(into)]
    #[serde(rename = "jobName")]
    pub r#job_name: String,
    /// Launch options for this Flex Template job. This is a common set of options across languages and templates. This should not be used to pass job parameters.
    /// 'An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.'
    #[builder(into)]
    #[serde(rename = "launchOptions")]
    pub r#launch_options: Option<std::collections::HashMap<String, String>>,
    /// 'The parameters for the Flex Template. Example: {"numWorkers":"5"}'
    /// 'An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.'
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<std::collections::HashMap<String, String>>,
    /// 'Use this to pass transform name mappings for streaming update jobs. Example: {"oldTransformName":"newTransformName",...}'
    /// 'An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.'
    #[builder(into)]
    #[serde(rename = "transformNameMappings")]
    pub r#transform_name_mappings: Option<std::collections::HashMap<String, String>>,
    /// Set this to true if you are sending a request to update a running streaming job. When set, the job name should be the same as the running job.
    #[builder(into)]
    #[serde(rename = "update")]
    pub r#update: Option<bool>,
}
