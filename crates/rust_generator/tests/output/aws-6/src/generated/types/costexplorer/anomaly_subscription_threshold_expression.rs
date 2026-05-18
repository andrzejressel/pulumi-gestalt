#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AnomalySubscriptionThresholdExpression {
    /// Return results that match both Dimension objects.
    #[builder(into)]
    #[serde(rename = "ands")]
    pub r#ands: Option<Vec<super::super::types::costexplorer::AnomalySubscriptionThresholdExpressionAnd>>,
    /// Configuration block for the filter that's based on  values. See Cost Category below.
    #[builder(into)]
    #[serde(rename = "costCategory")]
    pub r#cost_category: Option<Box<super::super::types::costexplorer::AnomalySubscriptionThresholdExpressionCostCategory>>,
    /// Configuration block for the specific Dimension to use for.
    #[builder(into)]
    #[serde(rename = "dimension")]
    pub r#dimension: Option<Box<super::super::types::costexplorer::AnomalySubscriptionThresholdExpressionDimension>>,
    /// Return results that do not match the Dimension object.
    #[builder(into)]
    #[serde(rename = "not")]
    pub r#not: Option<Box<super::super::types::costexplorer::AnomalySubscriptionThresholdExpressionNot>>,
    /// Return results that match either Dimension object.
    #[builder(into)]
    #[serde(rename = "ors")]
    pub r#ors: Option<Vec<super::super::types::costexplorer::AnomalySubscriptionThresholdExpressionOr>>,
    /// Configuration block for the specific Tag to use for. See Tags below.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<Box<super::super::types::costexplorer::AnomalySubscriptionThresholdExpressionTags>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AnomalySubscriptionThresholdExpression {
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
                    "ands",
                    &self.r#ands,
                ),
                to_pulumi_object_field(
                    "cost_category",
                    &self.r#cost_category,
                ),
                to_pulumi_object_field(
                    "dimension",
                    &self.r#dimension,
                ),
                to_pulumi_object_field(
                    "not",
                    &self.r#not,
                ),
                to_pulumi_object_field(
                    "ors",
                    &self.r#ors,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AnomalySubscriptionThresholdExpression {
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
                    r#ands: {
                        let field_value = match fields_map.get("ands") {
                            Some(value) => value,
                            None => bail!("Missing field 'ands' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cost_category: {
                        let field_value = match fields_map.get("cost_category") {
                            Some(value) => value,
                            None => bail!("Missing field 'cost_category' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dimension: {
                        let field_value = match fields_map.get("dimension") {
                            Some(value) => value,
                            None => bail!("Missing field 'dimension' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#not: {
                        let field_value = match fields_map.get("not") {
                            Some(value) => value,
                            None => bail!("Missing field 'not' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ors: {
                        let field_value = match fields_map.get("ors") {
                            Some(value) => value,
                            None => bail!("Missing field 'ors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
