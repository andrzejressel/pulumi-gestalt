#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceAutoscalingConfigAutoscalingLimits {
    /// The maximum number of nodes for this specific replica.
    #[builder(into)]
    #[serde(rename = "maxNodes")]
    pub r#max_nodes: Option<i32>,
    /// Specifies maximum number of processing units allocated to the instance.
    /// If set, this number should be multiples of 1000 and be greater than or equal to
    /// min_processing_units.
    #[builder(into)]
    #[serde(rename = "maxProcessingUnits")]
    pub r#max_processing_units: Option<i32>,
    /// The minimum number of nodes for this specific replica.
    #[builder(into)]
    #[serde(rename = "minNodes")]
    pub r#min_nodes: Option<i32>,
    /// Specifies minimum number of processing units allocated to the instance.
    /// If set, this number should be multiples of 1000.
    #[builder(into)]
    #[serde(rename = "minProcessingUnits")]
    pub r#min_processing_units: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceAutoscalingConfigAutoscalingLimits {
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
                    "maxNodes",
                    &self.r#max_nodes,
                ),
                to_pulumi_object_field(
                    "maxProcessingUnits",
                    &self.r#max_processing_units,
                ),
                to_pulumi_object_field(
                    "minNodes",
                    &self.r#min_nodes,
                ),
                to_pulumi_object_field(
                    "minProcessingUnits",
                    &self.r#min_processing_units,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceAutoscalingConfigAutoscalingLimits {
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
                    r#max_nodes: {
                        let field_value = match fields_map.get("maxNodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxNodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_processing_units: {
                        let field_value = match fields_map.get("maxProcessingUnits") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxProcessingUnits' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_nodes: {
                        let field_value = match fields_map.get("minNodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'minNodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_processing_units: {
                        let field_value = match fields_map.get("minProcessingUnits") {
                            Some(value) => value,
                            None => bail!("Missing field 'minProcessingUnits' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
