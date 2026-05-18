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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "category",
                    &self.r#category,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "impact",
                    &self.r#impact,
                ),
                to_pulumi_object_field(
                    "recommendation_name",
                    &self.r#recommendation_name,
                ),
                to_pulumi_object_field(
                    "recommendation_type_id",
                    &self.r#recommendation_type_id,
                ),
                to_pulumi_object_field(
                    "resource_name",
                    &self.r#resource_name,
                ),
                to_pulumi_object_field(
                    "resource_type",
                    &self.r#resource_type,
                ),
                to_pulumi_object_field(
                    "suppression_names",
                    &self.r#suppression_names,
                ),
                to_pulumi_object_field(
                    "updated_time",
                    &self.r#updated_time,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
