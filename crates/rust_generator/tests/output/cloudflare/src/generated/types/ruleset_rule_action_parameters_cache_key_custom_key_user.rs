#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyUser {
    /// Add device type to the custom key.
    #[builder(into)]
    #[serde(rename = "deviceType")]
    pub r#device_type: Option<bool>,
    /// Add geo data to the custom key.
    #[builder(into)]
    #[serde(rename = "geo")]
    pub r#geo: Option<bool>,
    /// Add language data to the custom key.
    #[builder(into)]
    #[serde(rename = "lang")]
    pub r#lang: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RulesetRuleActionParametersCacheKeyCustomKeyUser {
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
                "device_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#device_type,
                )
                .await,
            );
            map.insert(
                "geo".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#geo,
                )
                .await,
            );
            map.insert(
                "lang".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lang,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RulesetRuleActionParametersCacheKeyCustomKeyUser {
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
                    r#device_type: {
                        let field_value = match fields_map.get("device_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#geo: {
                        let field_value = match fields_map.get("geo") {
                            Some(value) => value,
                            None => bail!("Missing field 'geo' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lang: {
                        let field_value = match fields_map.get("lang") {
                            Some(value) => value,
                            None => bail!("Missing field 'lang' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
