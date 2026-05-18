#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LaunchScheduledSplitsConfigStepSegmentOverride {
    /// Specifies a number indicating the order to use to evaluate segment overrides, if there are more than one. Segment overrides with lower numbers are evaluated first.
    #[builder(into)]
    #[serde(rename = "evaluationOrder")]
    pub r#evaluation_order: i32,
    /// The name or ARN of the segment to use.
    #[builder(into)]
    #[serde(rename = "segment")]
    pub r#segment: String,
    /// The traffic allocation percentages among the feature variations to assign to this segment. This is a set of key-value pairs. The keys are variation names. The values represent the amount of traffic to allocate to that variation for this segment. This is expressed in thousandths of a percent, so a weight of 50000 represents 50% of traffic.
    #[builder(into)]
    #[serde(rename = "weights")]
    pub r#weights: std::collections::HashMap<String, i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LaunchScheduledSplitsConfigStepSegmentOverride {
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
                    "evaluation_order",
                    &self.r#evaluation_order,
                ),
                to_pulumi_object_field(
                    "segment",
                    &self.r#segment,
                ),
                to_pulumi_object_field(
                    "weights",
                    &self.r#weights,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LaunchScheduledSplitsConfigStepSegmentOverride {
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
                    r#evaluation_order: {
                        let field_value = match fields_map.get("evaluation_order") {
                            Some(value) => value,
                            None => bail!("Missing field 'evaluation_order' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#segment: {
                        let field_value = match fields_map.get("segment") {
                            Some(value) => value,
                            None => bail!("Missing field 'segment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weights: {
                        let field_value = match fields_map.get("weights") {
                            Some(value) => value,
                            None => bail!("Missing field 'weights' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
