#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SloRequestBasedSli {
    /// Used when good_service is defined by a count of values aggregated in a
    /// Distribution that fall into a good range. The total_service is the
    /// total count of all values aggregated in the Distribution.
    /// Defines a distribution TimeSeries filter and thresholds used for
    /// measuring good service and total service.
    /// Exactly one of `distribution_cut` or `good_total_ratio` can be set.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "distributionCut")]
    pub r#distribution_cut: Option<Box<super::super::types::monitoring::SloRequestBasedSliDistributionCut>>,
    /// A means to compute a ratio of `good_service` to `total_service`.
    /// Defines computing this ratio with two TimeSeries [monitoring filters](https://cloud.google.com/monitoring/api/v3/filters)
    /// Must specify exactly two of good, bad, and total service filters.
    /// The relationship good_service + bad_service = total_service
    /// will be assumed.
    /// Exactly one of `distribution_cut` or `good_total_ratio` can be set.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "goodTotalRatio")]
    pub r#good_total_ratio: Option<Box<super::super::types::monitoring::SloRequestBasedSliGoodTotalRatio>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SloRequestBasedSli {
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
                    "distribution_cut",
                    &self.r#distribution_cut,
                ),
                to_pulumi_object_field(
                    "good_total_ratio",
                    &self.r#good_total_ratio,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SloRequestBasedSli {
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
                    r#distribution_cut: {
                        let field_value = match fields_map.get("distribution_cut") {
                            Some(value) => value,
                            None => bail!("Missing field 'distribution_cut' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#good_total_ratio: {
                        let field_value = match fields_map.get("good_total_ratio") {
                            Some(value) => value,
                            None => bail!("Missing field 'good_total_ratio' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
