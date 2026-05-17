#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDiscoveryConfigTargetBigQueryTargetCadence {
    /// Governs when to update data profiles when the inspection rules defined by the `InspectTemplate` change. If not set, changing the template will not cause a data profile to update.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "inspectTemplateModifiedCadence")]
    pub r#inspect_template_modified_cadence: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetCadenceInspectTemplateModifiedCadence>>,
    /// Governs when to update data profiles when a schema is modified
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "schemaModifiedCadence")]
    pub r#schema_modified_cadence: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetCadenceSchemaModifiedCadence>>,
    /// Governs when to update profile when a table is modified.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "tableModifiedCadence")]
    pub r#table_modified_cadence: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetCadenceTableModifiedCadence>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDiscoveryConfigTargetBigQueryTargetCadence {
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
                "inspect_template_modified_cadence".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#inspect_template_modified_cadence,
                )
                .await,
            );
            map.insert(
                "schema_modified_cadence".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schema_modified_cadence,
                )
                .await,
            );
            map.insert(
                "table_modified_cadence".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#table_modified_cadence,
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
                        let field_value = match fields_map.get("inspect_template_modified_cadence") {
                            Some(value) => value,
                            None => bail!("Missing field 'inspect_template_modified_cadence' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schema_modified_cadence: {
                        let field_value = match fields_map.get("schema_modified_cadence") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema_modified_cadence' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#table_modified_cadence: {
                        let field_value = match fields_map.get("table_modified_cadence") {
                            Some(value) => value,
                            None => bail!("Missing field 'table_modified_cadence' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
