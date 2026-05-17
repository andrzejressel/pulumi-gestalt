#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVoicesVoice {
    /// Additional codes for languages available for the specified voice in addition to its default language.
    #[builder(into)]
    #[serde(rename = "additionalLanguageCodes")]
    pub r#additional_language_codes: Vec<String>,
    /// Gender of the voice.
    #[builder(into)]
    #[serde(rename = "gender")]
    pub r#gender: String,
    /// Amazon Polly assigned voice ID.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Language identification tag for filtering the list of voices returned. If not specified, all available voices are returned.
    #[builder(into)]
    #[serde(rename = "languageCode")]
    pub r#language_code: String,
    /// Human readable name of the language in English.
    #[builder(into)]
    #[serde(rename = "languageName")]
    pub r#language_name: String,
    /// Name of the voice.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specifies which engines are supported by a given voice.
    #[builder(into)]
    #[serde(rename = "supportedEngines")]
    pub r#supported_engines: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVoicesVoice {
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
                "additional_language_codes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#additional_language_codes,
                )
                .await,
            );
            map.insert(
                "gender".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gender,
                )
                .await,
            );
            map.insert(
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "language_code".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#language_code,
                )
                .await,
            );
            map.insert(
                "language_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#language_name,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "supported_engines".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#supported_engines,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVoicesVoice {
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
                    r#additional_language_codes: {
                        let field_value = match fields_map.get("additional_language_codes") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_language_codes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gender: {
                        let field_value = match fields_map.get("gender") {
                            Some(value) => value,
                            None => bail!("Missing field 'gender' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#language_code: {
                        let field_value = match fields_map.get("language_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'language_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#language_name: {
                        let field_value = match fields_map.get("language_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'language_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#supported_engines: {
                        let field_value = match fields_map.get("supported_engines") {
                            Some(value) => value,
                            None => bail!("Missing field 'supported_engines' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
