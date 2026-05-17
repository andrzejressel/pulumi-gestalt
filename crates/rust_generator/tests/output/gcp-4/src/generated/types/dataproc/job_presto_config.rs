#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobPrestoConfig {
    /// Presto client tags to attach to this query.
    #[builder(into)]
    #[serde(rename = "clientTags")]
    pub r#client_tags: Option<Vec<String>>,
    /// Whether to continue executing queries if a query fails. Setting to true can be useful when executing independent parallel queries. Defaults to false.
    #[builder(into)]
    #[serde(rename = "continueOnFailure")]
    pub r#continue_on_failure: Option<bool>,
    /// The runtime logging config of the job
    #[builder(into)]
    #[serde(rename = "loggingConfig")]
    pub r#logging_config: Option<Box<super::super::types::dataproc::JobPrestoConfigLoggingConfig>>,
    /// The format in which query output will be displayed. See the Presto documentation for supported output formats.
    /// 
    /// * `logging_config.driver_log_levels`- (Required) The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'
    #[builder(into)]
    #[serde(rename = "outputFormat")]
    pub r#output_format: Option<String>,
    /// A mapping of property names to values. Used to set Presto session properties Equivalent to using the --session flag in the Presto CLI.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Option<std::collections::HashMap<String, String>>,
    /// The HCFS URI of the script that contains SQL queries.
    /// Conflicts with `query_list`
    #[builder(into)]
    #[serde(rename = "queryFileUri")]
    pub r#query_file_uri: Option<String>,
    /// The list of SQL queries or statements to execute as part of the job.
    /// Conflicts with `query_file_uri`
    #[builder(into)]
    #[serde(rename = "queryLists")]
    pub r#query_lists: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobPrestoConfig {
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
                    "client_tags",
                    &self.r#client_tags,
                ),
                to_pulumi_object_field(
                    "continue_on_failure",
                    &self.r#continue_on_failure,
                ),
                to_pulumi_object_field(
                    "logging_config",
                    &self.r#logging_config,
                ),
                to_pulumi_object_field(
                    "output_format",
                    &self.r#output_format,
                ),
                to_pulumi_object_field(
                    "properties",
                    &self.r#properties,
                ),
                to_pulumi_object_field(
                    "query_file_uri",
                    &self.r#query_file_uri,
                ),
                to_pulumi_object_field(
                    "query_lists",
                    &self.r#query_lists,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobPrestoConfig {
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
                    r#client_tags: {
                        let field_value = match fields_map.get("client_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#continue_on_failure: {
                        let field_value = match fields_map.get("continue_on_failure") {
                            Some(value) => value,
                            None => bail!("Missing field 'continue_on_failure' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#output_format: {
                        let field_value = match fields_map.get("output_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#query_file_uri: {
                        let field_value = match fields_map.get("query_file_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_file_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_lists: {
                        let field_value = match fields_map.get("query_lists") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_lists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
