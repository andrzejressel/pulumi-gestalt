#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProjectEnvironment {
    /// ARN of the S3 bucket, path prefix and object key that contains the PEM-encoded certificate.
    #[builder(into)]
    #[serde(rename = "certificate")]
    pub r#certificate: Option<String>,
    /// Information about the compute resources the build project will use. Valid values: `BUILD_GENERAL1_SMALL`, `BUILD_GENERAL1_MEDIUM`, `BUILD_GENERAL1_LARGE`, `BUILD_GENERAL1_2XLARGE`, `BUILD_LAMBDA_1GB`, `BUILD_LAMBDA_2GB`, `BUILD_LAMBDA_4GB`, `BUILD_LAMBDA_8GB`, `BUILD_LAMBDA_10GB`. `BUILD_GENERAL1_SMALL` is only valid if `type` is set to `LINUX_CONTAINER`. When `type` is set to `LINUX_GPU_CONTAINER`, `compute_type` must be `BUILD_GENERAL1_LARGE`. When `type` is set to `LINUX_LAMBDA_CONTAINER` or `ARM_LAMBDA_CONTAINER`, `compute_type` must be `BUILD_LAMBDA_XGB`.`
    #[builder(into)]
    #[serde(rename = "computeType")]
    pub r#compute_type: String,
    /// Configuration block. Detailed below.
    #[builder(into)]
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Option<Vec<super::super::types::codebuild::ProjectEnvironmentEnvironmentVariable>>,
    /// Configuration block. Detailed below.
    #[builder(into)]
    #[serde(rename = "fleet")]
    pub r#fleet: Option<Box<super::super::types::codebuild::ProjectEnvironmentFleet>>,
    /// Docker image to use for this build project. Valid values include [Docker images provided by CodeBuild](https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-available.html) (e.g `aws/codebuild/amazonlinux2-x86_64-standard:4.0`), [Docker Hub images](https://hub.docker.com/) (e.g., `pulumi/pulumi:latest`), and full Docker repository URIs such as those for ECR (e.g., `137112412989.dkr.ecr.us-west-2.amazonaws.com/amazonlinux:latest`).
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: String,
    /// Type of credentials AWS CodeBuild uses to pull images in your build. Valid values: `CODEBUILD`, `SERVICE_ROLE`. When you use a cross-account or private registry image, you must use SERVICE_ROLE credentials. When you use an AWS CodeBuild curated image, you must use CodeBuild credentials. Defaults to `CODEBUILD`.
    #[builder(into)]
    #[serde(rename = "imagePullCredentialsType")]
    pub r#image_pull_credentials_type: Option<String>,
    /// Whether to enable running the Docker daemon inside a Docker container. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "privilegedMode")]
    pub r#privileged_mode: Option<bool>,
    /// Configuration block. Detailed below.
    #[builder(into)]
    #[serde(rename = "registryCredential")]
    pub r#registry_credential: Option<Box<super::super::types::codebuild::ProjectEnvironmentRegistryCredential>>,
    /// Type of build environment to use for related builds. Valid values: `LINUX_CONTAINER`, `LINUX_GPU_CONTAINER`, `WINDOWS_CONTAINER` (deprecated), `WINDOWS_SERVER_2019_CONTAINER`, `ARM_CONTAINER`, `LINUX_LAMBDA_CONTAINER`, `ARM_LAMBDA_CONTAINER`. For additional information, see the [CodeBuild User Guide](https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html).
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProjectEnvironment {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("certificate".to_string(), self.r#certificate.to_pulumi_value().await);
            map.insert("compute_type".to_string(), self.r#compute_type.to_pulumi_value().await);
            map.insert("environment_variables".to_string(), self.r#environment_variables.to_pulumi_value().await);
            map.insert("fleet".to_string(), self.r#fleet.to_pulumi_value().await);
            map.insert("image".to_string(), self.r#image.to_pulumi_value().await);
            map.insert("image_pull_credentials_type".to_string(), self.r#image_pull_credentials_type.to_pulumi_value().await);
            map.insert("privileged_mode".to_string(), self.r#privileged_mode.to_pulumi_value().await);
            map.insert("registry_credential".to_string(), self.r#registry_credential.to_pulumi_value().await);
            map.insert("type_".to_string(), self.r#type_.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProjectEnvironment {
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
                    r#certificate: {
                        let field_value = match fields_map.get("certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#compute_type: {
                        let field_value = match fields_map.get("compute_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'compute_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#environment_variables: {
                        let field_value = match fields_map.get("environment_variables") {
                            Some(value) => value,
                            None => bail!("Missing field 'environment_variables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::codebuild::ProjectEnvironmentEnvironmentVariable>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#fleet: {
                        let field_value = match fields_map.get("fleet") {
                            Some(value) => value,
                            None => bail!("Missing field 'fleet' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::codebuild::ProjectEnvironmentFleet>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#image: {
                        let field_value = match fields_map.get("image") {
                            Some(value) => value,
                            None => bail!("Missing field 'image' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#image_pull_credentials_type: {
                        let field_value = match fields_map.get("image_pull_credentials_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_pull_credentials_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#privileged_mode: {
                        let field_value = match fields_map.get("privileged_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'privileged_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#registry_credential: {
                        let field_value = match fields_map.get("registry_credential") {
                            Some(value) => value,
                            None => bail!("Missing field 'registry_credential' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::codebuild::ProjectEnvironmentRegistryCredential>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
