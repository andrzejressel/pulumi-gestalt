#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorCapacityAutoscaling {
    /// The maximum number of workers allocated to the connector.
    #[builder(into)]
    #[serde(rename = "maxWorkerCount")]
    pub r#max_worker_count: i32,
    /// The number of microcontroller units (MCUs) allocated to each connector worker. Valid values: `1`, `2`, `4`, `8`. The default value is `1`.
    #[builder(into)]
    #[serde(rename = "mcuCount")]
    pub r#mcu_count: Option<i32>,
    /// The minimum number of workers allocated to the connector.
    #[builder(into)]
    #[serde(rename = "minWorkerCount")]
    pub r#min_worker_count: i32,
    /// The scale-in policy for the connector. See `scale_in_policy` Block for details.
    #[builder(into)]
    #[serde(rename = "scaleInPolicy")]
    pub r#scale_in_policy: Option<Box<super::super::types::mskconnect::ConnectorCapacityAutoscalingScaleInPolicy>>,
    /// The scale-out policy for the connector. See `scale_out_policy` Block for details.
    #[builder(into)]
    #[serde(rename = "scaleOutPolicy")]
    pub r#scale_out_policy: Option<Box<super::super::types::mskconnect::ConnectorCapacityAutoscalingScaleOutPolicy>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectorCapacityAutoscaling {
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
                    "max_worker_count",
                    &self.r#max_worker_count,
                ),
                to_pulumi_object_field(
                    "mcu_count",
                    &self.r#mcu_count,
                ),
                to_pulumi_object_field(
                    "min_worker_count",
                    &self.r#min_worker_count,
                ),
                to_pulumi_object_field(
                    "scale_in_policy",
                    &self.r#scale_in_policy,
                ),
                to_pulumi_object_field(
                    "scale_out_policy",
                    &self.r#scale_out_policy,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectorCapacityAutoscaling {
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
                    r#max_worker_count: {
                        let field_value = match fields_map.get("max_worker_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_worker_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mcu_count: {
                        let field_value = match fields_map.get("mcu_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'mcu_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_worker_count: {
                        let field_value = match fields_map.get("min_worker_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_worker_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_in_policy: {
                        let field_value = match fields_map.get("scale_in_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'scale_in_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_out_policy: {
                        let field_value = match fields_map.get("scale_out_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'scale_out_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
