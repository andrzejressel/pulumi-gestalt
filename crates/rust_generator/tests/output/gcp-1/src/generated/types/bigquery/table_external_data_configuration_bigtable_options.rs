#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableExternalDataConfigurationBigtableOptions {
    /// A list of column families to expose in the table schema along with their types. This list restricts the column families that can be referenced in queries and specifies their value types. You can use this list to do type conversions - see the 'type' field for more details. If you leave this list empty, all column families are present in the table schema and their values are read as BYTES. During a query only the column families referenced in that query are read from Bigtable.  Structure is documented below.
    #[builder(into)]
    #[serde(rename = "columnFamilies")]
    pub r#column_families: Option<Vec<super::super::types::bigquery::TableExternalDataConfigurationBigtableOptionsColumnFamily>>,
    /// If field is true, then the column families that are not specified in columnFamilies list are not exposed in the table schema. Otherwise, they are read with BYTES type values. The default value is false.
    #[builder(into)]
    #[serde(rename = "ignoreUnspecifiedColumnFamilies")]
    pub r#ignore_unspecified_column_families: Option<bool>,
    /// If field is true, then each column family will be read as a single JSON column. Otherwise they are read as a repeated cell structure containing timestamp/value tuples. The default value is false.
    #[builder(into)]
    #[serde(rename = "outputColumnFamiliesAsJson")]
    pub r#output_column_families_as_json: Option<bool>,
    /// If field is true, then the rowkey column families will be read and converted to string. Otherwise they are read with BYTES type values and users need to manually cast them with CAST if necessary. The default value is false.
    #[builder(into)]
    #[serde(rename = "readRowkeyAsString")]
    pub r#read_rowkey_as_string: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TableExternalDataConfigurationBigtableOptions {
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
                    "column_families",
                    &self.r#column_families,
                ),
                to_pulumi_object_field(
                    "ignore_unspecified_column_families",
                    &self.r#ignore_unspecified_column_families,
                ),
                to_pulumi_object_field(
                    "output_column_families_as_json",
                    &self.r#output_column_families_as_json,
                ),
                to_pulumi_object_field(
                    "read_rowkey_as_string",
                    &self.r#read_rowkey_as_string,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableExternalDataConfigurationBigtableOptions {
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
                    r#column_families: {
                        let field_value = match fields_map.get("column_families") {
                            Some(value) => value,
                            None => bail!("Missing field 'column_families' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ignore_unspecified_column_families: {
                        let field_value = match fields_map.get("ignore_unspecified_column_families") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignore_unspecified_column_families' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_column_families_as_json: {
                        let field_value = match fields_map.get("output_column_families_as_json") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_column_families_as_json' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#read_rowkey_as_string: {
                        let field_value = match fields_map.get("read_rowkey_as_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_rowkey_as_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
