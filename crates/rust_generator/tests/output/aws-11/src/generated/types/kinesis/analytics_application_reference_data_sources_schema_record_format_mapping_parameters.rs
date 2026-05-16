#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AnalyticsApplicationReferenceDataSourcesSchemaRecordFormatMappingParameters {
    /// Mapping information when the record format uses delimiters.
    /// See CSV Mapping Parameters below for more details.
    #[builder(into)]
    #[serde(rename = "csv")]
    pub r#csv: Option<Box<super::super::types::kinesis::AnalyticsApplicationReferenceDataSourcesSchemaRecordFormatMappingParametersCsv>>,
    /// Mapping information when JSON is the record format on the streaming source.
    /// See JSON Mapping Parameters below for more details.
    #[builder(into)]
    #[serde(rename = "json")]
    pub r#json: Option<Box<super::super::types::kinesis::AnalyticsApplicationReferenceDataSourcesSchemaRecordFormatMappingParametersJson>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AnalyticsApplicationReferenceDataSourcesSchemaRecordFormatMappingParameters {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("csv".to_string(), self.r#csv.to_pulumi_value().await);
            map.insert("json".to_string(), self.r#json.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AnalyticsApplicationReferenceDataSourcesSchemaRecordFormatMappingParameters {
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
                    r#csv: {
                        let field_value = match fields_map.get("csv") {
                            Some(value) => value,
                            None => bail!("Missing field 'csv' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kinesis::AnalyticsApplicationReferenceDataSourcesSchemaRecordFormatMappingParametersCsv>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#json: {
                        let field_value = match fields_map.get("json") {
                            Some(value) => value,
                            None => bail!("Missing field 'json' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kinesis::AnalyticsApplicationReferenceDataSourcesSchemaRecordFormatMappingParametersJson>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
