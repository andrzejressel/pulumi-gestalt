#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClassifierCsvClassifier {
    /// Enables the processing of files that contain only one column.
    #[builder(into)]
    #[serde(rename = "allowSingleColumn")]
    pub r#allow_single_column: Option<bool>,
    /// Indicates whether the CSV file contains a header. This can be one of "ABSENT", "PRESENT", or "UNKNOWN".
    #[builder(into)]
    #[serde(rename = "containsHeader")]
    pub r#contains_header: Option<String>,
    /// Enables the custom datatype to be configured.
    #[builder(into)]
    #[serde(rename = "customDatatypeConfigured")]
    pub r#custom_datatype_configured: Option<bool>,
    /// A list of supported custom datatypes. Valid values are `BINARY`, `BOOLEAN`, `DATE`, `DECIMAL`, `DOUBLE`, `FLOAT`, `INT`, `LONG`, `SHORT`, `STRING`, `TIMESTAMP`.
    #[builder(into)]
    #[serde(rename = "customDatatypes")]
    pub r#custom_datatypes: Option<Vec<String>>,
    /// The delimiter used in the CSV to separate columns.
    #[builder(into)]
    #[serde(rename = "delimiter")]
    pub r#delimiter: Option<String>,
    /// Specifies whether to trim column values.
    #[builder(into)]
    #[serde(rename = "disableValueTrimming")]
    pub r#disable_value_trimming: Option<bool>,
    /// A list of strings representing column names.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<String>>,
    /// A custom symbol to denote what combines content into a single column value. It must be different from the column delimiter.
    #[builder(into)]
    #[serde(rename = "quoteSymbol")]
    pub r#quote_symbol: Option<String>,
    /// The SerDe for processing CSV. Valid values are `OpenCSVSerDe`, `LazySimpleSerDe`, `None`.
    #[builder(into)]
    #[serde(rename = "serde")]
    pub r#serde: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClassifierCsvClassifier {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("allow_single_column".to_string(), self.r#allow_single_column.to_pulumi_value().await);
            map.insert("contains_header".to_string(), self.r#contains_header.to_pulumi_value().await);
            map.insert("custom_datatype_configured".to_string(), self.r#custom_datatype_configured.to_pulumi_value().await);
            map.insert("custom_datatypes".to_string(), self.r#custom_datatypes.to_pulumi_value().await);
            map.insert("delimiter".to_string(), self.r#delimiter.to_pulumi_value().await);
            map.insert("disable_value_trimming".to_string(), self.r#disable_value_trimming.to_pulumi_value().await);
            map.insert("headers".to_string(), self.r#headers.to_pulumi_value().await);
            map.insert("quote_symbol".to_string(), self.r#quote_symbol.to_pulumi_value().await);
            map.insert("serde".to_string(), self.r#serde.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClassifierCsvClassifier {
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
                    r#allow_single_column: {
                        let field_value = match fields_map.get("allow_single_column") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_single_column' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#contains_header: {
                        let field_value = match fields_map.get("contains_header") {
                            Some(value) => value,
                            None => bail!("Missing field 'contains_header' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#custom_datatype_configured: {
                        let field_value = match fields_map.get("custom_datatype_configured") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_datatype_configured' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#custom_datatypes: {
                        let field_value = match fields_map.get("custom_datatypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_datatypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#delimiter: {
                        let field_value = match fields_map.get("delimiter") {
                            Some(value) => value,
                            None => bail!("Missing field 'delimiter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#disable_value_trimming: {
                        let field_value = match fields_map.get("disable_value_trimming") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_value_trimming' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#headers: {
                        let field_value = match fields_map.get("headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#quote_symbol: {
                        let field_value = match fields_map.get("quote_symbol") {
                            Some(value) => value,
                            None => bail!("Missing field 'quote_symbol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#serde: {
                        let field_value = match fields_map.get("serde") {
                            Some(value) => value,
                            None => bail!("Missing field 'serde' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
