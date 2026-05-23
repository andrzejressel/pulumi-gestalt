#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamSourceConfigOracleSourceConfig {
    /// Configuration to drop large object values.
    #[builder(into)]
    pub r#drop_large_objects: Option<Box<super::super::types::datastream::StreamSourceConfigOracleSourceConfigDropLargeObjects>>,
    /// Oracle objects to exclude from the stream.
    /// Structure is documented below.
    #[builder(into)]
    pub r#exclude_objects: Option<Box<super::super::types::datastream::StreamSourceConfigOracleSourceConfigExcludeObjects>>,
    /// Oracle objects to retrieve from the source.
    /// Structure is documented below.
    #[builder(into)]
    pub r#include_objects: Option<Box<super::super::types::datastream::StreamSourceConfigOracleSourceConfigIncludeObjects>>,
    /// Maximum number of concurrent backfill tasks. The number should be non negative.
    /// If not set (or set to 0), the system's default value will be used.
    #[builder(into)]
    pub r#max_concurrent_backfill_tasks: Option<i32>,
    /// Maximum number of concurrent CDC tasks. The number should be non negative.
    /// If not set (or set to 0), the system's default value will be used.
    #[builder(into)]
    pub r#max_concurrent_cdc_tasks: Option<i32>,
    /// Configuration to drop large object values.
    #[builder(into)]
    pub r#stream_large_objects: Option<Box<super::super::types::datastream::StreamSourceConfigOracleSourceConfigStreamLargeObjects>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StreamSourceConfigOracleSourceConfig {
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
                    "dropLargeObjects",
                    &self.r#drop_large_objects,
                ),
                to_pulumi_object_field(
                    "excludeObjects",
                    &self.r#exclude_objects,
                ),
                to_pulumi_object_field(
                    "includeObjects",
                    &self.r#include_objects,
                ),
                to_pulumi_object_field(
                    "maxConcurrentBackfillTasks",
                    &self.r#max_concurrent_backfill_tasks,
                ),
                to_pulumi_object_field(
                    "maxConcurrentCdcTasks",
                    &self.r#max_concurrent_cdc_tasks,
                ),
                to_pulumi_object_field(
                    "streamLargeObjects",
                    &self.r#stream_large_objects,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StreamSourceConfigOracleSourceConfig {
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
                    r#drop_large_objects: {
                        let field_value = match fields_map.get("dropLargeObjects") {
                            Some(value) => value,
                            None => bail!("Missing field 'dropLargeObjects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exclude_objects: {
                        let field_value = match fields_map.get("excludeObjects") {
                            Some(value) => value,
                            None => bail!("Missing field 'excludeObjects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_objects: {
                        let field_value = match fields_map.get("includeObjects") {
                            Some(value) => value,
                            None => bail!("Missing field 'includeObjects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_concurrent_backfill_tasks: {
                        let field_value = match fields_map.get("maxConcurrentBackfillTasks") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxConcurrentBackfillTasks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_concurrent_cdc_tasks: {
                        let field_value = match fields_map.get("maxConcurrentCdcTasks") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxConcurrentCdcTasks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stream_large_objects: {
                        let field_value = match fields_map.get("streamLargeObjects") {
                            Some(value) => value,
                            None => bail!("Missing field 'streamLargeObjects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
