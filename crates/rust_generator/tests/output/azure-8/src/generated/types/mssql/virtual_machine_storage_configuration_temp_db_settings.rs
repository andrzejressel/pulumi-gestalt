#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineStorageConfigurationTempDbSettings {
    /// The SQL Server default file count. This value defaults to `8`
    #[builder(into)]
    pub r#data_file_count: Option<i32>,
    /// The SQL Server default file size - This value defaults to `512`
    #[builder(into)]
    pub r#data_file_growth_in_mb: Option<i32>,
    /// The SQL Server default file size - This value defaults to `256`
    #[builder(into)]
    pub r#data_file_size_mb: Option<i32>,
    /// The SQL Server default path
    #[builder(into)]
    pub r#default_file_path: String,
    /// The SQL Server default file size - This value defaults to `512`
    #[builder(into)]
    pub r#log_file_growth_mb: Option<i32>,
    /// The SQL Server default file size - This value defaults to `256`
    #[builder(into)]
    pub r#log_file_size_mb: Option<i32>,
    /// A list of Logical Unit Numbers for the disks.
    #[builder(into)]
    pub r#luns: Vec<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualMachineStorageConfigurationTempDbSettings {
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
                    "dataFileCount",
                    &self.r#data_file_count,
                ),
                to_pulumi_object_field(
                    "dataFileGrowthInMb",
                    &self.r#data_file_growth_in_mb,
                ),
                to_pulumi_object_field(
                    "dataFileSizeMb",
                    &self.r#data_file_size_mb,
                ),
                to_pulumi_object_field(
                    "defaultFilePath",
                    &self.r#default_file_path,
                ),
                to_pulumi_object_field(
                    "logFileGrowthMb",
                    &self.r#log_file_growth_mb,
                ),
                to_pulumi_object_field(
                    "logFileSizeMb",
                    &self.r#log_file_size_mb,
                ),
                to_pulumi_object_field(
                    "luns",
                    &self.r#luns,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualMachineStorageConfigurationTempDbSettings {
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
                    r#data_file_count: {
                        let field_value = match fields_map.get("dataFileCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataFileCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_file_growth_in_mb: {
                        let field_value = match fields_map.get("dataFileGrowthInMb") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataFileGrowthInMb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_file_size_mb: {
                        let field_value = match fields_map.get("dataFileSizeMb") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataFileSizeMb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_file_path: {
                        let field_value = match fields_map.get("defaultFilePath") {
                            Some(value) => value,
                            None => bail!("Missing field 'defaultFilePath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_file_growth_mb: {
                        let field_value = match fields_map.get("logFileGrowthMb") {
                            Some(value) => value,
                            None => bail!("Missing field 'logFileGrowthMb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_file_size_mb: {
                        let field_value = match fields_map.get("logFileSizeMb") {
                            Some(value) => value,
                            None => bail!("Missing field 'logFileSizeMb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#luns: {
                        let field_value = match fields_map.get("luns") {
                            Some(value) => value,
                            None => bail!("Missing field 'luns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
