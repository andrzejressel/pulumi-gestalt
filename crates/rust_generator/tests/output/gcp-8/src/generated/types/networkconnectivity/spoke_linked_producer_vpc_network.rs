#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpokeLinkedProducerVpcNetwork {
    /// IP ranges encompassing the subnets to be excluded from peering.
    #[builder(into)]
    #[serde(rename = "excludeExportRanges")]
    pub r#exclude_export_ranges: Option<Vec<String>>,
    /// IP ranges allowed to be included from peering.
    #[builder(into)]
    #[serde(rename = "includeExportRanges")]
    pub r#include_export_ranges: Option<Vec<String>>,
    /// The URI of the Service Consumer VPC that the Producer VPC is peered with.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: String,
    /// The name of the VPC peering between the Service Consumer VPC and the Producer VPC (defined in the Tenant project) which is added to the NCC hub. This peering must be in ACTIVE state.
    #[builder(into)]
    #[serde(rename = "peering")]
    pub r#peering: String,
    /// (Output)
    /// The URI of the Producer VPC.
    #[builder(into)]
    #[serde(rename = "producerNetwork")]
    pub r#producer_network: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpokeLinkedProducerVpcNetwork {
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
                "exclude_export_ranges".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exclude_export_ranges,
                )
                .await,
            );
            map.insert(
                "include_export_ranges".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_export_ranges,
                )
                .await,
            );
            map.insert(
                "network".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network,
                )
                .await,
            );
            map.insert(
                "peering".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#peering,
                )
                .await,
            );
            map.insert(
                "producer_network".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#producer_network,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SpokeLinkedProducerVpcNetwork {
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
                    r#exclude_export_ranges: {
                        let field_value = match fields_map.get("exclude_export_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclude_export_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_export_ranges: {
                        let field_value = match fields_map.get("include_export_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_export_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network: {
                        let field_value = match fields_map.get("network") {
                            Some(value) => value,
                            None => bail!("Missing field 'network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#peering: {
                        let field_value = match fields_map.get("peering") {
                            Some(value) => value,
                            None => bail!("Missing field 'peering' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#producer_network: {
                        let field_value = match fields_map.get("producer_network") {
                            Some(value) => value,
                            None => bail!("Missing field 'producer_network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
