#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketLoggingV2TargetObjectKeyFormat {
    /// Partitioned S3 key for log objects. See below.
    #[builder(into)]
    #[serde(rename = "partitionedPrefix")]
    pub r#partitioned_prefix: Option<Box<super::super::types::s3::BucketLoggingV2TargetObjectKeyFormatPartitionedPrefix>>,
    /// Use the simple format for S3 keys for log objects. To use, set `simple_prefix {}`.
    #[builder(into)]
    #[serde(rename = "simplePrefix")]
    pub r#simple_prefix: Option<Box<super::super::types::s3::BucketLoggingV2TargetObjectKeyFormatSimplePrefix>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BucketLoggingV2TargetObjectKeyFormat {
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
                "partitioned_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#partitioned_prefix,
                )
                .await,
            );
            map.insert(
                "simple_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#simple_prefix,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BucketLoggingV2TargetObjectKeyFormat {
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
                    r#partitioned_prefix: {
                        let field_value = match fields_map.get("partitioned_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'partitioned_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#simple_prefix: {
                        let field_value = match fields_map.get("simple_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'simple_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
