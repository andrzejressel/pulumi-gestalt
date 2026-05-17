#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessApplicationLandingPageDesign {
    /// The button color of the landing page.
    #[builder(into)]
    #[serde(rename = "buttonColor")]
    pub r#button_color: Option<String>,
    /// The button text color of the landing page.
    #[builder(into)]
    #[serde(rename = "buttonTextColor")]
    pub r#button_text_color: Option<String>,
    /// The URL of the image to be displayed in the landing page.
    #[builder(into)]
    #[serde(rename = "imageUrl")]
    pub r#image_url: Option<String>,
    /// The message of the landing page.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// The title of the landing page.
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccessApplicationLandingPageDesign {
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
                "button_color".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#button_color,
                )
                .await,
            );
            map.insert(
                "button_text_color".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#button_text_color,
                )
                .await,
            );
            map.insert(
                "image_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image_url,
                )
                .await,
            );
            map.insert(
                "message".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#message,
                )
                .await,
            );
            map.insert(
                "title".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#title,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccessApplicationLandingPageDesign {
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
                    r#button_color: {
                        let field_value = match fields_map.get("button_color") {
                            Some(value) => value,
                            None => bail!("Missing field 'button_color' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#button_text_color: {
                        let field_value = match fields_map.get("button_text_color") {
                            Some(value) => value,
                            None => bail!("Missing field 'button_text_color' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_url: {
                        let field_value = match fields_map.get("image_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#message: {
                        let field_value = match fields_map.get("message") {
                            Some(value) => value,
                            None => bail!("Missing field 'message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#title: {
                        let field_value = match fields_map.get("title") {
                            Some(value) => value,
                            None => bail!("Missing field 'title' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
