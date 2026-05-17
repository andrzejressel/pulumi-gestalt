#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDistributionConfigurationDistribution {
    /// Nested list of AMI distribution configuration.
    #[builder(into)]
    #[serde(rename = "amiDistributionConfigurations")]
    pub r#ami_distribution_configurations: Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionAmiDistributionConfiguration>,
    /// Nested list of container distribution configurations.
    #[builder(into)]
    #[serde(rename = "containerDistributionConfigurations")]
    pub r#container_distribution_configurations: Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionContainerDistributionConfiguration>,
    /// Nested list of Windows faster-launching configurations to use for AMI distribution.
    #[builder(into)]
    #[serde(rename = "fastLaunchConfigurations")]
    pub r#fast_launch_configurations: Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionFastLaunchConfiguration>,
    /// Nested list of launch template configurations.
    #[builder(into)]
    #[serde(rename = "launchTemplateConfigurations")]
    pub r#launch_template_configurations: Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionLaunchTemplateConfiguration>,
    /// Set of Amazon Resource Names (ARNs) of License Manager License Configurations.
    #[builder(into)]
    #[serde(rename = "licenseConfigurationArns")]
    pub r#license_configuration_arns: Vec<String>,
    /// AWS Region of distribution.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: String,
    /// Nested list of S3 export configuration.
    #[builder(into)]
    #[serde(rename = "s3ExportConfigurations")]
    pub r#s_3_export_configurations: Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionS3ExportConfiguration>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDistributionConfigurationDistribution {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "ami_distribution_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ami_distribution_configurations,
                )
                .await,
            );
            map.insert(
                "container_distribution_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container_distribution_configurations,
                )
                .await,
            );
            map.insert(
                "fast_launch_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fast_launch_configurations,
                )
                .await,
            );
            map.insert(
                "launch_template_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#launch_template_configurations,
                )
                .await,
            );
            map.insert(
                "license_configuration_arns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#license_configuration_arns,
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
                "s_3_export_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_export_configurations,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
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
                        let field_value = match fields_map.get("ami_distribution_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'ami_distribution_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_distribution_configurations: {
                        let field_value = match fields_map.get("container_distribution_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_distribution_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fast_launch_configurations: {
                        let field_value = match fields_map.get("fast_launch_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'fast_launch_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#launch_template_configurations: {
                        let field_value = match fields_map.get("launch_template_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_template_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#license_configuration_arns: {
                        let field_value = match fields_map.get("license_configuration_arns") {
                            Some(value) => value,
                            None => bail!("Missing field 'license_configuration_arns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("s_3_export_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_export_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
