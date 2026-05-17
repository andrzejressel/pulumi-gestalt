#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransform {
    /// Apply transformation to all findings not specified in other ImageTransformation's selectedInfoTypes.
    #[builder(into)]
    #[serde(rename = "allInfoTypes")]
    pub r#all_info_types: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransformAllInfoTypes>>,
    /// Apply transformation to all text that doesn't match an infoType.
    #[builder(into)]
    #[serde(rename = "allText")]
    pub r#all_text: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransformAllText>>,
    /// The color to use when redacting content from an image. If not specified, the default is black.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "redactionColor")]
    pub r#redaction_color: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransformRedactionColor>>,
    /// Apply transformation to the selected infoTypes.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "selectedInfoTypes")]
    pub r#selected_info_types: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransformSelectedInfoTypes>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransform {
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
                "all_info_types".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#all_info_types,
                )
                .await,
            );
            map.insert(
                "all_text".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#all_text,
                )
                .await,
            );
            map.insert(
                "redaction_color".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#redaction_color,
                )
                .await,
            );
            map.insert(
                "selected_info_types".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#selected_info_types,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransform {
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
                    r#all_info_types: {
                        let field_value = match fields_map.get("all_info_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'all_info_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#all_text: {
                        let field_value = match fields_map.get("all_text") {
                            Some(value) => value,
                            None => bail!("Missing field 'all_text' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redaction_color: {
                        let field_value = match fields_map.get("redaction_color") {
                            Some(value) => value,
                            None => bail!("Missing field 'redaction_color' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#selected_info_types: {
                        let field_value = match fields_map.get("selected_info_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'selected_info_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
