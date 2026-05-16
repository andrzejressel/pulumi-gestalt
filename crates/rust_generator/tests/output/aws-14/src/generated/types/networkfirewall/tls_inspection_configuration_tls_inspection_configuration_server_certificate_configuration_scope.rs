#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScope {
    /// Set of configuration blocks describing the destination ports to inspect for. If not specified, this matches with any destination port. See Destination Ports below for details.
    #[builder(into)]
    #[serde(rename = "destinationPorts")]
    pub r#destination_ports: Option<Vec<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeDestinationPort>>,
    /// Set of configuration blocks describing the destination IP address and address ranges to inspect for, in CIDR notation. If not specified, this matches with any destination address. See Destination below for details.
    #[builder(into)]
    #[serde(rename = "destinations")]
    pub r#destinations: Option<Vec<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeDestination>>,
    /// Set of protocols to inspect for, specified using the protocol's assigned internet protocol number (IANA). Network Firewall currently supports TCP only. Valid values: `6`
    #[builder(into)]
    #[serde(rename = "protocols")]
    pub r#protocols: Vec<i32>,
    /// Set of configuration blocks describing the source ports to inspect for. If not specified, this matches with any source port. See Source Ports below for details.
    #[builder(into)]
    #[serde(rename = "sourcePorts")]
    pub r#source_ports: Option<Vec<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeSourcePort>>,
    /// Set of configuration blocks describing the source IP address and address ranges to inspect for, in CIDR notation. If not specified, this matches with any source address. See Source below for details.
    #[builder(into)]
    #[serde(rename = "sources")]
    pub r#sources: Option<Vec<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeSource>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScope {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("destination_ports".to_string(), self.r#destination_ports.to_pulumi_value().await);
            map.insert("destinations".to_string(), self.r#destinations.to_pulumi_value().await);
            map.insert("protocols".to_string(), self.r#protocols.to_pulumi_value().await);
            map.insert("source_ports".to_string(), self.r#source_ports.to_pulumi_value().await);
            map.insert("sources".to_string(), self.r#sources.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScope {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#destination_ports: {
                        let field_value = match fields_map.get("destination_ports") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_ports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeDestinationPort>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#destinations: {
                        let field_value = match fields_map.get("destinations") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeDestination>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#protocols: {
                        let field_value = match fields_map.get("protocols") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocols' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#source_ports: {
                        let field_value = match fields_map.get("source_ports") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_ports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeSourcePort>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sources: {
                        let field_value = match fields_map.get("sources") {
                            Some(value) => value,
                            None => bail!("Missing field 'sources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeSource>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
