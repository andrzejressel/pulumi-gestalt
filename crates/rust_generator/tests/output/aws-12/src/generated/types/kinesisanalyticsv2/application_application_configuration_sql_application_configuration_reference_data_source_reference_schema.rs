#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceReferenceSchema {
    /// Describes the mapping of each data element in the streaming source to the corresponding column in the in-application stream.
    #[builder(into)]
    #[serde(rename = "recordColumns")]
    pub r#record_columns: Vec<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceReferenceSchemaRecordColumn>,
    /// Specifies the encoding of the records in the streaming source. For example, `UTF-8`.
    #[builder(into)]
    #[serde(rename = "recordEncoding")]
    pub r#record_encoding: Option<String>,
    /// Specifies the format of the records on the streaming source.
    #[builder(into)]
    #[serde(rename = "recordFormat")]
    pub r#record_format: Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceReferenceSchemaRecordFormat>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceReferenceSchema {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("record_columns".to_string(), self.r#record_columns.to_pulumi_value().await);
            map.insert("record_encoding".to_string(), self.r#record_encoding.to_pulumi_value().await);
            map.insert("record_format".to_string(), self.r#record_format.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceReferenceSchema {
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
                    r#record_columns: {
                        let field_value = match fields_map.get("record_columns") {
                            Some(value) => value,
                            None => bail!("Missing field 'record_columns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceReferenceSchemaRecordColumn> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#record_encoding: {
                        let field_value = match fields_map.get("record_encoding") {
                            Some(value) => value,
                            None => bail!("Missing field 'record_encoding' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#record_format: {
                        let field_value = match fields_map.get("record_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'record_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceReferenceSchemaRecordFormat> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
