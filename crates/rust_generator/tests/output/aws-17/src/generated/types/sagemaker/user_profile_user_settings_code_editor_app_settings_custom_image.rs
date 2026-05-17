#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserProfileUserSettingsCodeEditorAppSettingsCustomImage {
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UserProfileUserSettingsCodeEditorAppSettingsCustomImage {
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
                "app_image_config_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#app_image_config_name,
                )
                .await,
            );
            map.insert(
                "image_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image_name,
                )
                .await,
            );
            map.insert(
                "image_version_number".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image_version_number,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UserProfileUserSettingsCodeEditorAppSettingsCustomImage {
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
                    r#app_image_config_name: {
                        let field_value = match fields_map.get("app_image_config_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_image_config_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_name: {
                        let field_value = match fields_map.get("image_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_version_number: {
                        let field_value = match fields_map.get("image_version_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_version_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
