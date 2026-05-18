#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstancePrivateDnsNameOptions {
    /// Indicates whether to respond to DNS queries for instance hostnames with DNS A records.
    #[builder(into)]
    #[serde(rename = "enableResourceNameDnsARecord")]
    pub r#enable_resource_name_dns_a_record: Option<bool>,
    /// Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA records.
    #[builder(into)]
    #[serde(rename = "enableResourceNameDnsAaaaRecord")]
    pub r#enable_resource_name_dns_aaaa_record: Option<bool>,
    /// Type of hostname for Amazon EC2 instances. For IPv4 only subnets, an instance DNS name must be based on the instance IPv4 address. For IPv6 native subnets, an instance DNS name must be based on the instance ID. For dual-stack subnets, you can specify whether DNS names use the instance IPv4 address or the instance ID. Valid values: `ip-name` and `resource-name`.
    #[builder(into)]
    #[serde(rename = "hostnameType")]
    pub r#hostname_type: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstancePrivateDnsNameOptions {
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
                    "enable_resource_name_dns_a_record",
                    &self.r#enable_resource_name_dns_a_record,
                ),
                to_pulumi_object_field(
                    "enable_resource_name_dns_aaaa_record",
                    &self.r#enable_resource_name_dns_aaaa_record,
                ),
                to_pulumi_object_field(
                    "hostname_type",
                    &self.r#hostname_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstancePrivateDnsNameOptions {
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
                    r#enable_resource_name_dns_a_record: {
                        let field_value = match fields_map.get("enable_resource_name_dns_a_record") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_resource_name_dns_a_record' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_resource_name_dns_aaaa_record: {
                        let field_value = match fields_map.get("enable_resource_name_dns_aaaa_record") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_resource_name_dns_aaaa_record' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hostname_type: {
                        let field_value = match fields_map.get("hostname_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostname_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
