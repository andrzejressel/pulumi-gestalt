#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDataSourceDataSourceBackupApplianceApplication {
    /// Appliance Id of the Backup Appliance.
    #[builder(into)]
    #[serde(rename = "applianceId")]
    pub r#appliance_id: String,
    /// The appid field of the application within the Backup Appliance.
    #[builder(into)]
    #[serde(rename = "applicationId")]
    pub r#application_id: String,
    /// The name of the Application as known to the Backup Appliance.
    #[builder(into)]
    #[serde(rename = "applicationName")]
    pub r#application_name: String,
    /// Appliance name.
    #[builder(into)]
    #[serde(rename = "backupAppliance")]
    pub r#backup_appliance: String,
    /// Hostid of the application host.
    #[builder(into)]
    #[serde(rename = "hostId")]
    pub r#host_id: String,
    /// Hostname of the host where the application is running.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: String,
    /// The type of the application. e.g. VMBackup
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDataSourceDataSourceBackupApplianceApplication {
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
                    "applianceId",
                    &self.r#appliance_id,
                ),
                to_pulumi_object_field(
                    "applicationId",
                    &self.r#application_id,
                ),
                to_pulumi_object_field(
                    "applicationName",
                    &self.r#application_name,
                ),
                to_pulumi_object_field(
                    "backupAppliance",
                    &self.r#backup_appliance,
                ),
                to_pulumi_object_field(
                    "hostId",
                    &self.r#host_id,
                ),
                to_pulumi_object_field(
                    "hostname",
                    &self.r#hostname,
                ),
                to_pulumi_object_field(
                    "type",
                    &self.r#type_,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDataSourceDataSourceBackupApplianceApplication {
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
                    r#appliance_id: {
                        let field_value = match fields_map.get("applianceId") {
                            Some(value) => value,
                            None => bail!("Missing field 'applianceId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#application_id: {
                        let field_value = match fields_map.get("applicationId") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicationId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#application_name: {
                        let field_value = match fields_map.get("applicationName") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicationName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup_appliance: {
                        let field_value = match fields_map.get("backupAppliance") {
                            Some(value) => value,
                            None => bail!("Missing field 'backupAppliance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_id: {
                        let field_value = match fields_map.get("hostId") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hostname: {
                        let field_value = match fields_map.get("hostname") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostname' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type") {
                            Some(value) => value,
                            None => bail!("Missing field 'type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
