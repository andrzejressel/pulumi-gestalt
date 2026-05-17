#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ThreatIntelligenceIndicatorParsedPattern {
    /// The type key of parsed pattern.
    #[builder(into)]
    #[serde(rename = "patternTypeKey")]
    pub r#pattern_type_key: Option<String>,
    /// A `pattern_type_values` block as defined below.
    #[builder(into)]
    #[serde(rename = "patternTypeValues")]
    pub r#pattern_type_values: Option<Vec<super::super::types::sentinel::ThreatIntelligenceIndicatorParsedPatternPatternTypeValue>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ThreatIntelligenceIndicatorParsedPattern {
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
                "pattern_type_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pattern_type_key,
                )
                .await,
            );
            map.insert(
                "pattern_type_values".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pattern_type_values,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ThreatIntelligenceIndicatorParsedPattern {
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
                    r#pattern_type_key: {
                        let field_value = match fields_map.get("pattern_type_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'pattern_type_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pattern_type_values: {
                        let field_value = match fields_map.get("pattern_type_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'pattern_type_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
