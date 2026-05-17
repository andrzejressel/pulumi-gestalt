#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentConfirmationSettingConfirmationResponseMessageGroupMessageImageResponseCard {
    /// Configuration blocks for buttons that should be displayed on the response card. The arrangement of the buttons is determined by the platform that displays the button. See `button`.
    #[builder(into)]
    #[serde(rename = "buttons")]
    pub r#buttons: Option<Vec<super::super::types::lex::V2ModelsIntentConfirmationSettingConfirmationResponseMessageGroupMessageImageResponseCardButton>>,
    /// URL of an image to display on the response card. The image URL must be publicly available so that the platform displaying the response card has access to the image.
    #[builder(into)]
    #[serde(rename = "imageUrl")]
    pub r#image_url: Option<String>,
    /// Subtitle to display on the response card. The format of the subtitle is determined by the platform displaying the response card.
    #[builder(into)]
    #[serde(rename = "subtitle")]
    pub r#subtitle: Option<String>,
    /// Title to display on the response card. The format of the title is determined by the platform displaying the response card.
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsIntentConfirmationSettingConfirmationResponseMessageGroupMessageImageResponseCard {
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
                "buttons".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#buttons,
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
                "subtitle".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subtitle,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsIntentConfirmationSettingConfirmationResponseMessageGroupMessageImageResponseCard {
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
                    r#buttons: {
                        let field_value = match fields_map.get("buttons") {
                            Some(value) => value,
                            None => bail!("Missing field 'buttons' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#subtitle: {
                        let field_value = match fields_map.get("subtitle") {
                            Some(value) => value,
                            None => bail!("Missing field 'subtitle' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
