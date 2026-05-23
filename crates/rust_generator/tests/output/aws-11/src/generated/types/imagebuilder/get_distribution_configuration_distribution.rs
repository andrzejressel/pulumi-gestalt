#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDistributionConfigurationDistribution {
    /// Nested list of AMI distribution configuration.
    #[builder(into)]
    pub r#ami_distribution_configurations: Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionAmiDistributionConfiguration>,
    /// Nested list of container distribution configurations.
    #[builder(into)]
    pub r#container_distribution_configurations: Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionContainerDistributionConfiguration>,
    /// Nested list of Windows faster-launching configurations to use for AMI distribution.
    #[builder(into)]
    pub r#fast_launch_configurations: Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionFastLaunchConfiguration>,
    /// Nested list of launch template configurations.
    #[builder(into)]
    pub r#launch_template_configurations: Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionLaunchTemplateConfiguration>,
    /// Set of Amazon Resource Names (ARNs) of License Manager License Configurations.
    #[builder(into)]
    pub r#license_configuration_arns: Vec<String>,
    /// AWS Region of distribution.
    #[builder(into)]
    pub r#region: String,
    /// Nested list of S3 export configuration.
    #[builder(into)]
    pub r#s_3_export_configurations: Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionS3ExportConfiguration>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDistributionConfigurationDistribution {
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
                    "amiDistributionConfigurations",
                    &self.r#ami_distribution_configurations,
                ),
                to_pulumi_object_field(
                    "containerDistributionConfigurations",
                    &self.r#container_distribution_configurations,
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
                    "s3ExportConfigurations",
                    &self.r#s_3_export_configurations,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDistributionConfigurationDistribution {
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
                    r#ami_distribution_configurations: {
                        let field_value = match fields_map.get("amiDistributionConfigurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'amiDistributionConfigurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_distribution_configurations: {
                        let field_value = match fields_map.get("containerDistributionConfigurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'containerDistributionConfigurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#s_3_export_configurations: {
                        let field_value = match fields_map.get("s3ExportConfigurations") {
                            Some(value) => value,
                            None => bail!("Missing field 's3ExportConfigurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
