#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RoutineSparkOptions {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
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
                    "connection",
                    &self.r#connection,
                ),
                to_pulumi_object_field(
                    "container_image",
                    &self.r#container_image,
                ),
                to_pulumi_object_field(
                    "file_uris",
                    &self.r#file_uris,
                ),
                to_pulumi_object_field(
                    "jar_uris",
                    &self.r#jar_uris,
                ),
                to_pulumi_object_field(
                    "main_class",
                    &self.r#main_class,
                ),
                to_pulumi_object_field(
                    "main_file_uri",
                    &self.r#main_file_uri,
                ),
                to_pulumi_object_field(
                    "properties",
                    &self.r#properties,
                ),
                to_pulumi_object_field(
                    "py_file_uris",
                    &self.r#py_file_uris,
                ),
                to_pulumi_object_field(
                    "runtime_version",
                    &self.r#runtime_version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RoutineSparkOptions {
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
                    r#connection: {
                        let field_value = match fields_map.get("connection") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_image: {
                        let field_value = match fields_map.get("container_image") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_image' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#jar_uris: {
                        let field_value = match fields_map.get("jar_uris") {
                            Some(value) => value,
                            None => bail!("Missing field 'jar_uris' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#main_file_uri: {
                        let field_value = match fields_map.get("main_file_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'main_file_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#py_file_uris: {
                        let field_value = match fields_map.get("py_file_uris") {
                            Some(value) => value,
                            None => bail!("Missing field 'py_file_uris' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#runtime_version: {
                        let field_value = match fields_map.get("runtime_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'runtime_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
