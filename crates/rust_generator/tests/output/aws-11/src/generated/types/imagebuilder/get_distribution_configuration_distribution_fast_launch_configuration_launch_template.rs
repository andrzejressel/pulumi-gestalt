#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDistributionConfigurationDistributionFastLaunchConfigurationLaunchTemplate {
    /// ID of the Amazon EC2 launch template.
    #[builder(into)]
    #[serde(rename = "launchTemplateId")]
    pub r#launch_template_id: String,
    /// The name of the launch template to use for faster launching for a Windows AMI.
    #[builder(into)]
    #[serde(rename = "launchTemplateName")]
    pub r#launch_template_name: String,
    /// The version of the launch template to use for faster launching for a Windows AMI.
    #[builder(into)]
    #[serde(rename = "launchTemplateVersion")]
    pub r#launch_template_version: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDistributionConfigurationDistributionFastLaunchConfigurationLaunchTemplate {
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
                "launch_template_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#launch_template_id,
                )
                .await,
            );
            map.insert(
                "launch_template_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#launch_template_name,
                )
                .await,
            );
            map.insert(
                "launch_template_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#launch_template_version,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDistributionConfigurationDistributionFastLaunchConfigurationLaunchTemplate {
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
                    r#launch_template_id: {
                        let field_value = match fields_map.get("launch_template_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_template_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#launch_template_name: {
                        let field_value = match fields_map.get("launch_template_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_template_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#launch_template_version: {
                        let field_value = match fields_map.get("launch_template_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_template_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
