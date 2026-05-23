#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRecommendationsRecommendation {
    /// The category of the recommendation.
    #[builder(into)]
    pub r#category: String,
    /// The description of the issue or the opportunity identified by the recommendation.
    #[builder(into)]
    pub r#description: String,
    /// The business impact of the recommendation.
    #[builder(into)]
    pub r#impact: String,
    /// The name of the Advisor Recommendation.
    #[builder(into)]
    pub r#recommendation_name: String,
    /// The recommendation type id of the Advisor Recommendation.
    #[builder(into)]
    pub r#recommendation_type_id: String,
    /// The name of the identified resource of the Advisor Recommendation.
    #[builder(into)]
    pub r#resource_name: String,
    /// The type of the identified resource of the Advisor Recommendation.
    #[builder(into)]
    pub r#resource_type: String,
    /// A list of Advisor Suppression names of the Advisor Recommendation.
    #[builder(into)]
    pub r#suppression_names: Vec<String>,
    /// The most recent time that Advisor checked the validity of the recommendation..
    #[builder(into)]
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
                    "recommendationName",
                    &self.r#recommendation_name,
                ),
                to_pulumi_object_field(
                    "recommendationTypeId",
                    &self.r#recommendation_type_id,
                ),
                to_pulumi_object_field(
                    "resourceName",
                    &self.r#resource_name,
                ),
                to_pulumi_object_field(
                    "resourceType",
                    &self.r#resource_type,
                ),
                to_pulumi_object_field(
                    "suppressionNames",
                    &self.r#suppression_names,
                ),
                to_pulumi_object_field(
                    "updatedTime",
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
                        let field_value = match fields_map.get("recommendationName") {
                            Some(value) => value,
                            None => bail!("Missing field 'recommendationName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recommendation_type_id: {
                        let field_value = match fields_map.get("recommendationTypeId") {
                            Some(value) => value,
                            None => bail!("Missing field 'recommendationTypeId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_name: {
                        let field_value = match fields_map.get("resourceName") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_type: {
                        let field_value = match fields_map.get("resourceType") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#suppression_names: {
                        let field_value = match fields_map.get("suppressionNames") {
                            Some(value) => value,
                            None => bail!("Missing field 'suppressionNames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#updated_time: {
                        let field_value = match fields_map.get("updatedTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'updatedTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
