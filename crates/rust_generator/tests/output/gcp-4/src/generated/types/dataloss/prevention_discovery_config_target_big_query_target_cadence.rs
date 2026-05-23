#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDiscoveryConfigTargetBigQueryTargetCadence {
    /// Governs when to update data profiles when the inspection rules defined by the `InspectTemplate` change. If not set, changing the template will not cause a data profile to update.
    /// Structure is documented below.
    #[builder(into)]
    pub r#inspect_template_modified_cadence: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetCadenceInspectTemplateModifiedCadence>>,
    /// Governs when to update data profiles when a schema is modified
    /// Structure is documented below.
    #[builder(into)]
    pub r#schema_modified_cadence: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetCadenceSchemaModifiedCadence>>,
    /// Governs when to update profile when a table is modified.
    /// Structure is documented below.
    #[builder(into)]
    pub r#table_modified_cadence: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetCadenceTableModifiedCadence>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDiscoveryConfigTargetBigQueryTargetCadence {
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
                    "inspectTemplateModifiedCadence",
                    &self.r#inspect_template_modified_cadence,
                ),
                to_pulumi_object_field(
                    "schemaModifiedCadence",
                    &self.r#schema_modified_cadence,
                ),
                to_pulumi_object_field(
                    "tableModifiedCadence",
                    &self.r#table_modified_cadence,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionDiscoveryConfigTargetBigQueryTargetCadence {
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
                    r#inspect_template_modified_cadence: {
                        let field_value = match fields_map.get("inspectTemplateModifiedCadence") {
                            Some(value) => value,
                            None => bail!("Missing field 'inspectTemplateModifiedCadence' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schema_modified_cadence: {
                        let field_value = match fields_map.get("schemaModifiedCadence") {
                            Some(value) => value,
                            None => bail!("Missing field 'schemaModifiedCadence' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#table_modified_cadence: {
                        let field_value = match fields_map.get("tableModifiedCadence") {
                            Some(value) => value,
                            None => bail!("Missing field 'tableModifiedCadence' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
