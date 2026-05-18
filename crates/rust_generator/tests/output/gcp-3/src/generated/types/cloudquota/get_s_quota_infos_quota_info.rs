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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "container_type",
                    &self.r#container_type,
                ),
                to_pulumi_object_field(
                    "dimensions",
                    &self.r#dimensions,
                ),
                to_pulumi_object_field(
                    "dimensions_infos",
                    &self.r#dimensions_infos,
                ),
                to_pulumi_object_field(
                    "is_concurrent",
                    &self.r#is_concurrent,
                ),
                to_pulumi_object_field(
                    "is_fixed",
                    &self.r#is_fixed,
                ),
                to_pulumi_object_field(
                    "is_precise",
                    &self.r#is_precise,
                ),
                to_pulumi_object_field(
                    "metric",
                    &self.r#metric,
                ),
                to_pulumi_object_field(
                    "metric_display_name",
                    &self.r#metric_display_name,
                ),
                to_pulumi_object_field(
                    "metric_unit",
                    &self.r#metric_unit,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "quota_display_name",
                    &self.r#quota_display_name,
                ),
                to_pulumi_object_field(
                    "quota_id",
                    &self.r#quota_id,
                ),
                to_pulumi_object_field(
                    "quota_increase_eligibilities",
                    &self.r#quota_increase_eligibilities,
                ),
                to_pulumi_object_field(
                    "refresh_interval",
                    &self.r#refresh_interval,
                ),
                to_pulumi_object_field(
                    "service",
                    &self.r#service,
                ),
                to_pulumi_object_field(
                    "service_request_quota_uri",
                    &self.r#service_request_quota_uri,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
