#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TaskExecutionSpec {
    /// The arguments to pass to the task. The args can use placeholders of the format ${placeholder} as part of key/value string. These will be interpolated before passing the args to the driver. Currently supported placeholders: - ${taskId} - ${job_time} To pass positional args, set the key as TASK_ARGS. The value should be a comma-separated string of all the positional arguments. To use a delimiter other than comma, refer to https://cloud.google.com/sdk/gcloud/reference/topic/escaping. In case of other keys being present in the args, then TASK_ARGS will be passed as the last argument. An object containing a list of 'key': value pairs. Example: { 'name': 'wrench', 'mass': '1.3kg', 'count': '3' }.
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Option<std::collections::HashMap<String, String>>,
    /// The Cloud KMS key to use for encryption, of the form: projects/{project_number}/locations/{locationId}/keyRings/{key-ring-name}/cryptoKeys/{key-name}.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "kmsKey")]
    pub r#kms_key: Option<String>,
    /// The maximum duration after which the job execution is expired. A duration in seconds with up to nine fractional digits, ending with 's'. Example: '3.5s'.
    #[builder(into)]
    #[serde(rename = "maxJobExecutionLifetime")]
    pub r#max_job_execution_lifetime: Option<String>,
    /// The ID of the project in which the resource belongs.
    /// If it is not provided, the provider project is used.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: Option<String>,
    /// Service account to use to execute a task. If not provided, the default Compute service account for the project is used.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: String,
}
