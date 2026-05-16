#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataCellsFilterTableDataRowFilter {
    /// (Optional) A wildcard that matches all rows.
    #[builder(into)]
    #[serde(rename = "allRowsWildcard")]
    pub r#all_rows_wildcard: Option<Box<super::super::types::lakeformation::DataCellsFilterTableDataRowFilterAllRowsWildcard>>,
    /// (Optional) A filter expression.
    #[builder(into)]
    #[serde(rename = "filterExpression")]
    pub r#filter_expression: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataCellsFilterTableDataRowFilter {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("all_rows_wildcard".to_string(), self.r#all_rows_wildcard.to_pulumi_value().await);
            map.insert("filter_expression".to_string(), self.r#filter_expression.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataCellsFilterTableDataRowFilter {
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
                    r#all_rows_wildcard: {
                        let field_value = match fields_map.get("all_rows_wildcard") {
                            Some(value) => value,
                            None => bail!("Missing field 'all_rows_wildcard' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lakeformation::DataCellsFilterTableDataRowFilterAllRowsWildcard>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#filter_expression: {
                        let field_value = match fields_map.get("filter_expression") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter_expression' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
