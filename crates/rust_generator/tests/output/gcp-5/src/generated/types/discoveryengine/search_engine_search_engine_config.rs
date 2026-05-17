#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SearchEngineSearchEngineConfig {
    /// The add-on that this search engine enables.
    /// Each value may be one of: `SEARCH_ADD_ON_LLM`.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "searchAddOns")]
    pub r#search_add_ons: Option<Vec<String>>,
    /// The search feature tier of this engine. Defaults to SearchTier.SEARCH_TIER_STANDARD if not specified.
    /// Default value is `SEARCH_TIER_STANDARD`.
    /// Possible values are: `SEARCH_TIER_STANDARD`, `SEARCH_TIER_ENTERPRISE`.
    #[builder(into)]
    #[serde(rename = "searchTier")]
    pub r#search_tier: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SearchEngineSearchEngineConfig {
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
                "search_add_ons".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#search_add_ons,
                )
                .await,
            );
            map.insert(
                "search_tier".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#search_tier,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SearchEngineSearchEngineConfig {
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
                    r#search_add_ons: {
                        let field_value = match fields_map.get("search_add_ons") {
                            Some(value) => value,
                            None => bail!("Missing field 'search_add_ons' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#search_tier: {
                        let field_value = match fields_map.get("search_tier") {
                            Some(value) => value,
                            None => bail!("Missing field 'search_tier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
