#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipeTargetParametersRedshiftDataParameters {
    /// The name of the database. Required when authenticating using temporary credentials.
    #[builder(into)]
    pub r#database: String,
    /// The database user name. Required when authenticating using temporary credentials.
    #[builder(into)]
    pub r#db_user: Option<String>,
    /// The name or ARN of the secret that enables access to the database. Required when authenticating using Secrets Manager.
    #[builder(into)]
    pub r#secret_manager_arn: Option<String>,
    /// List of SQL statements text to run, each of maximum length of 100,000.
    #[builder(into)]
    pub r#sqls: Vec<String>,
    /// The name of the SQL statement. You can name the SQL statement when you create it to identify the query.
    #[builder(into)]
    pub r#statement_name: Option<String>,
    /// Indicates whether to send an event back to EventBridge after the SQL statement runs.
    #[builder(into)]
    pub r#with_event: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipeTargetParametersRedshiftDataParameters {
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
                    "database",
                    &self.r#database,
                ),
                to_pulumi_object_field(
                    "dbUser",
                    &self.r#db_user,
                ),
                to_pulumi_object_field(
                    "secretManagerArn",
                    &self.r#secret_manager_arn,
                ),
                to_pulumi_object_field(
                    "sqls",
                    &self.r#sqls,
                ),
                to_pulumi_object_field(
                    "statementName",
                    &self.r#statement_name,
                ),
                to_pulumi_object_field(
                    "withEvent",
                    &self.r#with_event,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipeTargetParametersRedshiftDataParameters {
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
                    r#database: {
                        let field_value = match fields_map.get("database") {
                            Some(value) => value,
                            None => bail!("Missing field 'database' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#db_user: {
                        let field_value = match fields_map.get("dbUser") {
                            Some(value) => value,
                            None => bail!("Missing field 'dbUser' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_manager_arn: {
                        let field_value = match fields_map.get("secretManagerArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'secretManagerArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sqls: {
                        let field_value = match fields_map.get("sqls") {
                            Some(value) => value,
                            None => bail!("Missing field 'sqls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#statement_name: {
                        let field_value = match fields_map.get("statementName") {
                            Some(value) => value,
                            None => bail!("Missing field 'statementName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#with_event: {
                        let field_value = match fields_map.get("withEvent") {
                            Some(value) => value,
                            None => bail!("Missing field 'withEvent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
