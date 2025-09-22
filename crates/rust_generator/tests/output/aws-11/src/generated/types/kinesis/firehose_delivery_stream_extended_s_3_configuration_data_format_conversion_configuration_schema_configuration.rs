#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationSchemaConfiguration {
    /// The ID of the AWS Glue Data Catalog. If you don't supply this, the AWS account ID is used by default.
    #[builder(into)]
    #[serde(rename = "catalogId")]
    pub r#catalog_id: Option<String>,
    /// Specifies the name of the AWS Glue database that contains the schema for the output data.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: String,
    /// If you don't specify an AWS Region, the default is the current region.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Option<String>,
    /// The role that Kinesis Data Firehose can use to access AWS Glue. This role must be in the same account you use for Kinesis Data Firehose. Cross-account roles aren't allowed.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// Specifies the AWS Glue table that contains the column information that constitutes your data schema.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: String,
    /// Specifies the table version for the output data schema. Defaults to `LATEST`.
    #[builder(into)]
    #[serde(rename = "versionId")]
    pub r#version_id: Option<String>,
}
