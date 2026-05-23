#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SloRequestBasedSliGoodTotalRatio {
    /// A TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// quantifying bad service provided, either demanded service that
    /// was not provided or demanded service that was of inadequate
    /// quality. Exactly two of
    /// good, bad, or total service filter must be defined (where
    /// good + bad = total is assumed)
    /// Must have ValueType = DOUBLE or ValueType = INT64 and
    /// must have MetricKind = DELTA or MetricKind = CUMULATIVE.
    #[builder(into)]
    pub r#bad_service_filter: Option<String>,
    /// A TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// quantifying good service provided. Exactly two of
    /// good, bad, or total service filter must be defined (where
    /// good + bad = total is assumed)
    /// Must have ValueType = DOUBLE or ValueType = INT64 and
    /// must have MetricKind = DELTA or MetricKind = CUMULATIVE.
    #[builder(into)]
    pub r#good_service_filter: Option<String>,
    /// A TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// quantifying total demanded service. Exactly two of
    /// good, bad, or total service filter must be defined (where
    /// good + bad = total is assumed)
    /// Must have ValueType = DOUBLE or ValueType = INT64 and
    /// must have MetricKind = DELTA or MetricKind = CUMULATIVE.
    #[builder(into)]
    pub r#total_service_filter: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SloRequestBasedSliGoodTotalRatio {
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
                    "badServiceFilter",
                    &self.r#bad_service_filter,
                ),
                to_pulumi_object_field(
                    "goodServiceFilter",
                    &self.r#good_service_filter,
                ),
                to_pulumi_object_field(
                    "totalServiceFilter",
                    &self.r#total_service_filter,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SloRequestBasedSliGoodTotalRatio {
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
                    r#bad_service_filter: {
                        let field_value = match fields_map.get("badServiceFilter") {
                            Some(value) => value,
                            None => bail!("Missing field 'badServiceFilter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#good_service_filter: {
                        let field_value = match fields_map.get("goodServiceFilter") {
                            Some(value) => value,
                            None => bail!("Missing field 'goodServiceFilter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#total_service_filter: {
                        let field_value = match fields_map.get("totalServiceFilter") {
                            Some(value) => value,
                            None => bail!("Missing field 'totalServiceFilter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
