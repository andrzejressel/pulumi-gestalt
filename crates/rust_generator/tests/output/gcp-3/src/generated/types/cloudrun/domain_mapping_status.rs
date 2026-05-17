#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainMappingStatus {
    /// (Output)
    /// Array of observed DomainMappingConditions, indicating the current state
    /// of the DomainMapping.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "conditions")]
    pub r#conditions: Option<Vec<super::super::types::cloudrun::DomainMappingStatusCondition>>,
    /// (Output)
    /// The name of the route that the mapping currently points to.
    #[builder(into)]
    #[serde(rename = "mappedRouteName")]
    pub r#mapped_route_name: Option<String>,
    /// (Output)
    /// ObservedGeneration is the 'Generation' of the DomainMapping that
    /// was last processed by the controller.
    #[builder(into)]
    #[serde(rename = "observedGeneration")]
    pub r#observed_generation: Option<i32>,
    /// The resource records required to configure this domain mapping. These
    /// records must be added to the domain's DNS configuration in order to
    /// serve the application via this domain mapping.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "resourceRecords")]
    pub r#resource_records: Option<Vec<super::super::types::cloudrun::DomainMappingStatusResourceRecord>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainMappingStatus {
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
                "conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#conditions,
                )
                .await,
            );
            map.insert(
                "mapped_route_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mapped_route_name,
                )
                .await,
            );
            map.insert(
                "observed_generation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#observed_generation,
                )
                .await,
            );
            map.insert(
                "resource_records".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_records,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainMappingStatus {
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
                    r#conditions: {
                        let field_value = match fields_map.get("conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mapped_route_name: {
                        let field_value = match fields_map.get("mapped_route_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'mapped_route_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#observed_generation: {
                        let field_value = match fields_map.get("observed_generation") {
                            Some(value) => value,
                            None => bail!("Missing field 'observed_generation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_records: {
                        let field_value = match fields_map.get("resource_records") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_records' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
