#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceNetworkInterfaceAccessConfig {
    /// The IP address that will be 1:1 mapped to the instance's
    /// network ip. If not given, one will be generated.
    #[builder(into)]
    #[serde(rename = "natIp")]
    pub r#nat_ip: Option<String>,
    /// The service-level to be provided for IPv6 traffic when the
    /// subnet has an external subnet. Only PREMIUM or STANDARD tier is valid for IPv6.
    #[builder(into)]
    #[serde(rename = "networkTier")]
    pub r#network_tier: Option<String>,
    /// The domain name to be used when creating DNSv6
    /// records for the external IPv6 ranges..
    #[builder(into)]
    #[serde(rename = "publicPtrDomainName")]
    pub r#public_ptr_domain_name: Option<String>,
    /// A full or partial URL to a security policy to add to this instance. If this field is set to an empty string it will remove the associated security policy.
    #[builder(into)]
    #[serde(rename = "securityPolicy")]
    pub r#security_policy: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceNetworkInterfaceAccessConfig {
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
                    "nat_ip",
                    &self.r#nat_ip,
                ),
                to_pulumi_object_field(
                    "network_tier",
                    &self.r#network_tier,
                ),
                to_pulumi_object_field(
                    "public_ptr_domain_name",
                    &self.r#public_ptr_domain_name,
                ),
                to_pulumi_object_field(
                    "security_policy",
                    &self.r#security_policy,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceNetworkInterfaceAccessConfig {
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
                    r#nat_ip: {
                        let field_value = match fields_map.get("nat_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'nat_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_tier: {
                        let field_value = match fields_map.get("network_tier") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_tier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ptr_domain_name: {
                        let field_value = match fields_map.get("public_ptr_domain_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_ptr_domain_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_policy: {
                        let field_value = match fields_map.get("security_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
