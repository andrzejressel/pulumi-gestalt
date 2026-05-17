#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BatchSparkSqlBatch {
    /// HCFS URIs of jar files to be added to the Spark CLASSPATH.
    #[builder(into)]
    #[serde(rename = "jarFileUris")]
    pub r#jar_file_uris: Option<Vec<String>>,
    /// The HCFS URI of the script that contains Spark SQL queries to execute.
    #[builder(into)]
    #[serde(rename = "queryFileUri")]
    pub r#query_file_uri: Option<String>,
    /// Mapping of query variable names to values (equivalent to the Spark SQL command: SET name="value";).
    #[builder(into)]
    #[serde(rename = "queryVariables")]
    pub r#query_variables: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BatchSparkSqlBatch {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "jar_file_uris".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#jar_file_uris,
                )
                .await,
            );
            map.insert(
                "query_file_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_file_uri,
                )
                .await,
            );
            map.insert(
                "query_variables".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_variables,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BatchSparkSqlBatch {
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
                    r#jar_file_uris: {
                        let field_value = match fields_map.get("jar_file_uris") {
                            Some(value) => value,
                            None => bail!("Missing field 'jar_file_uris' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#query_variables: {
                        let field_value = match fields_map.get("query_variables") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_variables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
