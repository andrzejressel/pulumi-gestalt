#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineStorageConfigurationTempDbSettings {
    /// The SQL Server default file count. This value defaults to `8`
    #[builder(into)]
    #[serde(rename = "dataFileCount")]
    pub r#data_file_count: Option<i32>,
    /// The SQL Server default file size - This value defaults to `512`
    #[builder(into)]
    #[serde(rename = "dataFileGrowthInMb")]
    pub r#data_file_growth_in_mb: Option<i32>,
    /// The SQL Server default file size - This value defaults to `256`
    #[builder(into)]
    #[serde(rename = "dataFileSizeMb")]
    pub r#data_file_size_mb: Option<i32>,
    /// The SQL Server default path
    #[builder(into)]
    #[serde(rename = "defaultFilePath")]
    pub r#default_file_path: String,
    /// The SQL Server default file size - This value defaults to `512`
    #[builder(into)]
    #[serde(rename = "logFileGrowthMb")]
    pub r#log_file_growth_mb: Option<i32>,
    /// The SQL Server default file size - This value defaults to `256`
    #[builder(into)]
    #[serde(rename = "logFileSizeMb")]
    pub r#log_file_size_mb: Option<i32>,
    /// A list of Logical Unit Numbers for the disks.
    #[builder(into)]
    #[serde(rename = "luns")]
    pub r#luns: Vec<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualMachineStorageConfigurationTempDbSettings {
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
                "data_file_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_file_count,
                )
                .await,
            );
            map.insert(
                "data_file_growth_in_mb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_file_growth_in_mb,
                )
                .await,
            );
            map.insert(
                "data_file_size_mb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_file_size_mb,
                )
                .await,
            );
            map.insert(
                "default_file_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_file_path,
                )
                .await,
            );
            map.insert(
                "log_file_growth_mb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#log_file_growth_mb,
                )
                .await,
            );
            map.insert(
                "log_file_size_mb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#log_file_size_mb,
                )
                .await,
            );
            map.insert(
                "luns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#luns,
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
                        let field_value = match fields_map.get("data_file_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_file_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_file_growth_in_mb: {
                        let field_value = match fields_map.get("data_file_growth_in_mb") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_file_growth_in_mb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_file_size_mb: {
                        let field_value = match fields_map.get("data_file_size_mb") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_file_size_mb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_file_path: {
                        let field_value = match fields_map.get("default_file_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_file_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_file_growth_mb: {
                        let field_value = match fields_map.get("log_file_growth_mb") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_file_growth_mb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_file_size_mb: {
                        let field_value = match fields_map.get("log_file_size_mb") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_file_size_mb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
