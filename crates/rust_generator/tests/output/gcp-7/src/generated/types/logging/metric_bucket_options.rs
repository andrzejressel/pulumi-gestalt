#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MetricBucketOptions {
    /// Specifies a set of buckets with arbitrary widths.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "explicitBuckets")]
    pub r#explicit_buckets: Option<Box<super::super::types::logging::MetricBucketOptionsExplicitBuckets>>,
    /// Specifies an exponential sequence of buckets that have a width that is proportional to the value of
    /// the lower bound. Each bucket represents a constant relative uncertainty on a specific value in the bucket.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "exponentialBuckets")]
    pub r#exponential_buckets: Option<Box<super::super::types::logging::MetricBucketOptionsExponentialBuckets>>,
    /// Specifies a linear sequence of buckets that all have the same width (except overflow and underflow).
    /// Each bucket represents a constant absolute uncertainty on the specific value in the bucket.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "linearBuckets")]
    pub r#linear_buckets: Option<Box<super::super::types::logging::MetricBucketOptionsLinearBuckets>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MetricBucketOptions {
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
                    "explicit_buckets",
                    &self.r#explicit_buckets,
                ),
                to_pulumi_object_field(
                    "exponential_buckets",
                    &self.r#exponential_buckets,
                ),
                to_pulumi_object_field(
                    "linear_buckets",
                    &self.r#linear_buckets,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MetricBucketOptions {
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
                    r#explicit_buckets: {
                        let field_value = match fields_map.get("explicit_buckets") {
                            Some(value) => value,
                            None => bail!("Missing field 'explicit_buckets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exponential_buckets: {
                        let field_value = match fields_map.get("exponential_buckets") {
                            Some(value) => value,
                            None => bail!("Missing field 'exponential_buckets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#linear_buckets: {
                        let field_value = match fields_map.get("linear_buckets") {
                            Some(value) => value,
                            None => bail!("Missing field 'linear_buckets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
