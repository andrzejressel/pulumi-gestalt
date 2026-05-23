#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineStorageConfiguration {
    /// A `storage_settings` block as defined below.
    #[builder(into)]
    pub r#data_settings: Option<Box<super::super::types::mssql::VirtualMachineStorageConfigurationDataSettings>>,
    /// The type of disk configuration to apply to the SQL Server. Valid values include `NEW`, `EXTEND`, or `ADD`.
    #[builder(into)]
    pub r#disk_type: String,
    /// A `storage_settings` block as defined below.
    #[builder(into)]
    pub r#log_settings: Option<Box<super::super::types::mssql::VirtualMachineStorageConfigurationLogSettings>>,
    /// The type of storage workload. Valid values include `GENERAL`, `OLTP`, or `DW`.
    #[builder(into)]
    pub r#storage_workload_type: String,
    /// Specifies whether to set system databases (except tempDb) location to newly created data storage. Possible values are `true` and `false`. Defaults to `false`.
    #[builder(into)]
    pub r#system_db_on_data_disk_enabled: Option<bool>,
    /// An `temp_db_settings` block as defined below.
    #[builder(into)]
    pub r#temp_db_settings: Option<Box<super::super::types::mssql::VirtualMachineStorageConfigurationTempDbSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualMachineStorageConfiguration {
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
                    "dataSettings",
                    &self.r#data_settings,
                ),
                to_pulumi_object_field(
                    "diskType",
                    &self.r#disk_type,
                ),
                to_pulumi_object_field(
                    "logSettings",
                    &self.r#log_settings,
                ),
                to_pulumi_object_field(
                    "storageWorkloadType",
                    &self.r#storage_workload_type,
                ),
                to_pulumi_object_field(
                    "systemDbOnDataDiskEnabled",
                    &self.r#system_db_on_data_disk_enabled,
                ),
                to_pulumi_object_field(
                    "tempDbSettings",
                    &self.r#temp_db_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualMachineStorageConfiguration {
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
                    r#data_settings: {
                        let field_value = match fields_map.get("dataSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_type: {
                        let field_value = match fields_map.get("diskType") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_settings: {
                        let field_value = match fields_map.get("logSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'logSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_workload_type: {
                        let field_value = match fields_map.get("storageWorkloadType") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageWorkloadType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#system_db_on_data_disk_enabled: {
                        let field_value = match fields_map.get("systemDbOnDataDiskEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'systemDbOnDataDiskEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#temp_db_settings: {
                        let field_value = match fields_map.get("tempDbSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'tempDbSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
