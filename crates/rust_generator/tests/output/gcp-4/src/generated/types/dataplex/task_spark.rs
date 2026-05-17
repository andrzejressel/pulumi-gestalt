#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TaskSpark {
    /// Cloud Storage URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip.
    #[builder(into)]
    #[serde(rename = "archiveUris")]
    pub r#archive_uris: Option<Vec<String>>,
    /// Cloud Storage URIs of files to be placed in the working directory of each executor.
    #[builder(into)]
    #[serde(rename = "fileUris")]
    pub r#file_uris: Option<Vec<String>>,
    /// Infrastructure specification for the execution.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "infrastructureSpec")]
    pub r#infrastructure_spec: Option<Box<super::super::types::dataplex::TaskSparkInfrastructureSpec>>,
    /// The name of the driver's main class. The jar file that contains the class must be in the default CLASSPATH or specified in jar_file_uris. The execution args are passed in as a sequence of named process arguments (--key=value).
    #[builder(into)]
    #[serde(rename = "mainClass")]
    pub r#main_class: Option<String>,
    /// The Cloud Storage URI of the jar file that contains the main class. The execution args are passed in as a sequence of named process arguments (--key=value).
    #[builder(into)]
    #[serde(rename = "mainJarFileUri")]
    pub r#main_jar_file_uri: Option<String>,
    /// The Gcloud Storage URI of the main Python file to use as the driver. Must be a .py file. The execution args are passed in as a sequence of named process arguments (--key=value).
    #[builder(into)]
    #[serde(rename = "pythonScriptFile")]
    pub r#python_script_file: Option<String>,
    /// The query text. The execution args are used to declare a set of script variables (set key='value';).
    #[builder(into)]
    #[serde(rename = "sqlScript")]
    pub r#sql_script: Option<String>,
    /// A reference to a query file. This can be the Cloud Storage URI of the query file or it can the path to a SqlScript Content. The execution args are used to declare a set of script variables (set key='value';).
    #[builder(into)]
    #[serde(rename = "sqlScriptFile")]
    pub r#sql_script_file: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TaskSpark {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "archive_uris".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#archive_uris,
                )
                .await,
            );
            map.insert(
                "file_uris".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file_uris,
                )
                .await,
            );
            map.insert(
                "infrastructure_spec".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#infrastructure_spec,
                )
                .await,
            );
            map.insert(
                "main_class".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#main_class,
                )
                .await,
            );
            map.insert(
                "main_jar_file_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#main_jar_file_uri,
                )
                .await,
            );
            map.insert(
                "python_script_file".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#python_script_file,
                )
                .await,
            );
            map.insert(
                "sql_script".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sql_script,
                )
                .await,
            );
            map.insert(
                "sql_script_file".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sql_script_file,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TaskSpark {
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
                    r#file_uris: {
                        let field_value = match fields_map.get("file_uris") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_uris' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#infrastructure_spec: {
                        let field_value = match fields_map.get("infrastructure_spec") {
                            Some(value) => value,
                            None => bail!("Missing field 'infrastructure_spec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#python_script_file: {
                        let field_value = match fields_map.get("python_script_file") {
                            Some(value) => value,
                            None => bail!("Missing field 'python_script_file' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sql_script: {
                        let field_value = match fields_map.get("sql_script") {
                            Some(value) => value,
                            None => bail!("Missing field 'sql_script' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sql_script_file: {
                        let field_value = match fields_map.get("sql_script_file") {
                            Some(value) => value,
                            None => bail!("Missing field 'sql_script_file' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
