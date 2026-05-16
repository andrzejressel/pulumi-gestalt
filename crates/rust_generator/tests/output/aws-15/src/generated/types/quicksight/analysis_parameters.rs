#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AnalysisParameters {
    /// A list of parameters that have a data type of date-time. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_DateTimeParameter.html).
    #[builder(into)]
    #[serde(rename = "dateTimeParameters")]
    pub r#date_time_parameters: Option<Vec<super::super::types::quicksight::AnalysisParametersDateTimeParameter>>,
    /// A list of parameters that have a data type of decimal. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_DecimalParameter.html).
    #[builder(into)]
    #[serde(rename = "decimalParameters")]
    pub r#decimal_parameters: Option<Vec<super::super::types::quicksight::AnalysisParametersDecimalParameter>>,
    /// A list of parameters that have a data type of integer. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_IntegerParameter.html).
    #[builder(into)]
    #[serde(rename = "integerParameters")]
    pub r#integer_parameters: Option<Vec<super::super::types::quicksight::AnalysisParametersIntegerParameter>>,
    /// A list of parameters that have a data type of string. See [AWS API Documentation for complete description](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_StringParameter.html).
    #[builder(into)]
    #[serde(rename = "stringParameters")]
    pub r#string_parameters: Option<Vec<super::super::types::quicksight::AnalysisParametersStringParameter>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AnalysisParameters {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("date_time_parameters".to_string(), self.r#date_time_parameters.to_pulumi_value().await);
            map.insert("decimal_parameters".to_string(), self.r#decimal_parameters.to_pulumi_value().await);
            map.insert("integer_parameters".to_string(), self.r#integer_parameters.to_pulumi_value().await);
            map.insert("string_parameters".to_string(), self.r#string_parameters.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AnalysisParameters {
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
                    r#date_time_parameters: {
                        let field_value = match fields_map.get("date_time_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'date_time_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::quicksight::AnalysisParametersDateTimeParameter>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#decimal_parameters: {
                        let field_value = match fields_map.get("decimal_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'decimal_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::quicksight::AnalysisParametersDecimalParameter>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#integer_parameters: {
                        let field_value = match fields_map.get("integer_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'integer_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::quicksight::AnalysisParametersIntegerParameter>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#string_parameters: {
                        let field_value = match fields_map.get("string_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::quicksight::AnalysisParametersStringParameter>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
