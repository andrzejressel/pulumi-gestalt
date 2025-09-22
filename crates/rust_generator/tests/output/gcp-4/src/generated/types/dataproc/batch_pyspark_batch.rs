#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BatchPysparkBatch {
    /// HCFS URIs of archives to be extracted into the working directory of each executor.
    /// Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip.
    #[builder(into)]
    #[serde(rename = "archiveUris")]
    pub r#archive_uris: Option<Vec<String>>,
    /// The arguments to pass to the driver. Do not include arguments that can be set as batch
    /// properties, such as --conf, since a collision can occur that causes an incorrect batch submission.
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Option<Vec<String>>,
    /// HCFS URIs of files to be placed in the working directory of each executor.
    #[builder(into)]
    #[serde(rename = "fileUris")]
    pub r#file_uris: Option<Vec<String>>,
    /// HCFS URIs of jar files to add to the classpath of the Spark driver and tasks.
    #[builder(into)]
    #[serde(rename = "jarFileUris")]
    pub r#jar_file_uris: Option<Vec<String>>,
    /// The HCFS URI of the main Python file to use as the Spark driver. Must be a .py file.
    #[builder(into)]
    #[serde(rename = "mainPythonFileUri")]
    pub r#main_python_file_uri: Option<String>,
    /// HCFS file URIs of Python files to pass to the PySpark framework.
    /// Supported file types: .py, .egg, and .zip.
    #[builder(into)]
    #[serde(rename = "pythonFileUris")]
    pub r#python_file_uris: Option<Vec<String>>,
}
