#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkgroupConfiguration {
    /// Integer for the upper data usage limit (cutoff) for the amount of bytes a single query in a workgroup is allowed to scan. Must be at least `10485760`.
    #[builder(into)]
    #[serde(rename = "bytesScannedCutoffPerQuery")]
    pub r#bytes_scanned_cutoff_per_query: Option<i32>,
    /// Boolean whether the settings for the workgroup override client-side settings. For more information, see [Workgroup Settings Override Client-Side Settings](https://docs.aws.amazon.com/athena/latest/ug/workgroups-settings-override.html). Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enforceWorkgroupConfiguration")]
    pub r#enforce_workgroup_configuration: Option<bool>,
    /// Configuration block for the Athena Engine Versioning. For more information, see [Athena Engine Versioning](https://docs.aws.amazon.com/athena/latest/ug/engine-versions.html). See Engine Version below.
    #[builder(into)]
    #[serde(rename = "engineVersion")]
    pub r#engine_version: Option<Box<super::super::types::athena::WorkgroupConfigurationEngineVersion>>,
    /// Role used in a notebook session for accessing the user's resources.
    #[builder(into)]
    #[serde(rename = "executionRole")]
    pub r#execution_role: Option<String>,
    /// Boolean whether Amazon CloudWatch metrics are enabled for the workgroup. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "publishCloudwatchMetricsEnabled")]
    pub r#publish_cloudwatch_metrics_enabled: Option<bool>,
    /// If set to true , allows members assigned to a workgroup to reference Amazon S3 Requester Pays buckets in queries. If set to false , workgroup members cannot query data from Requester Pays buckets, and queries that retrieve data from Requester Pays buckets cause an error. The default is false . For more information about Requester Pays buckets, see [Requester Pays Buckets](https://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html) in the Amazon Simple Storage Service Developer Guide.
    #[builder(into)]
    #[serde(rename = "requesterPaysEnabled")]
    pub r#requester_pays_enabled: Option<bool>,
    /// Configuration block with result settings. See Result Configuration below.
    #[builder(into)]
    #[serde(rename = "resultConfiguration")]
    pub r#result_configuration: Option<Box<super::super::types::athena::WorkgroupConfigurationResultConfiguration>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkgroupConfiguration {
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
                    "bytes_scanned_cutoff_per_query",
                    &self.r#bytes_scanned_cutoff_per_query,
                ),
                to_pulumi_object_field(
                    "enforce_workgroup_configuration",
                    &self.r#enforce_workgroup_configuration,
                ),
                to_pulumi_object_field(
                    "engine_version",
                    &self.r#engine_version,
                ),
                to_pulumi_object_field(
                    "execution_role",
                    &self.r#execution_role,
                ),
                to_pulumi_object_field(
                    "publish_cloudwatch_metrics_enabled",
                    &self.r#publish_cloudwatch_metrics_enabled,
                ),
                to_pulumi_object_field(
                    "requester_pays_enabled",
                    &self.r#requester_pays_enabled,
                ),
                to_pulumi_object_field(
                    "result_configuration",
                    &self.r#result_configuration,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkgroupConfiguration {
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
                    r#bytes_scanned_cutoff_per_query: {
                        let field_value = match fields_map.get("bytes_scanned_cutoff_per_query") {
                            Some(value) => value,
                            None => bail!("Missing field 'bytes_scanned_cutoff_per_query' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enforce_workgroup_configuration: {
                        let field_value = match fields_map.get("enforce_workgroup_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'enforce_workgroup_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#engine_version: {
                        let field_value = match fields_map.get("engine_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'engine_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#execution_role: {
                        let field_value = match fields_map.get("execution_role") {
                            Some(value) => value,
                            None => bail!("Missing field 'execution_role' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#publish_cloudwatch_metrics_enabled: {
                        let field_value = match fields_map.get("publish_cloudwatch_metrics_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'publish_cloudwatch_metrics_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#requester_pays_enabled: {
                        let field_value = match fields_map.get("requester_pays_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'requester_pays_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#result_configuration: {
                        let field_value = match fields_map.get("result_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'result_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
