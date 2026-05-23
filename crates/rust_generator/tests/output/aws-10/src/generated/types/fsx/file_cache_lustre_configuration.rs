#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FileCacheLustreConfiguration {
    /// Specifies the cache deployment type. The only supported value is `CACHE_1`.
    #[builder(into)]
    pub r#deployment_type: String,
    #[builder(into)]
    pub r#log_configurations: Option<Vec<super::super::types::fsx::FileCacheLustreConfigurationLogConfiguration>>,
    /// The configuration for a Lustre MDT (Metadata Target) storage volume. See the `metadata_configuration` block.
    #[builder(into)]
    pub r#metadata_configurations: Vec<super::super::types::fsx::FileCacheLustreConfigurationMetadataConfiguration>,
    #[builder(into)]
    pub r#mount_name: Option<String>,
    /// Provisions the amount of read and write throughput for each 1 tebibyte (TiB) of cache storage capacity, in MB/s/TiB. The only supported value is `1000`.
    #[builder(into)]
    pub r#per_unit_storage_throughput: i32,
    /// A recurring weekly time, in the format `D:HH:MM`. `D` is the day of the week, for which `1` represents Monday and `7` represents Sunday. `HH` is the zero-padded hour of the day (0-23), and `MM` is the zero-padded minute of the hour. For example, 1:05:00 specifies maintenance at 5 AM Monday. See the [ISO week date](https://en.wikipedia.org/wiki/ISO_week_date) for more information.
    #[builder(into)]
    pub r#weekly_maintenance_start_time: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FileCacheLustreConfiguration {
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
                    "deploymentType",
                    &self.r#deployment_type,
                ),
                to_pulumi_object_field(
                    "logConfigurations",
                    &self.r#log_configurations,
                ),
                to_pulumi_object_field(
                    "metadataConfigurations",
                    &self.r#metadata_configurations,
                ),
                to_pulumi_object_field(
                    "mountName",
                    &self.r#mount_name,
                ),
                to_pulumi_object_field(
                    "perUnitStorageThroughput",
                    &self.r#per_unit_storage_throughput,
                ),
                to_pulumi_object_field(
                    "weeklyMaintenanceStartTime",
                    &self.r#weekly_maintenance_start_time,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("deploymentType") {
                            Some(value) => value,
                            None => bail!("Missing field 'deploymentType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_configurations: {
                        let field_value = match fields_map.get("logConfigurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'logConfigurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metadata_configurations: {
                        let field_value = match fields_map.get("metadataConfigurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'metadataConfigurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mount_name: {
                        let field_value = match fields_map.get("mountName") {
                            Some(value) => value,
                            None => bail!("Missing field 'mountName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#per_unit_storage_throughput: {
                        let field_value = match fields_map.get("perUnitStorageThroughput") {
                            Some(value) => value,
                            None => bail!("Missing field 'perUnitStorageThroughput' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weekly_maintenance_start_time: {
                        let field_value = match fields_map.get("weeklyMaintenanceStartTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'weeklyMaintenanceStartTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
