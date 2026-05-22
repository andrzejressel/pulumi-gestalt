#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GdcSparkApplicationSparkApplicationConfig {
    /// HCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: `.jar`, `.tar`, `.tar.gz`, `.tgz`, and `.zip`.
    #[builder(into)]
    #[serde(rename = "archiveUris")]
    pub r#archive_uris: Option<Vec<String>>,
    /// The arguments to pass to the driver. Do not include arguments that can be set as application properties, such as `--conf`, since a collision can occur that causes an incorrect application submission.
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
    /// The name of the driver main class. The jar file that contains the class must be in the classpath or specified in `jar_file_uris`.
    #[builder(into)]
    #[serde(rename = "mainClass")]
    pub r#main_class: Option<String>,
    /// The HCFS URI of the jar file that contains the main class.
    #[builder(into)]
    #[serde(rename = "mainJarFileUri")]
    pub r#main_jar_file_uri: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GdcSparkApplicationSparkApplicationConfig {
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
                    "archiveUris",
                    &self.r#archive_uris,
                ),
                to_pulumi_object_field(
                    "args",
                    &self.r#args,
                ),
                to_pulumi_object_field(
                    "fileUris",
                    &self.r#file_uris,
                ),
                to_pulumi_object_field(
                    "jarFileUris",
                    &self.r#jar_file_uris,
                ),
                to_pulumi_object_field(
                    "mainClass",
                    &self.r#main_class,
                ),
                to_pulumi_object_field(
                    "mainJarFileUri",
                    &self.r#main_jar_file_uri,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GdcSparkApplicationSparkApplicationConfig {
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
                        let field_value = match fields_map.get("archiveUris") {
                            Some(value) => value,
                            None => bail!("Missing field 'archiveUris' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("fileUris") {
                            Some(value) => value,
                            None => bail!("Missing field 'fileUris' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#jar_file_uris: {
                        let field_value = match fields_map.get("jarFileUris") {
                            Some(value) => value,
                            None => bail!("Missing field 'jarFileUris' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#main_class: {
                        let field_value = match fields_map.get("mainClass") {
                            Some(value) => value,
                            None => bail!("Missing field 'mainClass' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#main_jar_file_uri: {
                        let field_value = match fields_map.get("mainJarFileUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'mainJarFileUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
