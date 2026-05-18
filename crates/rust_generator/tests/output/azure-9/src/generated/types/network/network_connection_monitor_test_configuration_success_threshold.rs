#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkConnectionMonitorTestConfigurationSuccessThreshold {
    /// The maximum percentage of failed checks permitted for a test to be successful.
    #[builder(into)]
    #[serde(rename = "checksFailedPercent")]
    pub r#checks_failed_percent: Option<i32>,
    /// The maximum round-trip time in milliseconds permitted for a test to be successful.
    #[builder(into)]
    #[serde(rename = "roundTripTimeMs")]
    pub r#round_trip_time_ms: Option<f64>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkConnectionMonitorTestConfigurationSuccessThreshold {
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
                    "checks_failed_percent",
                    &self.r#checks_failed_percent,
                ),
                to_pulumi_object_field(
                    "round_trip_time_ms",
                    &self.r#round_trip_time_ms,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkConnectionMonitorTestConfigurationSuccessThreshold {
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
                    r#checks_failed_percent: {
                        let field_value = match fields_map.get("checks_failed_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'checks_failed_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#round_trip_time_ms: {
                        let field_value = match fields_map.get("round_trip_time_ms") {
                            Some(value) => value,
                            None => bail!("Missing field 'round_trip_time_ms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
