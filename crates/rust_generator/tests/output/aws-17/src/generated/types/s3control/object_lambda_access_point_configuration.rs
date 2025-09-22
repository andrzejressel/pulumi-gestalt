#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ObjectLambdaAccessPointConfiguration {
    /// Allowed features. Valid values: `GetObject-Range`, `GetObject-PartNumber`.
    #[builder(into)]
    #[serde(rename = "allowedFeatures")]
    pub r#allowed_features: Option<Vec<String>>,
    /// Whether or not the CloudWatch metrics configuration is enabled.
    #[builder(into)]
    #[serde(rename = "cloudWatchMetricsEnabled")]
    pub r#cloud_watch_metrics_enabled: Option<bool>,
    /// Standard access point associated with the Object Lambda Access Point.
    #[builder(into)]
    #[serde(rename = "supportingAccessPoint")]
    pub r#supporting_access_point: String,
    /// List of transformation configurations for the Object Lambda Access Point. See Transformation Configuration below for more details.
    #[builder(into)]
    #[serde(rename = "transformationConfigurations")]
    pub r#transformation_configurations: Vec<super::super::types::s3control::ObjectLambdaAccessPointConfigurationTransformationConfiguration>,
}
