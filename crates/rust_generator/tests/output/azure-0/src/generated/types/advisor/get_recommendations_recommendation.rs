#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRecommendationsRecommendation {
    /// The category of the recommendation.
    #[builder(into)]
    #[serde(rename = "category")]
    pub r#category: String,
    /// The description of the issue or the opportunity identified by the recommendation.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// The business impact of the recommendation.
    #[builder(into)]
    #[serde(rename = "impact")]
    pub r#impact: String,
    /// The name of the Advisor Recommendation.
    #[builder(into)]
    #[serde(rename = "recommendationName")]
    pub r#recommendation_name: String,
    /// The recommendation type id of the Advisor Recommendation.
    #[builder(into)]
    #[serde(rename = "recommendationTypeId")]
    pub r#recommendation_type_id: String,
    /// The name of the identified resource of the Advisor Recommendation.
    #[builder(into)]
    #[serde(rename = "resourceName")]
    pub r#resource_name: String,
    /// The type of the identified resource of the Advisor Recommendation.
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: String,
    /// A list of Advisor Suppression names of the Advisor Recommendation.
    #[builder(into)]
    #[serde(rename = "suppressionNames")]
    pub r#suppression_names: Vec<String>,
    /// The most recent time that Advisor checked the validity of the recommendation..
    #[builder(into)]
    #[serde(rename = "updatedTime")]
    pub r#updated_time: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRecommendationsRecommendation {
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
                "category".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#category,
                )
                .await,
            );
            map.insert(
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "impact".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#impact,
                )
                .await,
            );
            map.insert(
                "recommendation_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#recommendation_name,
                )
                .await,
            );
            map.insert(
                "recommendation_type_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#recommendation_type_id,
                )
                .await,
            );
            map.insert(
                "resource_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_name,
                )
                .await,
            );
            map.insert(
                "resource_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_type,
                )
                .await,
            );
            map.insert(
                "suppression_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#suppression_names,
                )
                .await,
            );
            map.insert(
                "updated_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#updated_time,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRecommendationsRecommendation {
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
                    r#category: {
                        let field_value = match fields_map.get("category") {
                            Some(value) => value,
                            None => bail!("Missing field 'category' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#impact: {
                        let field_value = match fields_map.get("impact") {
                            Some(value) => value,
                            None => bail!("Missing field 'impact' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recommendation_name: {
                        let field_value = match fields_map.get("recommendation_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'recommendation_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recommendation_type_id: {
                        let field_value = match fields_map.get("recommendation_type_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'recommendation_type_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_name: {
                        let field_value = match fields_map.get("resource_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_type: {
                        let field_value = match fields_map.get("resource_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#suppression_names: {
                        let field_value = match fields_map.get("suppression_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'suppression_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#updated_time: {
                        let field_value = match fields_map.get("updated_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'updated_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
