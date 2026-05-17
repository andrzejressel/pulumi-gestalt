#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableExternalDataConfigurationCsvOptions {
    /// Indicates if BigQuery should accept rows
    /// that are missing trailing optional columns.
    #[builder(into)]
    #[serde(rename = "allowJaggedRows")]
    pub r#allow_jagged_rows: Option<bool>,
    /// Indicates if BigQuery should allow
    /// quoted data sections that contain newline characters in a CSV file.
    /// The default value is false.
    #[builder(into)]
    #[serde(rename = "allowQuotedNewlines")]
    pub r#allow_quoted_newlines: Option<bool>,
    /// The character encoding of the data. The supported
    /// values are UTF-8 or ISO-8859-1.
    #[builder(into)]
    #[serde(rename = "encoding")]
    pub r#encoding: Option<String>,
    /// The separator for fields in a CSV file.
    #[builder(into)]
    #[serde(rename = "fieldDelimiter")]
    pub r#field_delimiter: Option<String>,
    /// The value that is used to quote data sections in a
    /// CSV file. If your data does not contain quoted sections, set the
    /// property value to an empty string. If your data contains quoted newline
    /// characters, you must also set the `allow_quoted_newlines` property to true.
    /// The API-side default is `"`, specified in the provider escaped as `\"`. Due to
    /// limitations with default values, this value is required to be
    /// explicitly set.
    #[builder(into)]
    #[serde(rename = "quote")]
    pub r#quote: String,
    /// The number of rows at the top of a CSV
    /// file that BigQuery will skip when reading the data.
    #[builder(into)]
    #[serde(rename = "skipLeadingRows")]
    pub r#skip_leading_rows: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TableExternalDataConfigurationCsvOptions {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "allow_jagged_rows",
                    &self.r#allow_jagged_rows,
                ),
                to_pulumi_object_field(
                    "allow_quoted_newlines",
                    &self.r#allow_quoted_newlines,
                ),
                to_pulumi_object_field(
                    "encoding",
                    &self.r#encoding,
                ),
                to_pulumi_object_field(
                    "field_delimiter",
                    &self.r#field_delimiter,
                ),
                to_pulumi_object_field(
                    "quote",
                    &self.r#quote,
                ),
                to_pulumi_object_field(
                    "skip_leading_rows",
                    &self.r#skip_leading_rows,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableExternalDataConfigurationCsvOptions {
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
                    r#allow_jagged_rows: {
                        let field_value = match fields_map.get("allow_jagged_rows") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_jagged_rows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_quoted_newlines: {
                        let field_value = match fields_map.get("allow_quoted_newlines") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_quoted_newlines' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#field_delimiter: {
                        let field_value = match fields_map.get("field_delimiter") {
                            Some(value) => value,
                            None => bail!("Missing field 'field_delimiter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#quote: {
                        let field_value = match fields_map.get("quote") {
                            Some(value) => value,
                            None => bail!("Missing field 'quote' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#skip_leading_rows: {
                        let field_value = match fields_map.get("skip_leading_rows") {
                            Some(value) => value,
                            None => bail!("Missing field 'skip_leading_rows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
