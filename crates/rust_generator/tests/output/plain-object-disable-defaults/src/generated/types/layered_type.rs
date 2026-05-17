#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LayeredType {
    /// The answer to the question
    #[builder(into)]
    #[serde(rename = "answer")]
    pub r#answer: Option<f64>,
    #[builder(into)]
    #[serde(rename = "other")]
    pub r#other: Box<super::types::HelmReleaseSettings>,
    /// Test how plain types interact
    #[builder(into)]
    #[serde(rename = "plainOther")]
    pub r#plain_other: Option<Box<super::types::HelmReleaseSettings>>,
    /// The question already answered
    #[builder(into)]
    #[serde(rename = "question")]
    pub r#question: Option<String>,
    #[builder(into)]
    #[serde(rename = "recursive")]
    pub r#recursive: Option<Box<super::types::LayeredType>>,
    /// To ask and answer
    #[builder(into)]
    #[serde(rename = "thinker")]
    pub r#thinker: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LayeredType {
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
                "answer".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#answer,
                )
                .await,
            );
            map.insert(
                "other".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#other,
                )
                .await,
            );
            map.insert(
                "plain_other".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#plain_other,
                )
                .await,
            );
            map.insert(
                "question".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#question,
                )
                .await,
            );
            map.insert(
                "recursive".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#recursive,
                )
                .await,
            );
            map.insert(
                "thinker".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#thinker,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LayeredType {
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
                    r#answer: {
                        let field_value = match fields_map.get("answer") {
                            Some(value) => value,
                            None => bail!("Missing field 'answer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#other: {
                        let field_value = match fields_map.get("other") {
                            Some(value) => value,
                            None => bail!("Missing field 'other' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#plain_other: {
                        let field_value = match fields_map.get("plain_other") {
                            Some(value) => value,
                            None => bail!("Missing field 'plain_other' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#question: {
                        let field_value = match fields_map.get("question") {
                            Some(value) => value,
                            None => bail!("Missing field 'question' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recursive: {
                        let field_value = match fields_map.get("recursive") {
                            Some(value) => value,
                            None => bail!("Missing field 'recursive' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#thinker: {
                        let field_value = match fields_map.get("thinker") {
                            Some(value) => value,
                            None => bail!("Missing field 'thinker' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
