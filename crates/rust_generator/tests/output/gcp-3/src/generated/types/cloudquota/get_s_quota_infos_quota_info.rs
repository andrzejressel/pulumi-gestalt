#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSQuotaInfosQuotaInfo {
    /// (Output) The container type of the QuotaInfo.
    #[builder(into)]
    #[serde(rename = "containerType")]
    pub r#container_type: String,
    /// The map of dimensions for this dimensions info. The key of a map entry is "region", "zone" or the name of a service specific dimension, and the value of a map entry is the value of the dimension. If a dimension does not appear in the map of dimensions, the dimensions info applies to all the dimension values except for those that have another DimenisonInfo instance configured for the specific value. Example: {"provider" : "Foo Inc"} where "provider" is a service specific dimension of a quota.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Vec<String>,
    /// (Output) The collection of dimensions info ordered by their dimensions from more specific ones to less specific ones.
    #[builder(into)]
    #[serde(rename = "dimensionsInfos")]
    pub r#dimensions_infos: Vec<super::super::types::cloudquota::GetSQuotaInfosQuotaInfoDimensionsInfo>,
    /// (Output) Whether the quota is a concurrent quota. Concurrent quotas are enforced on the total number of concurrent operations in flight at any given time.
    #[builder(into)]
    #[serde(rename = "isConcurrent")]
    pub r#is_concurrent: bool,
    /// (Output) Whether the quota value is fixed or adjustable.
    #[builder(into)]
    #[serde(rename = "isFixed")]
    pub r#is_fixed: bool,
    /// (Output) Whether this is a precise quota. A precise quota is tracked with absolute precision. In contrast, an imprecise quota is not tracked with precision.
    #[builder(into)]
    #[serde(rename = "isPrecise")]
    pub r#is_precise: bool,
    /// (Output) The metric of the quota. It specifies the resources consumption the quota is defined for, for example: `compute.googleapis.com/cpus`.
    #[builder(into)]
    #[serde(rename = "metric")]
    pub r#metric: String,
    /// (Output) The display name of the quota metric.
    #[builder(into)]
    #[serde(rename = "metricDisplayName")]
    pub r#metric_display_name: String,
    /// (Output) The unit in which the metric value is reported, e.g., `MByte`.
    #[builder(into)]
    #[serde(rename = "metricUnit")]
    pub r#metric_unit: String,
    /// (Output) Resource name of this QuotaInfo, for example: `projects/123/locations/global/services/compute.googleapis.com/quotaInfos/CpusPerProjectPerRegion`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// (Output) The display name of the quota.
    #[builder(into)]
    #[serde(rename = "quotaDisplayName")]
    pub r#quota_display_name: String,
    #[builder(into)]
    #[serde(rename = "quotaId")]
    pub r#quota_id: String,
    /// (Output) Whether it is eligible to request a higher quota value for this quota.
    #[builder(into)]
    #[serde(rename = "quotaIncreaseEligibilities")]
    pub r#quota_increase_eligibilities: Vec<super::super::types::cloudquota::GetSQuotaInfosQuotaInfoQuotaIncreaseEligibility>,
    /// (Output) The reset time interval for the quota. Refresh interval applies to rate quota only. Example: "minute" for per minute, "day" for per day, or "10 seconds" for every 10 seconds.
    #[builder(into)]
    #[serde(rename = "refreshInterval")]
    pub r#refresh_interval: String,
    /// The name of the service in which the quotas are defined.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: String,
    /// (Output) URI to the page where users can request more quota for the cloud service, for example: `https://console.cloud.google.com/iam-admin/quotas`.
    #[builder(into)]
    #[serde(rename = "serviceRequestQuotaUri")]
    pub r#service_request_quota_uri: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSQuotaInfosQuotaInfo {
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
                "container_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container_type,
                )
                .await,
            );
            map.insert(
                "dimensions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dimensions,
                )
                .await,
            );
            map.insert(
                "dimensions_infos".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dimensions_infos,
                )
                .await,
            );
            map.insert(
                "is_concurrent".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_concurrent,
                )
                .await,
            );
            map.insert(
                "is_fixed".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_fixed,
                )
                .await,
            );
            map.insert(
                "is_precise".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_precise,
                )
                .await,
            );
            map.insert(
                "metric".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metric,
                )
                .await,
            );
            map.insert(
                "metric_display_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metric_display_name,
                )
                .await,
            );
            map.insert(
                "metric_unit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metric_unit,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "quota_display_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#quota_display_name,
                )
                .await,
            );
            map.insert(
                "quota_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#quota_id,
                )
                .await,
            );
            map.insert(
                "quota_increase_eligibilities".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#quota_increase_eligibilities,
                )
                .await,
            );
            map.insert(
                "refresh_interval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#refresh_interval,
                )
                .await,
            );
            map.insert(
                "service".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service,
                )
                .await,
            );
            map.insert(
                "service_request_quota_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_request_quota_uri,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSQuotaInfosQuotaInfo {
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
                    r#container_type: {
                        let field_value = match fields_map.get("container_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dimensions: {
                        let field_value = match fields_map.get("dimensions") {
                            Some(value) => value,
                            None => bail!("Missing field 'dimensions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dimensions_infos: {
                        let field_value = match fields_map.get("dimensions_infos") {
                            Some(value) => value,
                            None => bail!("Missing field 'dimensions_infos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_concurrent: {
                        let field_value = match fields_map.get("is_concurrent") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_concurrent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_fixed: {
                        let field_value = match fields_map.get("is_fixed") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_fixed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_precise: {
                        let field_value = match fields_map.get("is_precise") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_precise' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric: {
                        let field_value = match fields_map.get("metric") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric_display_name: {
                        let field_value = match fields_map.get("metric_display_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_display_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric_unit: {
                        let field_value = match fields_map.get("metric_unit") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_unit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#quota_display_name: {
                        let field_value = match fields_map.get("quota_display_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'quota_display_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#quota_id: {
                        let field_value = match fields_map.get("quota_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'quota_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#quota_increase_eligibilities: {
                        let field_value = match fields_map.get("quota_increase_eligibilities") {
                            Some(value) => value,
                            None => bail!("Missing field 'quota_increase_eligibilities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#refresh_interval: {
                        let field_value = match fields_map.get("refresh_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'refresh_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service: {
                        let field_value = match fields_map.get("service") {
                            Some(value) => value,
                            None => bail!("Missing field 'service' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_request_quota_uri: {
                        let field_value = match fields_map.get("service_request_quota_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_request_quota_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
