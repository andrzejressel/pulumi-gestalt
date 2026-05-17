#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ThreatIntelligenceIndicatorGranularMarking {
    /// The language of granular marking of the Threat Intelligence Indicator.
    #[builder(into)]
    #[serde(rename = "language")]
    pub r#language: Option<String>,
    /// The reference of the granular marking of the Threat Intelligence Indicator.
    #[builder(into)]
    #[serde(rename = "markingRef")]
    pub r#marking_ref: Option<String>,
    /// A list of selectors of the granular marking of the Threat Intelligence Indicator.
    #[builder(into)]
    #[serde(rename = "selectors")]
    pub r#selectors: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ThreatIntelligenceIndicatorGranularMarking {
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
                "language".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#language,
                )
                .await,
            );
            map.insert(
                "marking_ref".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#marking_ref,
                )
                .await,
            );
            map.insert(
                "selectors".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#selectors,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ThreatIntelligenceIndicatorGranularMarking {
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
                    r#language: {
                        let field_value = match fields_map.get("language") {
                            Some(value) => value,
                            None => bail!("Missing field 'language' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#marking_ref: {
                        let field_value = match fields_map.get("marking_ref") {
                            Some(value) => value,
                            None => bail!("Missing field 'marking_ref' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#selectors: {
                        let field_value = match fields_map.get("selectors") {
                            Some(value) => value,
                            None => bail!("Missing field 'selectors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
