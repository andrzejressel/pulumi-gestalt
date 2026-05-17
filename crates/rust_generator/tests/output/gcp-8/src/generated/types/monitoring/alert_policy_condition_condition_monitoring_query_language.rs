#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertPolicyConditionConditionMonitoringQueryLanguage {
    /// The amount of time that a time series must
    /// violate the threshold to be considered
    /// failing. Currently, only values that are a
    /// multiple of a minute--e.g., 0, 60, 120, or
    /// 300 seconds--are supported. If an invalid
    /// value is given, an error will be returned.
    /// When choosing a duration, it is useful to
    /// keep in mind the frequency of the underlying
    /// time series data (which may also be affected
    /// by any alignments specified in the
    /// aggregations field); a good duration is long
    /// enough so that a single outlier does not
    /// generate spurious alerts, but short enough
    /// that unhealthy states are detected and
    /// alerted on quickly.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: String,
    /// A condition control that determines how
    /// metric-threshold conditions are evaluated when
    /// data stops arriving.
    /// Possible values are: `EVALUATION_MISSING_DATA_INACTIVE`, `EVALUATION_MISSING_DATA_ACTIVE`, `EVALUATION_MISSING_DATA_NO_OP`.
    #[builder(into)]
    #[serde(rename = "evaluationMissingData")]
    pub r#evaluation_missing_data: Option<String>,
    /// Monitoring Query Language query that outputs a boolean stream.
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: String,
    /// The number/percent of time series for which
    /// the comparison must hold in order for the
    /// condition to trigger. If unspecified, then
    /// the condition will trigger if the comparison
    /// is true for any of the time series that have
    /// been identified by filter and aggregations,
    /// or by the ratio, if denominator_filter and
    /// denominator_aggregations are specified.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "trigger")]
    pub r#trigger: Option<Box<super::super::types::monitoring::AlertPolicyConditionConditionMonitoringQueryLanguageTrigger>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AlertPolicyConditionConditionMonitoringQueryLanguage {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "duration",
                    &self.r#duration,
                ),
                to_pulumi_object_field(
                    "evaluation_missing_data",
                    &self.r#evaluation_missing_data,
                ),
                to_pulumi_object_field(
                    "query",
                    &self.r#query,
                ),
                to_pulumi_object_field(
                    "trigger",
                    &self.r#trigger,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AlertPolicyConditionConditionMonitoringQueryLanguage {
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
                    r#duration: {
                        let field_value = match fields_map.get("duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#evaluation_missing_data: {
                        let field_value = match fields_map.get("evaluation_missing_data") {
                            Some(value) => value,
                            None => bail!("Missing field 'evaluation_missing_data' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query: {
                        let field_value = match fields_map.get("query") {
                            Some(value) => value,
                            None => bail!("Missing field 'query' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
