#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransformRedactionColor {
    /// The amount of blue in the color as a value in the interval [0, 1].
    #[builder(into)]
    #[serde(rename = "blue")]
    pub r#blue: Option<f64>,
    /// The amount of green in the color as a value in the interval [0, 1].
    #[builder(into)]
    #[serde(rename = "green")]
    pub r#green: Option<f64>,
    /// The amount of red in the color as a value in the interval [0, 1].
    #[builder(into)]
    #[serde(rename = "red")]
    pub r#red: Option<f64>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransformRedactionColor {
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
                "blue".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#blue,
                )
                .await,
            );
            map.insert(
                "green".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#green,
                )
                .await,
            );
            map.insert(
                "red".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#red,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransformRedactionColor {
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
                    r#blue: {
                        let field_value = match fields_map.get("blue") {
                            Some(value) => value,
                            None => bail!("Missing field 'blue' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#green: {
                        let field_value = match fields_map.get("green") {
                            Some(value) => value,
                            None => bail!("Missing field 'green' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#red: {
                        let field_value = match fields_map.get("red") {
                            Some(value) => value,
                            None => bail!("Missing field 'red' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
