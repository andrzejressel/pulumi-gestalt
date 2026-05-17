#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionJobTriggerInspectJobActionSaveFindingsOutputConfig {
    /// Schema used for writing the findings for Inspect jobs. This field is only used for
    /// Inspect and must be unspecified for Risk jobs. Columns are derived from the Finding
    /// object. If appending to an existing table, any columns from the predefined schema
    /// that are missing will be added. No columns in the existing table will be deleted.
    /// If unspecified, then all available columns will be used for a new table or an (existing)
    /// table with no schema, and no changes will be made to an existing table that has a schema.
    /// Only for use with external storage.
    /// Possible values are: `BASIC_COLUMNS`, `GCS_COLUMNS`, `DATASTORE_COLUMNS`, `BIG_QUERY_COLUMNS`, `ALL_COLUMNS`.
    #[builder(into)]
    #[serde(rename = "outputSchema")]
    pub r#output_schema: Option<String>,
    /// Information on the location of the target BigQuery Table.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "table")]
    pub r#table: Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionSaveFindingsOutputConfigTable>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionJobTriggerInspectJobActionSaveFindingsOutputConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "output_schema".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#output_schema,
                )
                .await,
            );
            map.insert(
                "table".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#table,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionJobTriggerInspectJobActionSaveFindingsOutputConfig {
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
                    r#output_schema: {
                        let field_value = match fields_map.get("output_schema") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_schema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#table: {
                        let field_value = match fields_map.get("table") {
                            Some(value) => value,
                            None => bail!("Missing field 'table' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
