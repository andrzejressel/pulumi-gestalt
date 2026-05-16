#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertPolicyConditionConditionThresholdForecastOptions {
    /// The length of time into the future to forecast
    /// whether a timeseries will violate the threshold.
    /// If the predicted value is found to violate the
    /// threshold, and the violation is observed in all
    /// forecasts made for the Configured `duration`,
    /// then the timeseries is considered to be failing.
    #[builder(into)]
    #[serde(rename = "forecastHorizon")]
    pub r#forecast_horizon: String,
}
