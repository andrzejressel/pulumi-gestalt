#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct QueueRateLimits {
    /// (Output)
    /// The max burst size.
    /// Max burst size limits how fast tasks in queue are processed when many tasks are
    /// in the queue and the rate is high. This field allows the queue to have a high
    /// rate so processing starts shortly after a task is enqueued, but still limits
    /// resource usage when many tasks are enqueued in a short period of time.
    #[builder(into)]
    #[serde(rename = "maxBurstSize")]
    pub r#max_burst_size: Option<i32>,
    /// The maximum number of concurrent tasks that Cloud Tasks allows to
    /// be dispatched for this queue. After this threshold has been
    /// reached, Cloud Tasks stops dispatching tasks until the number of
    /// concurrent requests decreases.
    #[builder(into)]
    #[serde(rename = "maxConcurrentDispatches")]
    pub r#max_concurrent_dispatches: Option<i32>,
    /// The maximum rate at which tasks are dispatched from this queue.
    /// If unspecified when the queue is created, Cloud Tasks will pick the default.
    #[builder(into)]
    #[serde(rename = "maxDispatchesPerSecond")]
    pub r#max_dispatches_per_second: Option<f64>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for QueueRateLimits {
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
                "max_burst_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_burst_size,
                )
                .await,
            );
            map.insert(
                "max_concurrent_dispatches".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_concurrent_dispatches,
                )
                .await,
            );
            map.insert(
                "max_dispatches_per_second".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_dispatches_per_second,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for QueueRateLimits {
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
                    r#max_burst_size: {
                        let field_value = match fields_map.get("max_burst_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_burst_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_concurrent_dispatches: {
                        let field_value = match fields_map.get("max_concurrent_dispatches") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_concurrent_dispatches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_dispatches_per_second: {
                        let field_value = match fields_map.get("max_dispatches_per_second") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_dispatches_per_second' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
