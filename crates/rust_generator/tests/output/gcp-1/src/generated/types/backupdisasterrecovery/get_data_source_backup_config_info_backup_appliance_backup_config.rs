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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "applicationName",
                    &self.r#application_name,
                ),
                to_pulumi_object_field(
                    "backupApplianceId",
                    &self.r#backup_appliance_id,
                ),
                to_pulumi_object_field(
                    "backupApplianceName",
                    &self.r#backup_appliance_name,
                ),
                to_pulumi_object_field(
                    "hostName",
                    &self.r#host_name,
                ),
                to_pulumi_object_field(
                    "slaId",
                    &self.r#sla_id,
                ),
                to_pulumi_object_field(
                    "slpName",
                    &self.r#slp_name,
                ),
                to_pulumi_object_field(
                    "sltName",
                    &self.r#slt_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("applicationName") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicationName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup_appliance_id: {
                        let field_value = match fields_map.get("backupApplianceId") {
                            Some(value) => value,
                            None => bail!("Missing field 'backupApplianceId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup_appliance_name: {
                        let field_value = match fields_map.get("backupApplianceName") {
                            Some(value) => value,
                            None => bail!("Missing field 'backupApplianceName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_name: {
                        let field_value = match fields_map.get("hostName") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sla_id: {
                        let field_value = match fields_map.get("slaId") {
                            Some(value) => value,
                            None => bail!("Missing field 'slaId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#slp_name: {
                        let field_value = match fields_map.get("slpName") {
                            Some(value) => value,
                            None => bail!("Missing field 'slpName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#slt_name: {
                        let field_value = match fields_map.get("sltName") {
                            Some(value) => value,
                            None => bail!("Missing field 'sltName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
