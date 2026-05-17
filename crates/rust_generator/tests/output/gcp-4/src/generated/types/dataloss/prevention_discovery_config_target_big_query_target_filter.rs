#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDiscoveryConfigTargetBigQueryTargetFilter {
    /// Catch-all. This should always be the last filter in the list because anything above it will apply first.
    #[builder(into)]
    #[serde(rename = "otherTables")]
    pub r#other_tables: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetFilterOtherTables>>,
    /// The table to scan. Discovery configurations including this can only include one DiscoveryTarget (the DiscoveryTarget with this TableReference).
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "tableReference")]
    pub r#table_reference: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetFilterTableReference>>,
    /// A specific set of tables for this filter to apply to. A table collection must be specified in only one filter per config.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "tables")]
    pub r#tables: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetFilterTables>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDiscoveryConfigTargetBigQueryTargetFilter {
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
                    "other_tables",
                    &self.r#other_tables,
                ),
                to_pulumi_object_field(
                    "table_reference",
                    &self.r#table_reference,
                ),
                to_pulumi_object_field(
                    "tables",
                    &self.r#tables,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionDiscoveryConfigTargetBigQueryTargetFilter {
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
                    r#other_tables: {
                        let field_value = match fields_map.get("other_tables") {
                            Some(value) => value,
                            None => bail!("Missing field 'other_tables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#tables: {
                        let field_value = match fields_map.get("tables") {
                            Some(value) => value,
                            None => bail!("Missing field 'tables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
