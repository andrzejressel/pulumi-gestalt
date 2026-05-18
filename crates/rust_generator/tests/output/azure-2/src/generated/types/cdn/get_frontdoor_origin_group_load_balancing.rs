#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetFrontdoorOriginGroupLoadBalancing {
    /// Specifies the additional latency in milliseconds for probes to fall into the lowest latency bucket.
    #[builder(into)]
    #[serde(rename = "additionalLatencyInMilliseconds")]
    pub r#additional_latency_in_milliseconds: i32,
    /// Specifies the number of samples to consider for load balancing decisions.
    #[builder(into)]
    #[serde(rename = "sampleSize")]
    pub r#sample_size: i32,
    /// Specifies the number of samples within the sample period that must succeed.
    #[builder(into)]
    #[serde(rename = "successfulSamplesRequired")]
    pub r#successful_samples_required: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetFrontdoorOriginGroupLoadBalancing {
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
                    "additional_latency_in_milliseconds",
                    &self.r#additional_latency_in_milliseconds,
                ),
                to_pulumi_object_field(
                    "sample_size",
                    &self.r#sample_size,
                ),
                to_pulumi_object_field(
                    "successful_samples_required",
                    &self.r#successful_samples_required,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetFrontdoorOriginGroupLoadBalancing {
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
                    r#additional_latency_in_milliseconds: {
                        let field_value = match fields_map.get("additional_latency_in_milliseconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_latency_in_milliseconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sample_size: {
                        let field_value = match fields_map.get("sample_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'sample_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#successful_samples_required: {
                        let field_value = match fields_map.get("successful_samples_required") {
                            Some(value) => value,
                            None => bail!("Missing field 'successful_samples_required' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
