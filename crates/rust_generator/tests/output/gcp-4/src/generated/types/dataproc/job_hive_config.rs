#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobHiveConfig {
    /// Whether to continue executing queries if a query fails. The default value is false. Setting to true can be useful when executing independent parallel queries. Defaults to false.
    #[builder(into)]
    pub r#continue_on_failure: Option<bool>,
    /// HCFS URIs of jar files to add to the CLASSPATH of the Hive server and Hadoop MapReduce (MR) tasks. Can contain Hive SerDes and UDFs.
    #[builder(into)]
    pub r#jar_file_uris: Option<Vec<String>>,
    /// A mapping of property names and values, used to configure Hive. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in `/etc/hadoop/conf/*-site.xml`, `/etc/hive/conf/hive-site.xml`, and classes in user code..
    #[builder(into)]
    pub r#properties: Option<std::collections::HashMap<String, String>>,
    /// HCFS URI of file containing Hive script to execute as the job.
    /// Conflicts with `query_list`
    #[builder(into)]
    pub r#query_file_uri: Option<String>,
    /// The list of Hive queries or statements to execute as part of the job.
    /// Conflicts with `query_file_uri`
    #[builder(into)]
    pub r#query_lists: Option<Vec<String>>,
    /// Mapping of query variable names to values (equivalent to the Hive command: `SET name="value";`).
    #[builder(into)]
    pub r#script_variables: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobHiveConfig {
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
                    "continueOnFailure",
                    &self.r#continue_on_failure,
                ),
                to_pulumi_object_field(
                    "jarFileUris",
                    &self.r#jar_file_uris,
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
                    "queryLists",
                    &self.r#query_lists,
                ),
                to_pulumi_object_field(
                    "scriptVariables",
                    &self.r#script_variables,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobHiveConfig {
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
                    r#continue_on_failure: {
                        let field_value = match fields_map.get("continueOnFailure") {
                            Some(value) => value,
                            None => bail!("Missing field 'continueOnFailure' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#query_lists: {
                        let field_value = match fields_map.get("queryLists") {
                            Some(value) => value,
                            None => bail!("Missing field 'queryLists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#script_variables: {
                        let field_value = match fields_map.get("scriptVariables") {
                            Some(value) => value,
                            None => bail!("Missing field 'scriptVariables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
