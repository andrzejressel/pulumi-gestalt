#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSourceRelationalDatabaseConfigHttpEndpointConfig {
    /// AWS secret store ARN for database credentials.
    #[builder(into)]
    #[serde(rename = "awsSecretStoreArn")]
    pub r#aws_secret_store_arn: String,
    /// Logical database name.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: Option<String>,
    /// Amazon RDS cluster identifier.
    #[builder(into)]
    #[serde(rename = "dbClusterIdentifier")]
    pub r#db_cluster_identifier: String,
    /// AWS Region for RDS HTTP endpoint. Defaults to current region.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Option<String>,
    /// Logical schema name.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSourceRelationalDatabaseConfigHttpEndpointConfig {
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
                "aws_secret_store_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#aws_secret_store_arn,
                )
                .await,
            );
            map.insert(
                "database_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#database_name,
                )
                .await,
            );
            map.insert(
                "db_cluster_identifier".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#db_cluster_identifier,
                )
                .await,
            );
            map.insert(
                "region".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#region,
                )
                .await,
            );
            map.insert(
                "schema".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schema,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSourceRelationalDatabaseConfigHttpEndpointConfig {
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
                    r#aws_secret_store_arn: {
                        let field_value = match fields_map.get("aws_secret_store_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_secret_store_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_name: {
                        let field_value = match fields_map.get("database_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#db_cluster_identifier: {
                        let field_value = match fields_map.get("db_cluster_identifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'db_cluster_identifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#region: {
                        let field_value = match fields_map.get("region") {
                            Some(value) => value,
                            None => bail!("Missing field 'region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schema: {
                        let field_value = match fields_map.get("schema") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
