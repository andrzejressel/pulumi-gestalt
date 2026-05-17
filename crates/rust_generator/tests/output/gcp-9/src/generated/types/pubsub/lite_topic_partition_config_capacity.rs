#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LiteTopicPartitionConfigCapacity {
    /// Subscribe throughput capacity per partition in MiB/s. Must be >= 4 and <= 16.
    #[builder(into)]
    #[serde(rename = "publishMibPerSec")]
    pub r#publish_mib_per_sec: i32,
    /// Publish throughput capacity per partition in MiB/s. Must be >= 4 and <= 16.
    #[builder(into)]
    #[serde(rename = "subscribeMibPerSec")]
    pub r#subscribe_mib_per_sec: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LiteTopicPartitionConfigCapacity {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "publish_mib_per_sec".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#publish_mib_per_sec,
                )
                .await,
            );
            map.insert(
                "subscribe_mib_per_sec".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subscribe_mib_per_sec,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LiteTopicPartitionConfigCapacity {
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
                    r#publish_mib_per_sec: {
                        let field_value = match fields_map.get("publish_mib_per_sec") {
                            Some(value) => value,
                            None => bail!("Missing field 'publish_mib_per_sec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subscribe_mib_per_sec: {
                        let field_value = match fields_map.get("subscribe_mib_per_sec") {
                            Some(value) => value,
                            None => bail!("Missing field 'subscribe_mib_per_sec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
