#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FileCacheLustreConfiguration {
    /// Specifies the cache deployment type. The only supported value is `CACHE_1`.
    #[builder(into)]
    #[serde(rename = "deploymentType")]
    pub r#deployment_type: String,
    #[builder(into)]
    #[serde(rename = "logConfigurations")]
    pub r#log_configurations: Option<Vec<super::super::types::fsx::FileCacheLustreConfigurationLogConfiguration>>,
    /// The configuration for a Lustre MDT (Metadata Target) storage volume. See the `metadata_configuration` block.
    #[builder(into)]
    #[serde(rename = "metadataConfigurations")]
    pub r#metadata_configurations: Vec<super::super::types::fsx::FileCacheLustreConfigurationMetadataConfiguration>,
    #[builder(into)]
    #[serde(rename = "mountName")]
    pub r#mount_name: Option<String>,
    /// Provisions the amount of read and write throughput for each 1 tebibyte (TiB) of cache storage capacity, in MB/s/TiB. The only supported value is `1000`.
    #[builder(into)]
    #[serde(rename = "perUnitStorageThroughput")]
    pub r#per_unit_storage_throughput: i32,
    /// A recurring weekly time, in the format `D:HH:MM`. `D` is the day of the week, for which `1` represents Monday and `7` represents Sunday. `HH` is the zero-padded hour of the day (0-23), and `MM` is the zero-padded minute of the hour. For example, 1:05:00 specifies maintenance at 5 AM Monday. See the [ISO week date](https://en.wikipedia.org/wiki/ISO_week_date) for more information.
    #[builder(into)]
    #[serde(rename = "weeklyMaintenanceStartTime")]
    pub r#weekly_maintenance_start_time: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FileCacheLustreConfiguration {
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
                "deployment_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#deployment_type,
                )
                .await,
            );
            map.insert(
                "log_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#log_configurations,
                )
                .await,
            );
            map.insert(
                "metadata_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metadata_configurations,
                )
                .await,
            );
            map.insert(
                "mount_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mount_name,
                )
                .await,
            );
            map.insert(
                "per_unit_storage_throughput".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#per_unit_storage_throughput,
                )
                .await,
            );
            map.insert(
                "weekly_maintenance_start_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#weekly_maintenance_start_time,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FileCacheLustreConfiguration {
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
                    r#deployment_type: {
                        let field_value = match fields_map.get("deployment_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'deployment_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_configurations: {
                        let field_value = match fields_map.get("log_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metadata_configurations: {
                        let field_value = match fields_map.get("metadata_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'metadata_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mount_name: {
                        let field_value = match fields_map.get("mount_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'mount_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#per_unit_storage_throughput: {
                        let field_value = match fields_map.get("per_unit_storage_throughput") {
                            Some(value) => value,
                            None => bail!("Missing field 'per_unit_storage_throughput' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weekly_maintenance_start_time: {
                        let field_value = match fields_map.get("weekly_maintenance_start_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'weekly_maintenance_start_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
