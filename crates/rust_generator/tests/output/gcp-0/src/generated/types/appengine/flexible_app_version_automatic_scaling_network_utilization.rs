#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlexibleAppVersionAutomaticScalingNetworkUtilization {
    /// Target bytes received per second.
    #[builder(into)]
    #[serde(rename = "targetReceivedBytesPerSecond")]
    pub r#target_received_bytes_per_second: Option<i32>,
    /// Target packets received per second.
    #[builder(into)]
    #[serde(rename = "targetReceivedPacketsPerSecond")]
    pub r#target_received_packets_per_second: Option<i32>,
    /// Target bytes sent per second.
    #[builder(into)]
    #[serde(rename = "targetSentBytesPerSecond")]
    pub r#target_sent_bytes_per_second: Option<i32>,
    /// Target packets sent per second.
    #[builder(into)]
    #[serde(rename = "targetSentPacketsPerSecond")]
    pub r#target_sent_packets_per_second: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlexibleAppVersionAutomaticScalingNetworkUtilization {
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
                    "target_received_bytes_per_second",
                    &self.r#target_received_bytes_per_second,
                ),
                to_pulumi_object_field(
                    "target_received_packets_per_second",
                    &self.r#target_received_packets_per_second,
                ),
                to_pulumi_object_field(
                    "target_sent_bytes_per_second",
                    &self.r#target_sent_bytes_per_second,
                ),
                to_pulumi_object_field(
                    "target_sent_packets_per_second",
                    &self.r#target_sent_packets_per_second,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlexibleAppVersionAutomaticScalingNetworkUtilization {
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
                    r#target_received_bytes_per_second: {
                        let field_value = match fields_map.get("target_received_bytes_per_second") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_received_bytes_per_second' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_received_packets_per_second: {
                        let field_value = match fields_map.get("target_received_packets_per_second") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_received_packets_per_second' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_sent_bytes_per_second: {
                        let field_value = match fields_map.get("target_sent_bytes_per_second") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_sent_bytes_per_second' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_sent_packets_per_second: {
                        let field_value = match fields_map.get("target_sent_packets_per_second") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_sent_packets_per_second' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
