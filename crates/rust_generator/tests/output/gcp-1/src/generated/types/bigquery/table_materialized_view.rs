#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableMaterializedView {
    /// Allow non incremental materialized view definition.
    /// The default value is false.
    #[builder(into)]
    #[serde(rename = "allowNonIncrementalDefinition")]
    pub r#allow_non_incremental_definition: Option<bool>,
    /// Specifies whether to use BigQuery's automatic refresh for this materialized view when the base table is updated.
    /// The default value is true.
    #[builder(into)]
    #[serde(rename = "enableRefresh")]
    pub r#enable_refresh: Option<bool>,
    /// A query whose result is persisted.
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: String,
    /// The maximum frequency at which this materialized view will be refreshed.
    /// The default value is 1800000
    #[builder(into)]
    #[serde(rename = "refreshIntervalMs")]
    pub r#refresh_interval_ms: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TableMaterializedView {
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
                    "allow_non_incremental_definition",
                    &self.r#allow_non_incremental_definition,
                ),
                to_pulumi_object_field(
                    "enable_refresh",
                    &self.r#enable_refresh,
                ),
                to_pulumi_object_field(
                    "query",
                    &self.r#query,
                ),
                to_pulumi_object_field(
                    "refresh_interval_ms",
                    &self.r#refresh_interval_ms,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableMaterializedView {
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
                    r#allow_non_incremental_definition: {
                        let field_value = match fields_map.get("allow_non_incremental_definition") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_non_incremental_definition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_refresh: {
                        let field_value = match fields_map.get("enable_refresh") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_refresh' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query: {
                        let field_value = match fields_map.get("query") {
                            Some(value) => value,
                            None => bail!("Missing field 'query' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#refresh_interval_ms: {
                        let field_value = match fields_map.get("refresh_interval_ms") {
                            Some(value) => value,
                            None => bail!("Missing field 'refresh_interval_ms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
