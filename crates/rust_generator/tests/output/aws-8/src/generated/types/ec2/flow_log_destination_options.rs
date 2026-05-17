#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowLogDestinationOptions {
    /// The format for the flow log. Default value: `plain-text`. Valid values: `plain-text`, `parquet`.
    #[builder(into)]
    #[serde(rename = "fileFormat")]
    pub r#file_format: Option<String>,
    /// Indicates whether to use Hive-compatible prefixes for flow logs stored in Amazon S3. Default value: `false`.
    #[builder(into)]
    #[serde(rename = "hiveCompatiblePartitions")]
    pub r#hive_compatible_partitions: Option<bool>,
    /// Indicates whether to partition the flow log per hour. This reduces the cost and response time for queries. Default value: `false`.
    #[builder(into)]
    #[serde(rename = "perHourPartition")]
    pub r#per_hour_partition: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlowLogDestinationOptions {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "file_format",
                    &self.r#file_format,
                ),
                to_pulumi_object_field(
                    "hive_compatible_partitions",
                    &self.r#hive_compatible_partitions,
                ),
                to_pulumi_object_field(
                    "per_hour_partition",
                    &self.r#per_hour_partition,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlowLogDestinationOptions {
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
                    r#file_format: {
                        let field_value = match fields_map.get("file_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hive_compatible_partitions: {
                        let field_value = match fields_map.get("hive_compatible_partitions") {
                            Some(value) => value,
                            None => bail!("Missing field 'hive_compatible_partitions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#per_hour_partition: {
                        let field_value = match fields_map.get("per_hour_partition") {
                            Some(value) => value,
                            None => bail!("Missing field 'per_hour_partition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
