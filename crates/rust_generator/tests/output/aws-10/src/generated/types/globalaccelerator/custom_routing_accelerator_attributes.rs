#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomRoutingAcceleratorAttributes {
    /// Indicates whether flow logs are enabled. Defaults to `false`. Valid values: `true`, `false`.
    #[builder(into)]
    pub r#flow_logs_enabled: Option<bool>,
    /// The name of the Amazon S3 bucket for the flow logs. Required if `flow_logs_enabled` is `true`.
    #[builder(into)]
    pub r#flow_logs_s_3_bucket: Option<String>,
    /// The prefix for the location in the Amazon S3 bucket for the flow logs. Required if `flow_logs_enabled` is `true`.
    #[builder(into)]
    pub r#flow_logs_s_3_prefix: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CustomRoutingAcceleratorAttributes {
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
                    "flowLogsEnabled",
                    &self.r#flow_logs_enabled,
                ),
                to_pulumi_object_field(
                    "flowLogsS3Bucket",
                    &self.r#flow_logs_s_3_bucket,
                ),
                to_pulumi_object_field(
                    "flowLogsS3Prefix",
                    &self.r#flow_logs_s_3_prefix,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CustomRoutingAcceleratorAttributes {
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
                    r#flow_logs_enabled: {
                        let field_value = match fields_map.get("flowLogsEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'flowLogsEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#flow_logs_s_3_bucket: {
                        let field_value = match fields_map.get("flowLogsS3Bucket") {
                            Some(value) => value,
                            None => bail!("Missing field 'flowLogsS3Bucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#flow_logs_s_3_prefix: {
                        let field_value = match fields_map.get("flowLogsS3Prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'flowLogsS3Prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
