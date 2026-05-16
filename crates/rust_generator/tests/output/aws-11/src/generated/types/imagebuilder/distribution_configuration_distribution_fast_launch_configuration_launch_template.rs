#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionConfigurationDistributionFastLaunchConfigurationLaunchTemplate {
    /// The ID of the launch template to use for faster launching for a Windows AMI.
    #[builder(into)]
    #[serde(rename = "launchTemplateId")]
    pub r#launch_template_id: Option<String>,
    /// The name of the launch template to use for faster launching for a Windows AMI.
    #[builder(into)]
    #[serde(rename = "launchTemplateName")]
    pub r#launch_template_name: Option<String>,
    /// The version of the launch template to use for faster launching for a Windows AMI.
    #[builder(into)]
    #[serde(rename = "launchTemplateVersion")]
    pub r#launch_template_version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DistributionConfigurationDistributionFastLaunchConfigurationLaunchTemplate {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("launch_template_id".to_string(), self.r#launch_template_id.to_pulumi_value().await);
            map.insert("launch_template_name".to_string(), self.r#launch_template_name.to_pulumi_value().await);
            map.insert("launch_template_version".to_string(), self.r#launch_template_version.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DistributionConfigurationDistributionFastLaunchConfigurationLaunchTemplate {
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
                    r#launch_template_id: {
                        let field_value = match fields_map.get("launch_template_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_template_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#launch_template_name: {
                        let field_value = match fields_map.get("launch_template_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_template_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#launch_template_version: {
                        let field_value = match fields_map.get("launch_template_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_template_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
