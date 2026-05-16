#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainDefaultUserSettingsRSessionAppSettings {
    /// A list of custom SageMaker images that are configured to run as a RSession app. see `custom_image` Block below.
    #[builder(into)]
    #[serde(rename = "customImages")]
    pub r#custom_images: Option<Vec<super::super::types::sagemaker::DomainDefaultUserSettingsRSessionAppSettingsCustomImage>>,
    /// The default instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance. see `default_resource_spec` Block above.
    #[builder(into)]
    #[serde(rename = "defaultResourceSpec")]
    pub r#default_resource_spec: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsRSessionAppSettingsDefaultResourceSpec>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainDefaultUserSettingsRSessionAppSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("custom_images".to_string(), self.r#custom_images.to_pulumi_value().await);
            map.insert("default_resource_spec".to_string(), self.r#default_resource_spec.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainDefaultUserSettingsRSessionAppSettings {
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
                    r#custom_images: {
                        let field_value = match fields_map.get("custom_images") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_images' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::sagemaker::DomainDefaultUserSettingsRSessionAppSettingsCustomImage>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#default_resource_spec: {
                        let field_value = match fields_map.get("default_resource_spec") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_resource_spec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsRSessionAppSettingsDefaultResourceSpec>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
