#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZoneDiscoverySpecCsvOptions {
    /// Optional. The delimiter being used to separate values. This defaults to ','.
    #[builder(into)]
    #[serde(rename = "delimiter")]
    pub r#delimiter: Option<String>,
    /// Optional. Whether to disable the inference of data type for CSV data. If true, all columns will be registered as strings.
    #[builder(into)]
    #[serde(rename = "disableTypeInference")]
    pub r#disable_type_inference: Option<bool>,
    /// Optional. The character encoding of the data. The default is UTF-8.
    #[builder(into)]
    #[serde(rename = "encoding")]
    pub r#encoding: Option<String>,
    /// Optional. The number of rows to interpret as header rows that should be skipped when reading data rows.
    #[builder(into)]
    #[serde(rename = "headerRows")]
    pub r#header_rows: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ZoneDiscoverySpecCsvOptions {
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
                    "delimiter",
                    &self.r#delimiter,
                ),
                to_pulumi_object_field(
                    "disable_type_inference",
                    &self.r#disable_type_inference,
                ),
                to_pulumi_object_field(
                    "encoding",
                    &self.r#encoding,
                ),
                to_pulumi_object_field(
                    "header_rows",
                    &self.r#header_rows,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ZoneDiscoverySpecCsvOptions {
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
                    r#delimiter: {
                        let field_value = match fields_map.get("delimiter") {
                            Some(value) => value,
                            None => bail!("Missing field 'delimiter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_type_inference: {
                        let field_value = match fields_map.get("disable_type_inference") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_type_inference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encoding: {
                        let field_value = match fields_map.get("encoding") {
                            Some(value) => value,
                            None => bail!("Missing field 'encoding' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#header_rows: {
                        let field_value = match fields_map.get("header_rows") {
                            Some(value) => value,
                            None => bail!("Missing field 'header_rows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
