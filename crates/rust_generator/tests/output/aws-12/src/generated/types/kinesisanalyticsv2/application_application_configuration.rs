#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationApplicationConfiguration {
    /// The code location and type parameters for the application.
    #[builder(into)]
    #[serde(rename = "applicationCodeConfiguration")]
    pub r#application_code_configuration: Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationApplicationCodeConfiguration>,
    /// Describes whether snapshots are enabled for a Flink-based application.
    #[builder(into)]
    #[serde(rename = "applicationSnapshotConfiguration")]
    pub r#application_snapshot_configuration: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationApplicationSnapshotConfiguration>>,
    /// Describes execution properties for a Flink-based application.
    #[builder(into)]
    #[serde(rename = "environmentProperties")]
    pub r#environment_properties: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationEnvironmentProperties>>,
    /// The configuration of a Flink-based application.
    #[builder(into)]
    #[serde(rename = "flinkApplicationConfiguration")]
    pub r#flink_application_configuration: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationFlinkApplicationConfiguration>>,
    /// Describes the starting properties for a Flink-based application.
    #[builder(into)]
    #[serde(rename = "runConfiguration")]
    pub r#run_configuration: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationRunConfiguration>>,
    /// The configuration of a SQL-based application.
    #[builder(into)]
    #[serde(rename = "sqlApplicationConfiguration")]
    pub r#sql_application_configuration: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfiguration>>,
    /// The VPC configuration of a Flink-based application.
    #[builder(into)]
    #[serde(rename = "vpcConfiguration")]
    pub r#vpc_configuration: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationVpcConfiguration>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationApplicationConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("application_code_configuration".to_string(), self.r#application_code_configuration.to_pulumi_value().await);
            map.insert("application_snapshot_configuration".to_string(), self.r#application_snapshot_configuration.to_pulumi_value().await);
            map.insert("environment_properties".to_string(), self.r#environment_properties.to_pulumi_value().await);
            map.insert("flink_application_configuration".to_string(), self.r#flink_application_configuration.to_pulumi_value().await);
            map.insert("run_configuration".to_string(), self.r#run_configuration.to_pulumi_value().await);
            map.insert("sql_application_configuration".to_string(), self.r#sql_application_configuration.to_pulumi_value().await);
            map.insert("vpc_configuration".to_string(), self.r#vpc_configuration.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApplicationApplicationConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#application_code_configuration: {
                        let field_value = match fields_map.get("application_code_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_code_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationApplicationCodeConfiguration> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#application_snapshot_configuration: {
                        let field_value = match fields_map.get("application_snapshot_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_snapshot_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationApplicationSnapshotConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#environment_properties: {
                        let field_value = match fields_map.get("environment_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'environment_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationEnvironmentProperties>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#flink_application_configuration: {
                        let field_value = match fields_map.get("flink_application_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'flink_application_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationFlinkApplicationConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#run_configuration: {
                        let field_value = match fields_map.get("run_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'run_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationRunConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sql_application_configuration: {
                        let field_value = match fields_map.get("sql_application_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'sql_application_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#vpc_configuration: {
                        let field_value = match fields_map.get("vpc_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationVpcConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
