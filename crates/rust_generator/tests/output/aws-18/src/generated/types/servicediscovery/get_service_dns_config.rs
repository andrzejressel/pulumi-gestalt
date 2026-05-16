#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceDnsConfig {
    /// An array that contains one DnsRecord object for each resource record set. See `dns_records` Block for details.
    #[builder(into)]
    #[serde(rename = "dnsRecords")]
    pub r#dns_records: Vec<super::super::types::servicediscovery::GetServiceDnsConfigDnsRecord>,
    /// ID of the namespace that the service belongs to.
    #[builder(into)]
    #[serde(rename = "namespaceId")]
    pub r#namespace_id: String,
    /// Routing policy that you want to apply to all records that Route 53 creates when you register an instance and specify the service. Valid Values: MULTIVALUE, WEIGHTED
    #[builder(into)]
    #[serde(rename = "routingPolicy")]
    pub r#routing_policy: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetServiceDnsConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("dns_records".to_string(), self.r#dns_records.to_pulumi_value().await);
            map.insert("namespace_id".to_string(), self.r#namespace_id.to_pulumi_value().await);
            map.insert("routing_policy".to_string(), self.r#routing_policy.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetServiceDnsConfig {
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
                    r#dns_records: {
                        let field_value = match fields_map.get("dns_records") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_records' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::servicediscovery::GetServiceDnsConfigDnsRecord> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#namespace_id: {
                        let field_value = match fields_map.get("namespace_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'namespace_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#routing_policy: {
                        let field_value = match fields_map.get("routing_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'routing_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
