#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobSparkConfig {
    /// HCFS URIs of archives to be extracted in the working directory of .jar, .tar, .tar.gz, .tgz, and .zip.
    #[builder(into)]
    #[serde(rename = "archiveUris")]
    pub r#archive_uris: Option<Vec<String>>,
    /// The arguments to pass to the driver.
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Option<Vec<String>>,
    /// HCFS URIs of files to be copied to the working directory of Spark drivers and distributed tasks. Useful for naively parallel tasks.
    #[builder(into)]
    #[serde(rename = "fileUris")]
    pub r#file_uris: Option<Vec<String>>,
    /// HCFS URIs of jar files to add to the CLASSPATHs of the Spark driver and tasks.
    #[builder(into)]
    #[serde(rename = "jarFileUris")]
    pub r#jar_file_uris: Option<Vec<String>>,
    /// The runtime logging config of the job
    #[builder(into)]
    #[serde(rename = "loggingConfig")]
    pub r#logging_config: Option<Box<super::super::types::dataproc::JobSparkConfigLoggingConfig>>,
    /// The class containing the main method of the driver. Must be in a
    /// provided jar or jar that is already on the classpath. Conflicts with `main_jar_file_uri`
    #[builder(into)]
    #[serde(rename = "mainClass")]
    pub r#main_class: Option<String>,
    /// The HCFS URI of jar file containing
    /// the driver jar. Conflicts with `main_class`
    #[builder(into)]
    #[serde(rename = "mainJarFileUri")]
    pub r#main_jar_file_uri: Option<String>,
    /// A mapping of property names to values, used to configure Spark. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in `/etc/spark/conf/spark-defaults.conf` and classes in user code.
    /// 
    /// * `logging_config.driver_log_levels`- (Required) The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Option<std::collections::HashMap<String, String>>,
}
