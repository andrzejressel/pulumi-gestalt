#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSetPhysicalTableMapS3Source {
    /// ARN of the data source.
    #[builder(into)]
    #[serde(rename = "dataSourceArn")]
    pub r#data_source_arn: String,
    /// Column schema of the table. See input_columns.
    #[builder(into)]
    #[serde(rename = "inputColumns")]
    pub r#input_columns: Vec<super::super::types::quicksight::DataSetPhysicalTableMapS3SourceInputColumn>,
    /// Information about the format for the S3 source file or files. See upload_settings.
    #[builder(into)]
    #[serde(rename = "uploadSettings")]
    pub r#upload_settings: Box<super::super::types::quicksight::DataSetPhysicalTableMapS3SourceUploadSettings>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSetPhysicalTableMapS3Source {
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
                    "data_source_arn",
                    &self.r#data_source_arn,
                ),
                to_pulumi_object_field(
                    "input_columns",
                    &self.r#input_columns,
                ),
                to_pulumi_object_field(
                    "upload_settings",
                    &self.r#upload_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSetPhysicalTableMapS3Source {
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
                    r#data_source_arn: {
                        let field_value = match fields_map.get("data_source_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_source_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_columns: {
                        let field_value = match fields_map.get("input_columns") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_columns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upload_settings: {
                        let field_value = match fields_map.get("upload_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'upload_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
