#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServerlessCacheCacheUsageLimits {
    /// The maximum data storage limit in the cache, expressed in Gigabytes. See `data_storage` Block for details.
    #[builder(into)]
    #[serde(rename = "dataStorage")]
    pub r#data_storage: Option<Box<super::super::types::elasticache::ServerlessCacheCacheUsageLimitsDataStorage>>,
    /// The configuration for the number of ElastiCache Processing Units (ECPU) the cache can consume per second. See `ecpu_per_second` Block for details.
    #[builder(into)]
    #[serde(rename = "ecpuPerSeconds")]
    pub r#ecpu_per_seconds: Option<Vec<super::super::types::elasticache::ServerlessCacheCacheUsageLimitsEcpuPerSecond>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServerlessCacheCacheUsageLimits {
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
                "data_storage".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_storage,
                )
                .await,
            );
            map.insert(
                "ecpu_per_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ecpu_per_seconds,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServerlessCacheCacheUsageLimits {
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
                    r#data_storage: {
                        let field_value = match fields_map.get("data_storage") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_storage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ecpu_per_seconds: {
                        let field_value = match fields_map.get("ecpu_per_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'ecpu_per_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
