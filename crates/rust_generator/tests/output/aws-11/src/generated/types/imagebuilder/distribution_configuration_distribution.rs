#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionConfigurationDistribution {
    /// Configuration block with Amazon Machine Image (AMI) distribution settings. Detailed below.
    #[builder(into)]
    #[serde(rename = "amiDistributionConfiguration")]
    pub r#ami_distribution_configuration: Option<Box<super::super::types::imagebuilder::DistributionConfigurationDistributionAmiDistributionConfiguration>>,
    /// Configuration block with container distribution settings. Detailed below.
    #[builder(into)]
    #[serde(rename = "containerDistributionConfiguration")]
    pub r#container_distribution_configuration: Option<Box<super::super::types::imagebuilder::DistributionConfigurationDistributionContainerDistributionConfiguration>>,
    /// Set of Windows faster-launching configurations to use for AMI distribution. Detailed below.
    #[builder(into)]
    #[serde(rename = "fastLaunchConfigurations")]
    pub r#fast_launch_configurations: Option<Vec<super::super::types::imagebuilder::DistributionConfigurationDistributionFastLaunchConfiguration>>,
    /// Set of launch template configuration settings that apply to image distribution. Detailed below.
    #[builder(into)]
    #[serde(rename = "launchTemplateConfigurations")]
    pub r#launch_template_configurations: Option<Vec<super::super::types::imagebuilder::DistributionConfigurationDistributionLaunchTemplateConfiguration>>,
    /// Set of Amazon Resource Names (ARNs) of License Manager License Configurations.
    #[builder(into)]
    #[serde(rename = "licenseConfigurationArns")]
    pub r#license_configuration_arns: Option<Vec<String>>,
    /// AWS Region for the distribution.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: String,
    /// Configuration block with S3 export settings. Detailed below.
    #[builder(into)]
    #[serde(rename = "s3ExportConfiguration")]
    pub r#s_3_export_configuration: Option<Box<super::super::types::imagebuilder::DistributionConfigurationDistributionS3ExportConfiguration>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DistributionConfigurationDistribution {
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
                    "amiDistributionConfiguration",
                    &self.r#ami_distribution_configuration,
                ),
                to_pulumi_object_field(
                    "containerDistributionConfiguration",
                    &self.r#container_distribution_configuration,
                ),
                to_pulumi_object_field(
                    "fastLaunchConfigurations",
                    &self.r#fast_launch_configurations,
                ),
                to_pulumi_object_field(
                    "launchTemplateConfigurations",
                    &self.r#launch_template_configurations,
                ),
                to_pulumi_object_field(
                    "licenseConfigurationArns",
                    &self.r#license_configuration_arns,
                ),
                to_pulumi_object_field(
                    "region",
                    &self.r#region,
                ),
                to_pulumi_object_field(
                    "s3ExportConfiguration",
                    &self.r#s_3_export_configuration,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DistributionConfigurationDistribution {
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
                    r#ami_distribution_configuration: {
                        let field_value = match fields_map.get("amiDistributionConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'amiDistributionConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_distribution_configuration: {
                        let field_value = match fields_map.get("containerDistributionConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'containerDistributionConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fast_launch_configurations: {
                        let field_value = match fields_map.get("fastLaunchConfigurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'fastLaunchConfigurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#launch_template_configurations: {
                        let field_value = match fields_map.get("launchTemplateConfigurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'launchTemplateConfigurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#license_configuration_arns: {
                        let field_value = match fields_map.get("licenseConfigurationArns") {
                            Some(value) => value,
                            None => bail!("Missing field 'licenseConfigurationArns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#s_3_export_configuration: {
                        let field_value = match fields_map.get("s3ExportConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 's3ExportConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
