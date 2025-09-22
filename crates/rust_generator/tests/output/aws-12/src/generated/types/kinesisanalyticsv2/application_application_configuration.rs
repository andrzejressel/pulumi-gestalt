#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
