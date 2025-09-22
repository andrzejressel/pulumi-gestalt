#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceParameters {
    /// Parameters for connecting to Amazon Elasticsearch.
    #[builder(into)]
    #[serde(rename = "amazonElasticsearch")]
    pub r#amazon_elasticsearch: Option<Box<super::super::types::quicksight::DataSourceParametersAmazonElasticsearch>>,
    /// Parameters for connecting to Athena.
    #[builder(into)]
    #[serde(rename = "athena")]
    pub r#athena: Option<Box<super::super::types::quicksight::DataSourceParametersAthena>>,
    /// Parameters for connecting to Aurora MySQL.
    #[builder(into)]
    #[serde(rename = "aurora")]
    pub r#aurora: Option<Box<super::super::types::quicksight::DataSourceParametersAurora>>,
    /// Parameters for connecting to Aurora Postgresql.
    #[builder(into)]
    #[serde(rename = "auroraPostgresql")]
    pub r#aurora_postgresql: Option<Box<super::super::types::quicksight::DataSourceParametersAuroraPostgresql>>,
    /// Parameters for connecting to AWS IOT Analytics.
    #[builder(into)]
    #[serde(rename = "awsIotAnalytics")]
    pub r#aws_iot_analytics: Option<Box<super::super::types::quicksight::DataSourceParametersAwsIotAnalytics>>,
    /// Parameters for connecting to Databricks.
    #[builder(into)]
    #[serde(rename = "databricks")]
    pub r#databricks: Option<Box<super::super::types::quicksight::DataSourceParametersDatabricks>>,
    /// Parameters for connecting to Jira.
    #[builder(into)]
    #[serde(rename = "jira")]
    pub r#jira: Option<Box<super::super::types::quicksight::DataSourceParametersJira>>,
    /// Parameters for connecting to MariaDB.
    #[builder(into)]
    #[serde(rename = "mariaDb")]
    pub r#maria_db: Option<Box<super::super::types::quicksight::DataSourceParametersMariaDb>>,
    /// Parameters for connecting to MySQL.
    #[builder(into)]
    #[serde(rename = "mysql")]
    pub r#mysql: Option<Box<super::super::types::quicksight::DataSourceParametersMysql>>,
    /// Parameters for connecting to Oracle.
    #[builder(into)]
    #[serde(rename = "oracle")]
    pub r#oracle: Option<Box<super::super::types::quicksight::DataSourceParametersOracle>>,
    /// Parameters for connecting to Postgresql.
    #[builder(into)]
    #[serde(rename = "postgresql")]
    pub r#postgresql: Option<Box<super::super::types::quicksight::DataSourceParametersPostgresql>>,
    /// Parameters for connecting to Presto.
    #[builder(into)]
    #[serde(rename = "presto")]
    pub r#presto: Option<Box<super::super::types::quicksight::DataSourceParametersPresto>>,
    /// Parameters for connecting to RDS.
    #[builder(into)]
    #[serde(rename = "rds")]
    pub r#rds: Option<Box<super::super::types::quicksight::DataSourceParametersRds>>,
    /// Parameters for connecting to Redshift.
    #[builder(into)]
    #[serde(rename = "redshift")]
    pub r#redshift: Option<Box<super::super::types::quicksight::DataSourceParametersRedshift>>,
    /// Parameters for connecting to S3.
    #[builder(into)]
    #[serde(rename = "s3")]
    pub r#s_3: Option<Box<super::super::types::quicksight::DataSourceParametersS3>>,
    /// Parameters for connecting to ServiceNow.
    #[builder(into)]
    #[serde(rename = "serviceNow")]
    pub r#service_now: Option<Box<super::super::types::quicksight::DataSourceParametersServiceNow>>,
    /// Parameters for connecting to Snowflake.
    #[builder(into)]
    #[serde(rename = "snowflake")]
    pub r#snowflake: Option<Box<super::super::types::quicksight::DataSourceParametersSnowflake>>,
    /// Parameters for connecting to Spark.
    #[builder(into)]
    #[serde(rename = "spark")]
    pub r#spark: Option<Box<super::super::types::quicksight::DataSourceParametersSpark>>,
    /// Parameters for connecting to SQL Server.
    #[builder(into)]
    #[serde(rename = "sqlServer")]
    pub r#sql_server: Option<Box<super::super::types::quicksight::DataSourceParametersSqlServer>>,
    /// Parameters for connecting to Teradata.
    #[builder(into)]
    #[serde(rename = "teradata")]
    pub r#teradata: Option<Box<super::super::types::quicksight::DataSourceParametersTeradata>>,
    /// Parameters for connecting to Twitter.
    #[builder(into)]
    #[serde(rename = "twitter")]
    pub r#twitter: Option<Box<super::super::types::quicksight::DataSourceParametersTwitter>>,
}
