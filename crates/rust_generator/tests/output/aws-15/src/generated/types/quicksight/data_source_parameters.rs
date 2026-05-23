#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSourceParameters {
    /// Parameters for connecting to Amazon Elasticsearch.
    #[builder(into)]
    pub r#amazon_elasticsearch: Option<Box<super::super::types::quicksight::DataSourceParametersAmazonElasticsearch>>,
    /// Parameters for connecting to Athena.
    #[builder(into)]
    pub r#athena: Option<Box<super::super::types::quicksight::DataSourceParametersAthena>>,
    /// Parameters for connecting to Aurora MySQL.
    #[builder(into)]
    pub r#aurora: Option<Box<super::super::types::quicksight::DataSourceParametersAurora>>,
    /// Parameters for connecting to Aurora Postgresql.
    #[builder(into)]
    pub r#aurora_postgresql: Option<Box<super::super::types::quicksight::DataSourceParametersAuroraPostgresql>>,
    /// Parameters for connecting to AWS IOT Analytics.
    #[builder(into)]
    pub r#aws_iot_analytics: Option<Box<super::super::types::quicksight::DataSourceParametersAwsIotAnalytics>>,
    /// Parameters for connecting to Databricks.
    #[builder(into)]
    pub r#databricks: Option<Box<super::super::types::quicksight::DataSourceParametersDatabricks>>,
    /// Parameters for connecting to Jira.
    #[builder(into)]
    pub r#jira: Option<Box<super::super::types::quicksight::DataSourceParametersJira>>,
    /// Parameters for connecting to MariaDB.
    #[builder(into)]
    pub r#maria_db: Option<Box<super::super::types::quicksight::DataSourceParametersMariaDb>>,
    /// Parameters for connecting to MySQL.
    #[builder(into)]
    pub r#mysql: Option<Box<super::super::types::quicksight::DataSourceParametersMysql>>,
    /// Parameters for connecting to Oracle.
    #[builder(into)]
    pub r#oracle: Option<Box<super::super::types::quicksight::DataSourceParametersOracle>>,
    /// Parameters for connecting to Postgresql.
    #[builder(into)]
    pub r#postgresql: Option<Box<super::super::types::quicksight::DataSourceParametersPostgresql>>,
    /// Parameters for connecting to Presto.
    #[builder(into)]
    pub r#presto: Option<Box<super::super::types::quicksight::DataSourceParametersPresto>>,
    /// Parameters for connecting to RDS.
    #[builder(into)]
    pub r#rds: Option<Box<super::super::types::quicksight::DataSourceParametersRds>>,
    /// Parameters for connecting to Redshift.
    #[builder(into)]
    pub r#redshift: Option<Box<super::super::types::quicksight::DataSourceParametersRedshift>>,
    /// Parameters for connecting to S3.
    #[builder(into)]
    pub r#s_3: Option<Box<super::super::types::quicksight::DataSourceParametersS3>>,
    /// Parameters for connecting to ServiceNow.
    #[builder(into)]
    pub r#service_now: Option<Box<super::super::types::quicksight::DataSourceParametersServiceNow>>,
    /// Parameters for connecting to Snowflake.
    #[builder(into)]
    pub r#snowflake: Option<Box<super::super::types::quicksight::DataSourceParametersSnowflake>>,
    /// Parameters for connecting to Spark.
    #[builder(into)]
    pub r#spark: Option<Box<super::super::types::quicksight::DataSourceParametersSpark>>,
    /// Parameters for connecting to SQL Server.
    #[builder(into)]
    pub r#sql_server: Option<Box<super::super::types::quicksight::DataSourceParametersSqlServer>>,
    /// Parameters for connecting to Teradata.
    #[builder(into)]
    pub r#teradata: Option<Box<super::super::types::quicksight::DataSourceParametersTeradata>>,
    /// Parameters for connecting to Twitter.
    #[builder(into)]
    pub r#twitter: Option<Box<super::super::types::quicksight::DataSourceParametersTwitter>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSourceParameters {
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
                    "amazonElasticsearch",
                    &self.r#amazon_elasticsearch,
                ),
                to_pulumi_object_field(
                    "athena",
                    &self.r#athena,
                ),
                to_pulumi_object_field(
                    "aurora",
                    &self.r#aurora,
                ),
                to_pulumi_object_field(
                    "auroraPostgresql",
                    &self.r#aurora_postgresql,
                ),
                to_pulumi_object_field(
                    "awsIotAnalytics",
                    &self.r#aws_iot_analytics,
                ),
                to_pulumi_object_field(
                    "databricks",
                    &self.r#databricks,
                ),
                to_pulumi_object_field(
                    "jira",
                    &self.r#jira,
                ),
                to_pulumi_object_field(
                    "mariaDb",
                    &self.r#maria_db,
                ),
                to_pulumi_object_field(
                    "mysql",
                    &self.r#mysql,
                ),
                to_pulumi_object_field(
                    "oracle",
                    &self.r#oracle,
                ),
                to_pulumi_object_field(
                    "postgresql",
                    &self.r#postgresql,
                ),
                to_pulumi_object_field(
                    "presto",
                    &self.r#presto,
                ),
                to_pulumi_object_field(
                    "rds",
                    &self.r#rds,
                ),
                to_pulumi_object_field(
                    "redshift",
                    &self.r#redshift,
                ),
                to_pulumi_object_field(
                    "s3",
                    &self.r#s_3,
                ),
                to_pulumi_object_field(
                    "serviceNow",
                    &self.r#service_now,
                ),
                to_pulumi_object_field(
                    "snowflake",
                    &self.r#snowflake,
                ),
                to_pulumi_object_field(
                    "spark",
                    &self.r#spark,
                ),
                to_pulumi_object_field(
                    "sqlServer",
                    &self.r#sql_server,
                ),
                to_pulumi_object_field(
                    "teradata",
                    &self.r#teradata,
                ),
                to_pulumi_object_field(
                    "twitter",
                    &self.r#twitter,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSourceParameters {
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
                    r#amazon_elasticsearch: {
                        let field_value = match fields_map.get("amazonElasticsearch") {
                            Some(value) => value,
                            None => bail!("Missing field 'amazonElasticsearch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#athena: {
                        let field_value = match fields_map.get("athena") {
                            Some(value) => value,
                            None => bail!("Missing field 'athena' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#aurora: {
                        let field_value = match fields_map.get("aurora") {
                            Some(value) => value,
                            None => bail!("Missing field 'aurora' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#aurora_postgresql: {
                        let field_value = match fields_map.get("auroraPostgresql") {
                            Some(value) => value,
                            None => bail!("Missing field 'auroraPostgresql' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#aws_iot_analytics: {
                        let field_value = match fields_map.get("awsIotAnalytics") {
                            Some(value) => value,
                            None => bail!("Missing field 'awsIotAnalytics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#databricks: {
                        let field_value = match fields_map.get("databricks") {
                            Some(value) => value,
                            None => bail!("Missing field 'databricks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#jira: {
                        let field_value = match fields_map.get("jira") {
                            Some(value) => value,
                            None => bail!("Missing field 'jira' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maria_db: {
                        let field_value = match fields_map.get("mariaDb") {
                            Some(value) => value,
                            None => bail!("Missing field 'mariaDb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mysql: {
                        let field_value = match fields_map.get("mysql") {
                            Some(value) => value,
                            None => bail!("Missing field 'mysql' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oracle: {
                        let field_value = match fields_map.get("oracle") {
                            Some(value) => value,
                            None => bail!("Missing field 'oracle' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#postgresql: {
                        let field_value = match fields_map.get("postgresql") {
                            Some(value) => value,
                            None => bail!("Missing field 'postgresql' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#presto: {
                        let field_value = match fields_map.get("presto") {
                            Some(value) => value,
                            None => bail!("Missing field 'presto' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rds: {
                        let field_value = match fields_map.get("rds") {
                            Some(value) => value,
                            None => bail!("Missing field 'rds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redshift: {
                        let field_value = match fields_map.get("redshift") {
                            Some(value) => value,
                            None => bail!("Missing field 'redshift' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3: {
                        let field_value = match fields_map.get("s3") {
                            Some(value) => value,
                            None => bail!("Missing field 's3' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_now: {
                        let field_value = match fields_map.get("serviceNow") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceNow' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snowflake: {
                        let field_value = match fields_map.get("snowflake") {
                            Some(value) => value,
                            None => bail!("Missing field 'snowflake' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spark: {
                        let field_value = match fields_map.get("spark") {
                            Some(value) => value,
                            None => bail!("Missing field 'spark' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sql_server: {
                        let field_value = match fields_map.get("sqlServer") {
                            Some(value) => value,
                            None => bail!("Missing field 'sqlServer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#teradata: {
                        let field_value = match fields_map.get("teradata") {
                            Some(value) => value,
                            None => bail!("Missing field 'teradata' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#twitter: {
                        let field_value = match fields_map.get("twitter") {
                            Some(value) => value,
                            None => bail!("Missing field 'twitter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
