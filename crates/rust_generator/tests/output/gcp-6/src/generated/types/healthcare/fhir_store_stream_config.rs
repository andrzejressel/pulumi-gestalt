#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FhirStoreStreamConfig {
    /// The destination BigQuery structure that contains both the dataset location and corresponding schema config.
    /// The output is organized in one table per resource type. The server reuses the existing tables (if any) that
    /// are named after the resource types, e.g. "Patient", "Observation". When there is no existing table for a given
    /// resource type, the server attempts to create one.
    /// See the [streaming config reference](https://cloud.google.com/healthcare/docs/reference/rest/v1beta1/projects.locations.datasets.fhirStores#streamconfig) for more details.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "bigqueryDestination")]
    pub r#bigquery_destination: Box<super::super::types::healthcare::FhirStoreStreamConfigBigqueryDestination>,
    /// Supply a FHIR resource type (such as "Patient" or "Observation"). See
    /// https://www.hl7.org/fhir/valueset-resource-types.html for a list of all FHIR resource types. The server treats
    /// an empty list as an intent to stream all the supported resource types in this FHIR store.
    #[builder(into)]
    #[serde(rename = "resourceTypes")]
    pub r#resource_types: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FhirStoreStreamConfig {
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
                "bigquery_destination".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bigquery_destination,
                )
                .await,
            );
            map.insert(
                "resource_types".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_types,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FhirStoreStreamConfig {
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
                    r#bigquery_destination: {
                        let field_value = match fields_map.get("bigquery_destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'bigquery_destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_types: {
                        let field_value = match fields_map.get("resource_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
