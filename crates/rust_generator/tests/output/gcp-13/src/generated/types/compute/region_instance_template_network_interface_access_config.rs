#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionInstanceTemplateNetworkInterfaceAccessConfig {
    /// The IP address that will be 1:1 mapped to the instance's
    /// network ip. If not given, one will be generated.
    #[builder(into)]
    #[serde(rename = "natIp")]
    pub r#nat_ip: Option<String>,
    /// The service-level to be provided for IPv6 traffic when the
    /// subnet has an external subnet. Only PREMIUM and STANDARD tier is valid for IPv6.
    #[builder(into)]
    #[serde(rename = "networkTier")]
    pub r#network_tier: Option<String>,
    /// The DNS domain name for the public PTR record.The DNS domain name for the public PTR record.
    #[builder(into)]
    #[serde(rename = "publicPtrDomainName")]
    pub r#public_ptr_domain_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegionInstanceTemplateNetworkInterfaceAccessConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
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
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegionInstanceTemplateNetworkInterfaceAccessConfig {
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
