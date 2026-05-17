#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertPolicyConditionConditionAbsent {
    /// Specifies the alignment of data points in
    /// individual time series as well as how to
    /// combine the retrieved time series together
    /// (such as when aggregating multiple streams
    /// on each resource to a single stream for each
    /// resource or when aggregating streams across
    /// all members of a group of resources).
    /// Multiple aggregations are applied in the
    /// order specified.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "aggregations")]
    pub r#aggregations: Option<Vec<super::super::types::monitoring::AlertPolicyConditionConditionAbsentAggregation>>,
    /// The amount of time that a time series must
    /// fail to report new data to be considered
    /// failing. Currently, only values that are a
    /// multiple of a minute--e.g. 60s, 120s, or 300s
    /// --are supported.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: String,
    /// A filter that identifies which time series
    /// should be compared with the threshold.The
    /// filter is similar to the one that is
    /// specified in the
    /// MetricService.ListTimeSeries request (that
    /// call is useful to verify the time series
    /// that will be retrieved / processed) and must
    /// specify the metric type and optionally may
    /// contain restrictions on resource type,
    /// resource labels, and metric labels. This
    /// field may not exceed 2048 Unicode characters
    /// in length.
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: Option<String>,
    /// The number/percent of time series for which
    /// the comparison must hold in order for the
    /// condition to trigger. If unspecified, then
    /// the condition will trigger if the comparison
    /// is true for any of the time series that have
    /// been identified by filter and aggregations.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "trigger")]
    pub r#trigger: Option<Box<super::super::types::monitoring::AlertPolicyConditionConditionAbsentTrigger>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AlertPolicyConditionConditionAbsent {
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
                "aggregations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#aggregations,
                )
                .await,
            );
            map.insert(
                "duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#duration,
                )
                .await,
            );
            map.insert(
                "filter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#filter,
                )
                .await,
            );
            map.insert(
                "trigger".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#trigger,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AlertPolicyConditionConditionAbsent {
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
                    r#aggregations: {
                        let field_value = match fields_map.get("aggregations") {
                            Some(value) => value,
                            None => bail!("Missing field 'aggregations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#duration: {
                        let field_value = match fields_map.get("duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter: {
                        let field_value = match fields_map.get("filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trigger: {
                        let field_value = match fields_map.get("trigger") {
                            Some(value) => value,
                            None => bail!("Missing field 'trigger' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
