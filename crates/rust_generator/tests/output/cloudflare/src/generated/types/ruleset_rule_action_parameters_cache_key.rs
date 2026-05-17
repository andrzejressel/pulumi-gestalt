#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RulesetRuleActionParametersCacheKey {
    /// Cache by device type.
    #[builder(into)]
    #[serde(rename = "cacheByDeviceType")]
    pub r#cache_by_device_type: Option<bool>,
    /// Cache deception armor.
    #[builder(into)]
    #[serde(rename = "cacheDeceptionArmor")]
    pub r#cache_deception_armor: Option<bool>,
    /// Custom key parameters for the request.
    #[builder(into)]
    #[serde(rename = "customKey")]
    pub r#custom_key: Option<Box<super::types::RulesetRuleActionParametersCacheKeyCustomKey>>,
    /// Ignore query strings order.
    #[builder(into)]
    #[serde(rename = "ignoreQueryStringsOrder")]
    pub r#ignore_query_strings_order: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RulesetRuleActionParametersCacheKey {
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
                "cache_by_device_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cache_by_device_type,
                )
                .await,
            );
            map.insert(
                "cache_deception_armor".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cache_deception_armor,
                )
                .await,
            );
            map.insert(
                "custom_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_key,
                )
                .await,
            );
            map.insert(
                "ignore_query_strings_order".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ignore_query_strings_order,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RulesetRuleActionParametersCacheKey {
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
                    r#cache_by_device_type: {
                        let field_value = match fields_map.get("cache_by_device_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_by_device_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_deception_armor: {
                        let field_value = match fields_map.get("cache_deception_armor") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_deception_armor' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_key: {
                        let field_value = match fields_map.get("custom_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ignore_query_strings_order: {
                        let field_value = match fields_map.get("ignore_query_strings_order") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignore_query_strings_order' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
