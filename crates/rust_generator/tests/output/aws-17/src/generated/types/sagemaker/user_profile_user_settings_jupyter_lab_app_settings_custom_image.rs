#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserProfileUserSettingsJupyterLabAppSettingsCustomImage {
    /// The name of the App Image Config.
    #[builder(into)]
    #[serde(rename = "appImageConfigName")]
    pub r#app_image_config_name: String,
    /// The name of the Custom Image.
    #[builder(into)]
    #[serde(rename = "imageName")]
    pub r#image_name: String,
    /// The version number of the Custom Image.
    #[builder(into)]
    #[serde(rename = "imageVersionNumber")]
    pub r#image_version_number: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UserProfileUserSettingsJupyterLabAppSettingsCustomImage {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("app_image_config_name".to_string(), self.r#app_image_config_name.to_pulumi_value().await);
            map.insert("image_name".to_string(), self.r#image_name.to_pulumi_value().await);
            map.insert("image_version_number".to_string(), self.r#image_version_number.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UserProfileUserSettingsJupyterLabAppSettingsCustomImage {
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
                    r#app_image_config_name: {
                        let field_value = match fields_map.get("app_image_config_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_image_config_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#image_name: {
                        let field_value = match fields_map.get("image_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#image_version_number: {
                        let field_value = match fields_map.get("image_version_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_version_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
