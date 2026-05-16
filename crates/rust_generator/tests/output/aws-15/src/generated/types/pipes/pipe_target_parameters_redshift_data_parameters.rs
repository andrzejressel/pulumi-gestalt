#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipeTargetParametersRedshiftDataParameters {
    /// The name of the database. Required when authenticating using temporary credentials.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: String,
    /// The database user name. Required when authenticating using temporary credentials.
    #[builder(into)]
    #[serde(rename = "dbUser")]
    pub r#db_user: Option<String>,
    /// The name or ARN of the secret that enables access to the database. Required when authenticating using Secrets Manager.
    #[builder(into)]
    #[serde(rename = "secretManagerArn")]
    pub r#secret_manager_arn: Option<String>,
    /// List of SQL statements text to run, each of maximum length of 100,000.
    #[builder(into)]
    #[serde(rename = "sqls")]
    pub r#sqls: Vec<String>,
    /// The name of the SQL statement. You can name the SQL statement when you create it to identify the query.
    #[builder(into)]
    #[serde(rename = "statementName")]
    pub r#statement_name: Option<String>,
    /// Indicates whether to send an event back to EventBridge after the SQL statement runs.
    #[builder(into)]
    #[serde(rename = "withEvent")]
    pub r#with_event: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipeTargetParametersRedshiftDataParameters {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("database".to_string(), self.r#database.to_pulumi_value().await);
            map.insert("db_user".to_string(), self.r#db_user.to_pulumi_value().await);
            map.insert("secret_manager_arn".to_string(), self.r#secret_manager_arn.to_pulumi_value().await);
            map.insert("sqls".to_string(), self.r#sqls.to_pulumi_value().await);
            map.insert("statement_name".to_string(), self.r#statement_name.to_pulumi_value().await);
            map.insert("with_event".to_string(), self.r#with_event.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipeTargetParametersRedshiftDataParameters {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#database: {
                        let field_value = match fields_map.get("database") {
                            Some(value) => value,
                            None => bail!("Missing field 'database' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#db_user: {
                        let field_value = match fields_map.get("db_user") {
                            Some(value) => value,
                            None => bail!("Missing field 'db_user' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#secret_manager_arn: {
                        let field_value = match fields_map.get("secret_manager_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_manager_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sqls: {
                        let field_value = match fields_map.get("sqls") {
                            Some(value) => value,
                            None => bail!("Missing field 'sqls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#statement_name: {
                        let field_value = match fields_map.get("statement_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'statement_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#with_event: {
                        let field_value = match fields_map.get("with_event") {
                            Some(value) => value,
                            None => bail!("Missing field 'with_event' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
