#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobSparkConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "archive_uris",
                    &self.r#archive_uris,
                ),
                to_pulumi_object_field(
                    "args",
                    &self.r#args,
                ),
                to_pulumi_object_field(
                    "file_uris",
                    &self.r#file_uris,
                ),
                to_pulumi_object_field(
                    "jar_file_uris",
                    &self.r#jar_file_uris,
                ),
                to_pulumi_object_field(
                    "logging_config",
                    &self.r#logging_config,
                ),
                to_pulumi_object_field(
                    "main_class",
                    &self.r#main_class,
                ),
                to_pulumi_object_field(
                    "main_jar_file_uri",
                    &self.r#main_jar_file_uri,
                ),
                to_pulumi_object_field(
                    "properties",
                    &self.r#properties,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobSparkConfig {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#archive_uris: {
                        let field_value = match fields_map.get("archive_uris") {
                            Some(value) => value,
                            None => bail!("Missing field 'archive_uris' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#args: {
                        let field_value = match fields_map.get("args") {
                            Some(value) => value,
                            None => bail!("Missing field 'args' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_uris: {
                        let field_value = match fields_map.get("file_uris") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_uris' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#jar_file_uris: {
                        let field_value = match fields_map.get("jar_file_uris") {
                            Some(value) => value,
                            None => bail!("Missing field 'jar_file_uris' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logging_config: {
                        let field_value = match fields_map.get("logging_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'logging_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#main_class: {
                        let field_value = match fields_map.get("main_class") {
                            Some(value) => value,
                            None => bail!("Missing field 'main_class' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#main_jar_file_uri: {
                        let field_value = match fields_map.get("main_jar_file_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'main_jar_file_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#properties: {
                        let field_value = match fields_map.get("properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
