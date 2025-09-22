#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RoutineSparkOptions {
    /// Archive files to be extracted into the working directory of each executor. For more information about Apache Spark, see Apache Spark.
    #[builder(into)]
    #[serde(rename = "archiveUris")]
    pub r#archive_uris: Option<Vec<String>>,
    /// Fully qualified name of the user-provided Spark connection object.
    /// Format: "projects/{projectId}/locations/{locationId}/connections/{connectionId}"
    #[builder(into)]
    #[serde(rename = "connection")]
    pub r#connection: Option<String>,
    /// Custom container image for the runtime environment.
    #[builder(into)]
    #[serde(rename = "containerImage")]
    pub r#container_image: Option<String>,
    /// Files to be placed in the working directory of each executor. For more information about Apache Spark, see Apache Spark.
    #[builder(into)]
    #[serde(rename = "fileUris")]
    pub r#file_uris: Option<Vec<String>>,
    /// JARs to include on the driver and executor CLASSPATH. For more information about Apache Spark, see Apache Spark.
    #[builder(into)]
    #[serde(rename = "jarUris")]
    pub r#jar_uris: Option<Vec<String>>,
    /// The fully qualified name of a class in jarUris, for example, com.example.wordcount.
    /// Exactly one of mainClass and main_jar_uri field should be set for Java/Scala language type.
    #[builder(into)]
    #[serde(rename = "mainClass")]
    pub r#main_class: Option<String>,
    /// The main file/jar URI of the Spark application.
    /// Exactly one of the definitionBody field and the mainFileUri field must be set for Python.
    /// Exactly one of mainClass and mainFileUri field should be set for Java/Scala language type.
    #[builder(into)]
    #[serde(rename = "mainFileUri")]
    pub r#main_file_uri: Option<String>,
    /// Configuration properties as a set of key/value pairs, which will be passed on to the Spark application.
    /// For more information, see Apache Spark and the procedure option list.
    /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Option<std::collections::HashMap<String, String>>,
    /// Python files to be placed on the PYTHONPATH for PySpark application. Supported file types: .py, .egg, and .zip. For more information about Apache Spark, see Apache Spark.
    #[builder(into)]
    #[serde(rename = "pyFileUris")]
    pub r#py_file_uris: Option<Vec<String>>,
    /// Runtime version. If not specified, the default runtime version is used.
    #[builder(into)]
    #[serde(rename = "runtimeVersion")]
    pub r#runtime_version: Option<String>,
}
