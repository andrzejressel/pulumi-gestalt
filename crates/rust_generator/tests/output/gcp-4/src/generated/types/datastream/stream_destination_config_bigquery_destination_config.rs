#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamDestinationConfigBigqueryDestinationConfig {
    /// AppendOnly mode defines that the stream of changes (INSERT, UPDATE-INSERT, UPDATE-DELETE and DELETE
    /// events) to a source table will be written to the destination Google BigQuery table, retaining the
    /// historical state of the data.
    #[builder(into)]
    #[serde(rename = "appendOnly")]
    pub r#append_only: Option<Box<super::super::types::datastream::StreamDestinationConfigBigqueryDestinationConfigAppendOnly>>,
    /// The guaranteed data freshness (in seconds) when querying tables created by the stream.
    /// Editing this field will only affect new tables created in the future, but existing tables
    /// will not be impacted. Lower values mean that queries will return fresher data, but may result in higher cost.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s". Defaults to 900s.
    #[builder(into)]
    #[serde(rename = "dataFreshness")]
    pub r#data_freshness: Option<String>,
    /// Merge mode defines that all changes to a table will be merged at the destination Google BigQuery
    /// table. This is the default write mode. When selected, BigQuery reflects the way the data is stored
    /// in the source database. With Merge mode, no historical record of the change events is kept.
    #[builder(into)]
    #[serde(rename = "merge")]
    pub r#merge: Option<Box<super::super::types::datastream::StreamDestinationConfigBigqueryDestinationConfigMerge>>,
    /// A single target dataset to which all data will be streamed.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "singleTargetDataset")]
    pub r#single_target_dataset: Option<Box<super::super::types::datastream::StreamDestinationConfigBigqueryDestinationConfigSingleTargetDataset>>,
    /// Destination datasets are created so that hierarchy of the destination data objects matches the source hierarchy.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sourceHierarchyDatasets")]
    pub r#source_hierarchy_datasets: Option<Box<super::super::types::datastream::StreamDestinationConfigBigqueryDestinationConfigSourceHierarchyDatasets>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StreamDestinationConfigBigqueryDestinationConfig {
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
                    "append_only",
                    &self.r#append_only,
                ),
                to_pulumi_object_field(
                    "data_freshness",
                    &self.r#data_freshness,
                ),
                to_pulumi_object_field(
                    "merge",
                    &self.r#merge,
                ),
                to_pulumi_object_field(
                    "single_target_dataset",
                    &self.r#single_target_dataset,
                ),
                to_pulumi_object_field(
                    "source_hierarchy_datasets",
                    &self.r#source_hierarchy_datasets,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StreamDestinationConfigBigqueryDestinationConfig {
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
                    r#append_only: {
                        let field_value = match fields_map.get("append_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'append_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_freshness: {
                        let field_value = match fields_map.get("data_freshness") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_freshness' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#merge: {
                        let field_value = match fields_map.get("merge") {
                            Some(value) => value,
                            None => bail!("Missing field 'merge' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#single_target_dataset: {
                        let field_value = match fields_map.get("single_target_dataset") {
                            Some(value) => value,
                            None => bail!("Missing field 'single_target_dataset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_hierarchy_datasets: {
                        let field_value = match fields_map.get("source_hierarchy_datasets") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_hierarchy_datasets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
