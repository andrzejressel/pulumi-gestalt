#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatascanDataProfileSpec {
    /// The fields to exclude from data profile.
    /// If specified, the fields will be excluded from data profile, regardless of `include_fields` value.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "excludeFields")]
    pub r#exclude_fields: Option<Box<super::super::types::dataplex::DatascanDataProfileSpecExcludeFields>>,
    /// The fields to include in data profile.
    /// If not specified, all fields at the time of profile scan job execution are included, except for ones listed in `exclude_fields`.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "includeFields")]
    pub r#include_fields: Option<Box<super::super::types::dataplex::DatascanDataProfileSpecIncludeFields>>,
    /// Actions to take upon job completion.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "postScanActions")]
    pub r#post_scan_actions: Option<Box<super::super::types::dataplex::DatascanDataProfileSpecPostScanActions>>,
    /// A filter applied to all rows in a single DataScan job. The filter needs to be a valid SQL expression for a WHERE clause in BigQuery standard SQL syntax. Example: col1 >= 0 AND col2 < 10
    #[builder(into)]
    #[serde(rename = "rowFilter")]
    pub r#row_filter: Option<String>,
    /// The percentage of the records to be selected from the dataset for DataScan.
    /// Value can range between 0.0 and 100.0 with up to 3 significant decimal digits.
    /// Sampling is not applied if `sampling_percent` is not specified, 0 or 100.
    #[builder(into)]
    #[serde(rename = "samplingPercent")]
    pub r#sampling_percent: Option<f64>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DatascanDataProfileSpec {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "exclude_fields".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exclude_fields,
                )
                .await,
            );
            map.insert(
                "include_fields".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_fields,
                )
                .await,
            );
            map.insert(
                "post_scan_actions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#post_scan_actions,
                )
                .await,
            );
            map.insert(
                "row_filter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#row_filter,
                )
                .await,
            );
            map.insert(
                "sampling_percent".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sampling_percent,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DatascanDataProfileSpec {
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
                    r#exclude_fields: {
                        let field_value = match fields_map.get("exclude_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclude_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_fields: {
                        let field_value = match fields_map.get("include_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#post_scan_actions: {
                        let field_value = match fields_map.get("post_scan_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'post_scan_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#row_filter: {
                        let field_value = match fields_map.get("row_filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'row_filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sampling_percent: {
                        let field_value = match fields_map.get("sampling_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'sampling_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
