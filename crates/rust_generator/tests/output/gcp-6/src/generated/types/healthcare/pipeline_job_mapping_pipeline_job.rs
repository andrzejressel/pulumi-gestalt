#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipelineJobMappingPipelineJob {
    /// If set, the mapping pipeline will write snapshots to this
    /// FHIR store without assigning stable IDs. You must
    /// grant your pipeline project's Cloud Healthcare Service
    /// Agent serviceaccount healthcare.fhirResources.executeBundle
    /// and healthcare.fhirResources.create permissions on the
    /// destination store. The destination store must set
    /// [disableReferentialIntegrity][FhirStore.disable_referential_integrity]
    /// to true. The destination store must use FHIR version R4.
    /// Format: project/{projectID}/locations/{locationID}/datasets/{datasetName}/fhirStores/{fhirStoreID}.
    #[builder(into)]
    #[serde(rename = "fhirStoreDestination")]
    pub r#fhir_store_destination: Option<String>,
    /// A streaming FHIR data source.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fhirStreamingSource")]
    pub r#fhir_streaming_source: Option<Box<super::super::types::healthcare::PipelineJobMappingPipelineJobFhirStreamingSource>>,
    /// The location of the mapping configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "mappingConfig")]
    pub r#mapping_config: Box<super::super::types::healthcare::PipelineJobMappingPipelineJobMappingConfig>,
    /// If set to true, a mapping pipeline will send output snapshots
    /// to the reconciliation pipeline in its dataset. A reconciliation
    /// pipeline must exist in this dataset before a mapping pipeline
    /// with a reconciliation destination can be created.
    #[builder(into)]
    #[serde(rename = "reconciliationDestination")]
    pub r#reconciliation_destination: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipelineJobMappingPipelineJob {
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
                "fhir_store_destination".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fhir_store_destination,
                )
                .await,
            );
            map.insert(
                "fhir_streaming_source".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fhir_streaming_source,
                )
                .await,
            );
            map.insert(
                "mapping_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mapping_config,
                )
                .await,
            );
            map.insert(
                "reconciliation_destination".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reconciliation_destination,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipelineJobMappingPipelineJob {
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
                    r#fhir_store_destination: {
                        let field_value = match fields_map.get("fhir_store_destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'fhir_store_destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fhir_streaming_source: {
                        let field_value = match fields_map.get("fhir_streaming_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'fhir_streaming_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mapping_config: {
                        let field_value = match fields_map.get("mapping_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'mapping_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reconciliation_destination: {
                        let field_value = match fields_map.get("reconciliation_destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'reconciliation_destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
