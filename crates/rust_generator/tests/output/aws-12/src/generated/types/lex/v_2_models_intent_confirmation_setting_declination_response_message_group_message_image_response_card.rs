#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentConfirmationSettingDeclinationResponseMessageGroupMessageImageResponseCard {
    /// Configuration blocks for buttons that should be displayed on the response card. The arrangement of the buttons is determined by the platform that displays the button. See `button`.
    #[builder(into)]
    #[serde(rename = "buttons")]
    pub r#buttons: Option<Vec<super::super::types::lex::V2ModelsIntentConfirmationSettingDeclinationResponseMessageGroupMessageImageResponseCardButton>>,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsIntentConfirmationSettingDeclinationResponseMessageGroupMessageImageResponseCard {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("buttons".to_string(), self.r#buttons.to_pulumi_value().await);
            map.insert("image_url".to_string(), self.r#image_url.to_pulumi_value().await);
            map.insert("subtitle".to_string(), self.r#subtitle.to_pulumi_value().await);
            map.insert("title".to_string(), self.r#title.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsIntentConfirmationSettingDeclinationResponseMessageGroupMessageImageResponseCard {
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
                    r#buttons: {
                        let field_value = match fields_map.get("buttons") {
                            Some(value) => value,
                            None => bail!("Missing field 'buttons' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::lex::V2ModelsIntentConfirmationSettingDeclinationResponseMessageGroupMessageImageResponseCardButton>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#image_url: {
                        let field_value = match fields_map.get("image_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#subtitle: {
                        let field_value = match fields_map.get("subtitle") {
                            Some(value) => value,
                            None => bail!("Missing field 'subtitle' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#title: {
                        let field_value = match fields_map.get("title") {
                            Some(value) => value,
                            None => bail!("Missing field 'title' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
