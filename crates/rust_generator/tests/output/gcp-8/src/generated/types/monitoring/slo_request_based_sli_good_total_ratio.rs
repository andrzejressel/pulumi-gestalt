#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
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
    #[serde(rename = "badServiceFilter")]
    pub r#bad_service_filter: Option<String>,
    /// A TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// quantifying good service provided. Exactly two of
    /// good, bad, or total service filter must be defined (where
    /// good + bad = total is assumed)
    /// Must have ValueType = DOUBLE or ValueType = INT64 and
    /// must have MetricKind = DELTA or MetricKind = CUMULATIVE.
    #[builder(into)]
    #[serde(rename = "goodServiceFilter")]
    pub r#good_service_filter: Option<String>,
    /// A TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// quantifying total demanded service. Exactly two of
    /// good, bad, or total service filter must be defined (where
    /// good + bad = total is assumed)
    /// Must have ValueType = DOUBLE or ValueType = INT64 and
    /// must have MetricKind = DELTA or MetricKind = CUMULATIVE.
    #[builder(into)]
    #[serde(rename = "totalServiceFilter")]
    pub r#total_service_filter: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SloRequestBasedSliGoodTotalRatio {
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
                "bad_service_filter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bad_service_filter,
                )
                .await,
            );
            map.insert(
                "good_service_filter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#good_service_filter,
                )
                .await,
            );
            map.insert(
                "total_service_filter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#total_service_filter,
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
                        let field_value = match fields_map.get("bad_service_filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'bad_service_filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#good_service_filter: {
                        let field_value = match fields_map.get("good_service_filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'good_service_filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#total_service_filter: {
                        let field_value = match fields_map.get("total_service_filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'total_service_filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
