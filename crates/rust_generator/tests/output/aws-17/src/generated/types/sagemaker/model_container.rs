#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ModelContainer {
    /// The DNS host name for the container.
    #[builder(into)]
    #[serde(rename = "containerHostname")]
    pub r#container_hostname: Option<String>,
    /// Environment variables for the Docker container.
    /// A list of key value pairs.
    #[builder(into)]
    #[serde(rename = "environment")]
    pub r#environment: Option<std::collections::HashMap<String, String>>,
    /// The registry path where the inference code image is stored in Amazon ECR.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Option<String>,
    /// Specifies whether the model container is in Amazon ECR or a private Docker registry accessible from your Amazon Virtual Private Cloud (VPC). For more information see [Using a Private Docker Registry for Real-Time Inference Containers](https://docs.aws.amazon.com/sagemaker/latest/dg/your-algorithms-containers-inference-private.html). see Image Config.
    #[builder(into)]
    #[serde(rename = "imageConfig")]
    pub r#image_config: Box<Option<super::super::types::sagemaker::ModelContainerImageConfig>>,
    /// The inference specification name in the model package version.
    #[builder(into)]
    #[serde(rename = "inferenceSpecificationName")]
    pub r#inference_specification_name: Option<String>,
    /// The container hosts value `SingleModel/MultiModel`. The default value is `SingleModel`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    /// The location of model data to deploy. Use this for uncompressed model deployment. For information about how to deploy an uncompressed model, see [Deploying uncompressed models](https://docs.aws.amazon.com/sagemaker/latest/dg/large-model-inference-uncompressed.html) in the _AWS SageMaker Developer Guide_.
    #[builder(into)]
    #[serde(rename = "modelDataSource")]
    pub r#model_data_source: Box<Option<super::super::types::sagemaker::ModelContainerModelDataSource>>,
    /// The URL for the S3 location where model artifacts are stored.
    #[builder(into)]
    #[serde(rename = "modelDataUrl")]
    pub r#model_data_url: Option<String>,
    /// The Amazon Resource Name (ARN) of the model package to use to create the model.
    #[builder(into)]
    #[serde(rename = "modelPackageName")]
    pub r#model_package_name: Option<String>,
    /// Specifies additional configuration for multi-model endpoints. see Multi Model Config.
    #[builder(into)]
    #[serde(rename = "multiModelConfig")]
    pub r#multi_model_config: Box<Option<super::super::types::sagemaker::ModelContainerMultiModelConfig>>,
}
