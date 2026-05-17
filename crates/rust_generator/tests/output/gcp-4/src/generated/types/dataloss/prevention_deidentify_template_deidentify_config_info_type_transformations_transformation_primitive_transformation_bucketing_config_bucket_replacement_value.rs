#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationBucketingConfigBucketReplacementValue {
    /// Represents a whole or partial calendar date.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dateValue")]
    pub r#date_value: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationBucketingConfigBucketReplacementValueDateValue>>,
    /// Represents a day of the week.
    /// Possible values are: `MONDAY`, `TUESDAY`, `WEDNESDAY`, `THURSDAY`, `FRIDAY`, `SATURDAY`, `SUNDAY`.
    #[builder(into)]
    #[serde(rename = "dayOfWeekValue")]
    pub r#day_of_week_value: Option<String>,
    /// A float value.
    #[builder(into)]
    #[serde(rename = "floatValue")]
    pub r#float_value: Option<f64>,
    /// An integer value (int64 format)
    #[builder(into)]
    #[serde(rename = "integerValue")]
    pub r#integer_value: Option<String>,
    /// A string value.
    #[builder(into)]
    #[serde(rename = "stringValue")]
    pub r#string_value: Option<String>,
    /// Represents a time of day.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "timeValue")]
    pub r#time_value: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationBucketingConfigBucketReplacementValueTimeValue>>,
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "timestampValue")]
    pub r#timestamp_value: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationBucketingConfigBucketReplacementValue {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "date_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#date_value,
                )
                .await,
            );
            map.insert(
                "day_of_week_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#day_of_week_value,
                )
                .await,
            );
            map.insert(
                "float_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#float_value,
                )
                .await,
            );
            map.insert(
                "integer_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#integer_value,
                )
                .await,
            );
            map.insert(
                "string_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#string_value,
                )
                .await,
            );
            map.insert(
                "time_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#time_value,
                )
                .await,
            );
            map.insert(
                "timestamp_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timestamp_value,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationBucketingConfigBucketReplacementValue {
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
                    r#date_value: {
                        let field_value = match fields_map.get("date_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'date_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#day_of_week_value: {
                        let field_value = match fields_map.get("day_of_week_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'day_of_week_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#float_value: {
                        let field_value = match fields_map.get("float_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'float_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#integer_value: {
                        let field_value = match fields_map.get("integer_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'integer_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_value: {
                        let field_value = match fields_map.get("string_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_value: {
                        let field_value = match fields_map.get("time_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timestamp_value: {
                        let field_value = match fields_map.get("timestamp_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestamp_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
