#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDataSourceBackupConfigInfoBackupApplianceBackupConfig {
    /// The name of the application.
    #[builder(into)]
    #[serde(rename = "applicationName")]
    pub r#application_name: String,
    /// The ID of the backup appliance.
    #[builder(into)]
    #[serde(rename = "backupApplianceId")]
    pub r#backup_appliance_id: String,
    /// The name of the backup appliance.
    #[builder(into)]
    #[serde(rename = "backupApplianceName")]
    pub r#backup_appliance_name: String,
    /// The name of the host where the application is running.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: String,
    /// The ID of the SLA of this application.
    #[builder(into)]
    #[serde(rename = "slaId")]
    pub r#sla_id: String,
    /// The name of the SLP associated with the application.
    #[builder(into)]
    #[serde(rename = "slpName")]
    pub r#slp_name: String,
    /// The name of the SLT associated with the application.
    #[builder(into)]
    #[serde(rename = "sltName")]
    pub r#slt_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDataSourceBackupConfigInfoBackupApplianceBackupConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "application_name",
                    &self.r#application_name,
                ),
                to_pulumi_object_field(
                    "backup_appliance_id",
                    &self.r#backup_appliance_id,
                ),
                to_pulumi_object_field(
                    "backup_appliance_name",
                    &self.r#backup_appliance_name,
                ),
                to_pulumi_object_field(
                    "host_name",
                    &self.r#host_name,
                ),
                to_pulumi_object_field(
                    "sla_id",
                    &self.r#sla_id,
                ),
                to_pulumi_object_field(
                    "slp_name",
                    &self.r#slp_name,
                ),
                to_pulumi_object_field(
                    "slt_name",
                    &self.r#slt_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDataSourceBackupConfigInfoBackupApplianceBackupConfig {
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
                    r#application_name: {
                        let field_value = match fields_map.get("application_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup_appliance_id: {
                        let field_value = match fields_map.get("backup_appliance_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_appliance_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup_appliance_name: {
                        let field_value = match fields_map.get("backup_appliance_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_appliance_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_name: {
                        let field_value = match fields_map.get("host_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sla_id: {
                        let field_value = match fields_map.get("sla_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'sla_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#slp_name: {
                        let field_value = match fields_map.get("slp_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'slp_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#slt_name: {
                        let field_value = match fields_map.get("slt_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'slt_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
