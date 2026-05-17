#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetEnvironmentConfigWorkloadsConfigWorker {
    /// CPU request and limit for a single Airflow worker replica.
    #[builder(into)]
    #[serde(rename = "cpu")]
    pub r#cpu: f64,
    /// Maximum number of workers for autoscaling.
    #[builder(into)]
    #[serde(rename = "maxCount")]
    pub r#max_count: i32,
    /// Memory (GB) request and limit for a single Airflow worker replica.
    #[builder(into)]
    #[serde(rename = "memoryGb")]
    pub r#memory_gb: f64,
    /// Minimum number of workers for autoscaling.
    #[builder(into)]
    #[serde(rename = "minCount")]
    pub r#min_count: i32,
    /// Storage (GB) request and limit for a single Airflow worker replica.
    #[builder(into)]
    #[serde(rename = "storageGb")]
    pub r#storage_gb: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetEnvironmentConfigWorkloadsConfigWorker {
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
                "cpu".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cpu,
                )
                .await,
            );
            map.insert(
                "max_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_count,
                )
                .await,
            );
            map.insert(
                "memory_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#memory_gb,
                )
                .await,
            );
            map.insert(
                "min_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_count,
                )
                .await,
            );
            map.insert(
                "storage_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_gb,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetEnvironmentConfigWorkloadsConfigWorker {
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
                    r#cpu: {
                        let field_value = match fields_map.get("cpu") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_count: {
                        let field_value = match fields_map.get("max_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_gb: {
                        let field_value = match fields_map.get("memory_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_count: {
                        let field_value = match fields_map.get("min_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_gb: {
                        let field_value = match fields_map.get("storage_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
