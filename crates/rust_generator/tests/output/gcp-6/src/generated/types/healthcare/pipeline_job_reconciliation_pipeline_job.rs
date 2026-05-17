#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipelineJobReconciliationPipelineJob {
    /// The harmonized FHIR store to write harmonized FHIR resources to,
    /// in the format of: project/{projectID}/locations/{locationID}/datasets/{datasetName}/fhirStores/{id}
    #[builder(into)]
    #[serde(rename = "fhirStoreDestination")]
    pub r#fhir_store_destination: Option<String>,
    /// Specifies the top level directory of the matching configs used
    /// in all mapping pipelines, which extract properties for resources
    /// to be matched on.
    /// Example: gs://{bucket-id}/{path/to/matching/configs}
    #[builder(into)]
    #[serde(rename = "matchingUriPrefix")]
    pub r#matching_uri_prefix: String,
    /// Specifies the location of the reconciliation configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "mergeConfig")]
    pub r#merge_config: Box<super::super::types::healthcare::PipelineJobReconciliationPipelineJobMergeConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipelineJobReconciliationPipelineJob {
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
                "matching_uri_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#matching_uri_prefix,
                )
                .await,
            );
            map.insert(
                "merge_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#merge_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipelineJobReconciliationPipelineJob {
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
                    r#matching_uri_prefix: {
                        let field_value = match fields_map.get("matching_uri_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'matching_uri_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#merge_config: {
                        let field_value = match fields_map.get("merge_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'merge_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
