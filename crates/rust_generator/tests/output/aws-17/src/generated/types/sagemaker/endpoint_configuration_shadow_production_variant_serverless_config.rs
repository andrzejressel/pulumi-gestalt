#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointConfigurationShadowProductionVariantServerlessConfig {
    /// The maximum number of concurrent invocations your serverless endpoint can process. Valid values are between `1` and `200`.
    #[builder(into)]
    #[serde(rename = "maxConcurrency")]
    pub r#max_concurrency: i32,
    /// The memory size of your serverless endpoint. Valid values are in 1 GB increments: `1024` MB, `2048` MB, `3072` MB, `4096` MB, `5120` MB, or `6144` MB.
    #[builder(into)]
    #[serde(rename = "memorySizeInMb")]
    pub r#memory_size_in_mb: i32,
    /// The amount of provisioned concurrency to allocate for the serverless endpoint. Should be less than or equal to `max_concurrency`. Valid values are between `1` and `200`.
    #[builder(into)]
    #[serde(rename = "provisionedConcurrency")]
    pub r#provisioned_concurrency: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointConfigurationShadowProductionVariantServerlessConfig {
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
                "max_concurrency".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_concurrency,
                )
                .await,
            );
            map.insert(
                "memory_size_in_mb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#memory_size_in_mb,
                )
                .await,
            );
            map.insert(
                "provisioned_concurrency".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#provisioned_concurrency,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointConfigurationShadowProductionVariantServerlessConfig {
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
                    r#max_concurrency: {
                        let field_value = match fields_map.get("max_concurrency") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_concurrency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_size_in_mb: {
                        let field_value = match fields_map.get("memory_size_in_mb") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory_size_in_mb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioned_concurrency: {
                        let field_value = match fields_map.get("provisioned_concurrency") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioned_concurrency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
