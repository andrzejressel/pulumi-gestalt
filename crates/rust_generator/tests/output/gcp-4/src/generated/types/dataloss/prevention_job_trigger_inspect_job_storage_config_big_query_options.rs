#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionJobTriggerInspectJobStorageConfigBigQueryOptions {
    /// References to fields excluded from scanning.
    /// This allows you to skip inspection of entire columns which you know have no findings.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "excludedFields")]
    pub r#excluded_fields: Option<Vec<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigBigQueryOptionsExcludedField>>,
    /// Specifies the BigQuery fields that will be returned with findings.
    /// If not specified, no identifying fields will be returned for findings.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "identifyingFields")]
    pub r#identifying_fields: Option<Vec<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigBigQueryOptionsIdentifyingField>>,
    /// Limit scanning only to these fields.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "includedFields")]
    pub r#included_fields: Option<Vec<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigBigQueryOptionsIncludedField>>,
    /// Max number of rows to scan. If the table has more rows than this value, the rest of the rows are omitted.
    /// If not set, or if set to 0, all rows will be scanned. Only one of rowsLimit and rowsLimitPercent can be
    /// specified. Cannot be used in conjunction with TimespanConfig.
    #[builder(into)]
    #[serde(rename = "rowsLimit")]
    pub r#rows_limit: Option<i32>,
    /// Max percentage of rows to scan. The rest are omitted. The number of rows scanned is rounded down.
    /// Must be between 0 and 100, inclusively. Both 0 and 100 means no limit. Defaults to 0. Only one of
    /// rowsLimit and rowsLimitPercent can be specified. Cannot be used in conjunction with TimespanConfig.
    #[builder(into)]
    #[serde(rename = "rowsLimitPercent")]
    pub r#rows_limit_percent: Option<i32>,
    /// How to sample rows if not all rows are scanned. Meaningful only when used in conjunction with either
    /// rowsLimit or rowsLimitPercent. If not specified, rows are scanned in the order BigQuery reads them.
    /// If TimespanConfig is set, set this to an empty string to avoid using the default value.
    /// Default value is `TOP`.
    /// Possible values are: `TOP`, `RANDOM_START`.
    #[builder(into)]
    #[serde(rename = "sampleMethod")]
    pub r#sample_method: Option<String>,
    /// Set of files to scan.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "tableReference")]
    pub r#table_reference: Box<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigBigQueryOptionsTableReference>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionJobTriggerInspectJobStorageConfigBigQueryOptions {
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
                "excluded_fields".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#excluded_fields,
                )
                .await,
            );
            map.insert(
                "identifying_fields".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#identifying_fields,
                )
                .await,
            );
            map.insert(
                "included_fields".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#included_fields,
                )
                .await,
            );
            map.insert(
                "rows_limit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rows_limit,
                )
                .await,
            );
            map.insert(
                "rows_limit_percent".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rows_limit_percent,
                )
                .await,
            );
            map.insert(
                "sample_method".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sample_method,
                )
                .await,
            );
            map.insert(
                "table_reference".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#table_reference,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionJobTriggerInspectJobStorageConfigBigQueryOptions {
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
                    r#excluded_fields: {
                        let field_value = match fields_map.get("excluded_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'excluded_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#identifying_fields: {
                        let field_value = match fields_map.get("identifying_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'identifying_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#included_fields: {
                        let field_value = match fields_map.get("included_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'included_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rows_limit: {
                        let field_value = match fields_map.get("rows_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'rows_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rows_limit_percent: {
                        let field_value = match fields_map.get("rows_limit_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'rows_limit_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sample_method: {
                        let field_value = match fields_map.get("sample_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'sample_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#table_reference: {
                        let field_value = match fields_map.get("table_reference") {
                            Some(value) => value,
                            None => bail!("Missing field 'table_reference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
