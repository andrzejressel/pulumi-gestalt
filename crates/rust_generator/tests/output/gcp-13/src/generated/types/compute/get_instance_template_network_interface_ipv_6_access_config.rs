#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetInstanceTemplateNetworkInterfaceIpv6AccessConfig {
    /// The first IPv6 address of the external IPv6 range associated with this instance, prefix length is stored in externalIpv6PrefixLength in ipv6AccessConfig. The field is output only, an IPv6 address from a subnetwork associated with the instance will be allocated dynamically.
    #[builder(into)]
    #[serde(rename = "externalIpv6")]
    pub r#external_ipv_6: String,
    /// The prefix length of the external IPv6 range.
    #[builder(into)]
    #[serde(rename = "externalIpv6PrefixLength")]
    pub r#external_ipv_6_prefix_length: String,
    /// The name of the instance template. One of `name`, `filter` or `self_link_unique` must be provided.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The [networking tier][network-tier] used for configuring
    /// this instance template. This field can take the following values: PREMIUM or
    /// STANDARD. If this field is not specified, it is assumed to be PREMIUM.
    #[builder(into)]
    #[serde(rename = "networkTier")]
    pub r#network_tier: String,
    /// The domain name to be used when creating DNSv6 records for the external IPv6 ranges.
    #[builder(into)]
    #[serde(rename = "publicPtrDomainName")]
    pub r#public_ptr_domain_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetInstanceTemplateNetworkInterfaceIpv6AccessConfig {
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
                    "external_ipv_6",
                    &self.r#external_ipv_6,
                ),
                to_pulumi_object_field(
                    "external_ipv_6_prefix_length",
                    &self.r#external_ipv_6_prefix_length,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
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
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetInstanceTemplateNetworkInterfaceIpv6AccessConfig {
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
                    r#external_ipv_6: {
                        let field_value = match fields_map.get("external_ipv_6") {
                            Some(value) => value,
                            None => bail!("Missing field 'external_ipv_6' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#external_ipv_6_prefix_length: {
                        let field_value = match fields_map.get("external_ipv_6_prefix_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'external_ipv_6_prefix_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
