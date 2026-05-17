#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkInsightsAnalysisForwardPathComponentOutboundHeader {
    #[builder(into)]
    #[serde(rename = "destinationAddresses")]
    pub r#destination_addresses: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "destinationPortRanges")]
    pub r#destination_port_ranges: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponentOutboundHeaderDestinationPortRange>>,
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
    #[builder(into)]
    #[serde(rename = "sourceAddresses")]
    pub r#source_addresses: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "sourcePortRanges")]
    pub r#source_port_ranges: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponentOutboundHeaderSourcePortRange>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkInsightsAnalysisForwardPathComponentOutboundHeader {
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
                "destination_addresses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destination_addresses,
                )
                .await,
            );
            map.insert(
                "destination_port_ranges".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destination_port_ranges,
                )
                .await,
            );
            map.insert(
                "protocol".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#protocol,
                )
                .await,
            );
            map.insert(
                "source_addresses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_addresses,
                )
                .await,
            );
            map.insert(
                "source_port_ranges".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_port_ranges,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkInsightsAnalysisForwardPathComponentOutboundHeader {
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
                    r#destination_addresses: {
                        let field_value = match fields_map.get("destination_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_port_ranges: {
                        let field_value = match fields_map.get("destination_port_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_port_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocol: {
                        let field_value = match fields_map.get("protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_addresses: {
                        let field_value = match fields_map.get("source_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_port_ranges: {
                        let field_value = match fields_map.get("source_port_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_port_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
