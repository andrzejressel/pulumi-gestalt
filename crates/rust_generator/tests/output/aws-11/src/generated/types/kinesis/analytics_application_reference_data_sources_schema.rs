#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AnalyticsApplicationReferenceDataSourcesSchema {
    /// The Record Column mapping for the streaming source data element.
    /// See Record Columns below for more details.
    #[builder(into)]
    #[serde(rename = "recordColumns")]
    pub r#record_columns: Vec<super::super::types::kinesis::AnalyticsApplicationReferenceDataSourcesSchemaRecordColumn>,
    /// The Encoding of the record in the streaming source.
    #[builder(into)]
    #[serde(rename = "recordEncoding")]
    pub r#record_encoding: Option<String>,
    /// The Record Format and mapping information to schematize a record.
    /// See Record Format below for more details.
    #[builder(into)]
    #[serde(rename = "recordFormat")]
    pub r#record_format: Box<super::super::types::kinesis::AnalyticsApplicationReferenceDataSourcesSchemaRecordFormat>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AnalyticsApplicationReferenceDataSourcesSchema {
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
                    "recordColumns",
                    &self.r#record_columns,
                ),
                to_pulumi_object_field(
                    "recordEncoding",
                    &self.r#record_encoding,
                ),
                to_pulumi_object_field(
                    "recordFormat",
                    &self.r#record_format,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AnalyticsApplicationReferenceDataSourcesSchema {
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
                    r#record_columns: {
                        let field_value = match fields_map.get("recordColumns") {
                            Some(value) => value,
                            None => bail!("Missing field 'recordColumns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_encoding: {
                        let field_value = match fields_map.get("recordEncoding") {
                            Some(value) => value,
                            None => bail!("Missing field 'recordEncoding' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_format: {
                        let field_value = match fields_map.get("recordFormat") {
                            Some(value) => value,
                            None => bail!("Missing field 'recordFormat' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
