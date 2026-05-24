#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkflowTemplateJobPrestoJob {
    /// Presto client tags to attach to this query
    #[builder(into)]
    pub r#client_tags: Option<Vec<String>>,
    /// Whether to continue executing queries if a query fails. The default value is `false`. Setting to `true` can be useful when executing independent parallel queries.
    #[builder(into)]
    pub r#continue_on_failure: Option<bool>,
    /// The runtime log config for job execution.
    #[builder(into)]
    pub r#logging_config: Option<Box<super::super::types::dataproc::WorkflowTemplateJobPrestoJobLoggingConfig>>,
    /// The format in which query output will be displayed. See the Presto documentation for supported output formats
    #[builder(into)]
    pub r#output_format: Option<String>,
    /// A mapping of property names to values. Used to set Presto (https://prestodb.io/docs/current/sql/set-session.html) Equivalent to using the --session flag in the Presto CLI
    #[builder(into)]
    pub r#properties: Option<std::collections::BTreeMap<String, String>>,
    /// The HCFS URI of the script that contains SQL queries.
    #[builder(into)]
    pub r#query_file_uri: Option<String>,
    /// A list of queries.
    #[builder(into)]
    pub r#query_list: Option<Box<super::super::types::dataproc::WorkflowTemplateJobPrestoJobQueryList>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkflowTemplateJobPrestoJob {
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
                    "clientTags",
                    &self.r#client_tags,
                ),
                to_pulumi_object_field(
                    "continueOnFailure",
                    &self.r#continue_on_failure,
                ),
                to_pulumi_object_field(
                    "loggingConfig",
                    &self.r#logging_config,
                ),
                to_pulumi_object_field(
                    "outputFormat",
                    &self.r#output_format,
                ),
                to_pulumi_object_field(
                    "properties",
                    &self.r#properties,
                ),
                to_pulumi_object_field(
                    "queryFileUri",
                    &self.r#query_file_uri,
                ),
                to_pulumi_object_field(
                    "queryList",
                    &self.r#query_list,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkflowTemplateJobPrestoJob {
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
                        let field_value = match fields_map.get("clientTags") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientTags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#continue_on_failure: {
                        let field_value = match fields_map.get("continueOnFailure") {
                            Some(value) => value,
                            None => bail!("Missing field 'continueOnFailure' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logging_config: {
                        let field_value = match fields_map.get("loggingConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'loggingConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_format: {
                        let field_value = match fields_map.get("outputFormat") {
                            Some(value) => value,
                            None => bail!("Missing field 'outputFormat' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("queryFileUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'queryFileUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_list: {
                        let field_value = match fields_map.get("queryList") {
                            Some(value) => value,
                            None => bail!("Missing field 'queryList' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
