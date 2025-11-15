#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterStepHadoopJarStep {
    /// List of command line arguments passed to the JAR file's main function when executed.
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Option<Vec<String>>,
    /// Path to a JAR file run during the step.
    #[builder(into)]
    #[serde(rename = "jar")]
    pub r#jar: String,
    /// Name of the main class in the specified Java file. If not specified, the JAR file should specify a Main-Class in its manifest file.
    #[builder(into)]
    #[serde(rename = "mainClass")]
    pub r#main_class: Option<String>,
    /// Key-Value map of Java properties that are set when the step runs. You can use these properties to pass key value pairs to your main function.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Option<std::collections::HashMap<String, String>>,
}
