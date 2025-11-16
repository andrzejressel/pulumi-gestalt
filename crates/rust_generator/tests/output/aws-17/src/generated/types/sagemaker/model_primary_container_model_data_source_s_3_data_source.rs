#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ModelPrimaryContainerModelDataSourceS3DataSource {
    /// How the model data is prepared. Allowed values are: `None` and `Gzip`.
    #[builder(into)]
    #[serde(rename = "compressionType")]
    pub r#compression_type: String,
    /// Specifies the access configuration file for the ML model. You can explicitly accept the model end-user license agreement (EULA) within the [`model_access_config` configuration block]. see Model Access Config.
    #[builder(into)]
    #[serde(rename = "modelAccessConfig")]
    pub r#model_access_config: Option<Box<super::super::types::sagemaker::ModelPrimaryContainerModelDataSourceS3DataSourceModelAccessConfig>>,
    /// The type of model data to deploy. Allowed values are: `S3Object` and `S3Prefix`.
    #[builder(into)]
    #[serde(rename = "s3DataType")]
    pub r#s_3_data_type: String,
    /// The S3 path of model data to deploy.
    #[builder(into)]
    #[serde(rename = "s3Uri")]
    pub r#s_3_uri: String,
}
