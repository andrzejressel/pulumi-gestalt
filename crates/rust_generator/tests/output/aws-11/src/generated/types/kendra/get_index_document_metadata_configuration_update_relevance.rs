#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetIndexDocumentMetadataConfigurationUpdateRelevance {
    /// Time period that the boost applies to. For more information, refer to [Duration](https://docs.aws.amazon.com/kendra/latest/APIReference/API_Relevance.html#Kendra-Type-Relevance-Duration).
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: String,
    /// How "fresh" a document is. For more information, refer to [Freshness](https://docs.aws.amazon.com/kendra/latest/APIReference/API_Relevance.html#Kendra-Type-Relevance-Freshness).
    #[builder(into)]
    #[serde(rename = "freshness")]
    pub r#freshness: bool,
    /// Relative importance of the field in the search. Larger numbers provide more of a boost than smaller numbers. Minimum value of 1. Maximum value of 10.
    #[builder(into)]
    #[serde(rename = "importance")]
    pub r#importance: i32,
    /// Determines how values should be interpreted. For more information, refer to [RankOrder](https://docs.aws.amazon.com/kendra/latest/APIReference/API_Relevance.html#Kendra-Type-Relevance-RankOrder).
    #[builder(into)]
    #[serde(rename = "rankOrder")]
    pub r#rank_order: String,
    /// A list of values that should be given a different boost when they appear in the result list. For more information, refer to [ValueImportanceMap](https://docs.aws.amazon.com/kendra/latest/APIReference/API_Relevance.html#Kendra-Type-Relevance-ValueImportanceMap).
    #[builder(into)]
    #[serde(rename = "valuesImportanceMap")]
    pub r#values_importance_map: std::collections::HashMap<String, i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetIndexDocumentMetadataConfigurationUpdateRelevance {
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
                "duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#duration,
                )
                .await,
            );
            map.insert(
                "freshness".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#freshness,
                )
                .await,
            );
            map.insert(
                "importance".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#importance,
                )
                .await,
            );
            map.insert(
                "rank_order".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rank_order,
                )
                .await,
            );
            map.insert(
                "values_importance_map".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#values_importance_map,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetIndexDocumentMetadataConfigurationUpdateRelevance {
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
                    r#duration: {
                        let field_value = match fields_map.get("duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#freshness: {
                        let field_value = match fields_map.get("freshness") {
                            Some(value) => value,
                            None => bail!("Missing field 'freshness' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#importance: {
                        let field_value = match fields_map.get("importance") {
                            Some(value) => value,
                            None => bail!("Missing field 'importance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rank_order: {
                        let field_value = match fields_map.get("rank_order") {
                            Some(value) => value,
                            None => bail!("Missing field 'rank_order' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#values_importance_map: {
                        let field_value = match fields_map.get("values_importance_map") {
                            Some(value) => value,
                            None => bail!("Missing field 'values_importance_map' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
