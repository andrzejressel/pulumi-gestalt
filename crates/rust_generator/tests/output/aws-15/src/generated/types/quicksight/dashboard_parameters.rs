#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DashboardParameters {
    /// A list of parameters that have a data type of date-time. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_DateTimeParameter.html).
    #[builder(into)]
    #[serde(rename = "dateTimeParameters")]
    pub r#date_time_parameters: Option<Vec<super::super::types::quicksight::DashboardParametersDateTimeParameter>>,
    /// A list of parameters that have a data type of decimal. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_DecimalParameter.html).
    #[builder(into)]
    #[serde(rename = "decimalParameters")]
    pub r#decimal_parameters: Option<Vec<super::super::types::quicksight::DashboardParametersDecimalParameter>>,
    /// A list of parameters that have a data type of integer. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_IntegerParameter.html).
    #[builder(into)]
    #[serde(rename = "integerParameters")]
    pub r#integer_parameters: Option<Vec<super::super::types::quicksight::DashboardParametersIntegerParameter>>,
    /// A list of parameters that have a data type of string. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_StringParameter.html).
    #[builder(into)]
    #[serde(rename = "stringParameters")]
    pub r#string_parameters: Option<Vec<super::super::types::quicksight::DashboardParametersStringParameter>>,
}
