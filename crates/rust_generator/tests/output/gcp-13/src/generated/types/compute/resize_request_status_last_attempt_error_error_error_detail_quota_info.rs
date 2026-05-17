#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResizeRequestStatusLastAttemptErrorErrorErrorDetailQuotaInfo {
    /// (Output)
    /// The map holding related quota dimensions
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Option<std::collections::HashMap<String, String>>,
    /// (Output)
    /// Future quota limit being rolled out. The limit's unit depends on the quota type or metric.
    #[builder(into)]
    #[serde(rename = "futureLimit")]
    pub r#future_limit: Option<i32>,
    /// (Output)
    /// Current effective quota limit. The limit's unit depends on the quota type or metric.
    #[builder(into)]
    #[serde(rename = "limit")]
    pub r#limit: Option<i32>,
    /// (Output)
    /// The name of the quota limit.
    #[builder(into)]
    #[serde(rename = "limitName")]
    pub r#limit_name: Option<String>,
    /// (Output)
    /// The Compute Engine quota metric name.
    #[builder(into)]
    #[serde(rename = "metricName")]
    pub r#metric_name: Option<String>,
    /// (Output)
    /// Rollout status of the future quota limit.
    #[builder(into)]
    #[serde(rename = "rolloutStatus")]
    pub r#rollout_status: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ResizeRequestStatusLastAttemptErrorErrorErrorDetailQuotaInfo {
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
                "dimensions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dimensions,
                )
                .await,
            );
            map.insert(
                "future_limit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#future_limit,
                )
                .await,
            );
            map.insert(
                "limit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#limit,
                )
                .await,
            );
            map.insert(
                "limit_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#limit_name,
                )
                .await,
            );
            map.insert(
                "metric_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metric_name,
                )
                .await,
            );
            map.insert(
                "rollout_status".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rollout_status,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ResizeRequestStatusLastAttemptErrorErrorErrorDetailQuotaInfo {
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
                    r#dimensions: {
                        let field_value = match fields_map.get("dimensions") {
                            Some(value) => value,
                            None => bail!("Missing field 'dimensions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#future_limit: {
                        let field_value = match fields_map.get("future_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'future_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#limit: {
                        let field_value = match fields_map.get("limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#limit_name: {
                        let field_value = match fields_map.get("limit_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'limit_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric_name: {
                        let field_value = match fields_map.get("metric_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rollout_status: {
                        let field_value = match fields_map.get("rollout_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'rollout_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
