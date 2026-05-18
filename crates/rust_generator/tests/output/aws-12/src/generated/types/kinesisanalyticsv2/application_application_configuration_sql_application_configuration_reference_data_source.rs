#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSource {
    #[builder(into)]
    #[serde(rename = "referenceId")]
    pub r#reference_id: Option<String>,
    /// Describes the format of the data in the streaming source, and how each data element maps to corresponding columns created in the in-application stream.
    #[builder(into)]
    #[serde(rename = "referenceSchema")]
    pub r#reference_schema: Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceReferenceSchema>,
    /// Identifies the S3 bucket and object that contains the reference data.
    #[builder(into)]
    #[serde(rename = "s3ReferenceDataSource")]
    pub r#s_3_reference_data_source: Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceS3ReferenceDataSource>,
    /// The name of the in-application table to create.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSource {
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
                    "reference_id",
                    &self.r#reference_id,
                ),
                to_pulumi_object_field(
                    "reference_schema",
                    &self.r#reference_schema,
                ),
                to_pulumi_object_field(
                    "s_3_reference_data_source",
                    &self.r#s_3_reference_data_source,
                ),
                to_pulumi_object_field(
                    "table_name",
                    &self.r#table_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSource {
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
                    r#reference_id: {
                        let field_value = match fields_map.get("reference_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'reference_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reference_schema: {
                        let field_value = match fields_map.get("reference_schema") {
                            Some(value) => value,
                            None => bail!("Missing field 'reference_schema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_reference_data_source: {
                        let field_value = match fields_map.get("s_3_reference_data_source") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_reference_data_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#table_name: {
                        let field_value = match fields_map.get("table_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'table_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
