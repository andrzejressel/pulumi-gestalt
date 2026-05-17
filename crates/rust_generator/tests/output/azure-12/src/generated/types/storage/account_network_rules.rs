#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountNetworkRules {
    /// Specifies whether traffic is bypassed for Logging/Metrics/AzureServices. Valid options are any combination of `Logging`, `Metrics`, `AzureServices`, or `None`.
    #[builder(into)]
    #[serde(rename = "bypasses")]
    pub r#bypasses: Option<Vec<String>>,
    /// Specifies the default action of allow or deny when no other rules match. Valid options are `Deny` or `Allow`.
    #[builder(into)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: String,
    /// List of public IP or IP ranges in CIDR Format. Only IPv4 addresses are allowed. /31 CIDRs, /32 CIDRs, and Private IP address ranges (as defined in [RFC 1918](https://tools.ietf.org/html/rfc1918#section-3)), are not allowed.
    #[builder(into)]
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Option<Vec<String>>,
    /// One or more `private_link_access` block as defined below.
    /// 
    /// > **Note:** If specifying `network_rules`, one of either `ip_rules` or `virtual_network_subnet_ids` must be specified and `default_action` must be set to `Deny`.
    /// 
    /// > **Note:** Network Rules can be defined either directly on the `azure.storage.Account` resource, or using the `azure.storage.AccountNetworkRules` resource - but the two cannot be used together. If both are used against the same Storage Account, spurious changes will occur. When managing Network Rules using this resource, to change from a `default_action` of `Deny` to `Allow` requires defining, rather than removing, the block.
    /// 
    /// > **Note:** The prefix of `ip_rules` must be between 0 and 30 and only supports public IP addresses.
    /// 
    /// > **Note:** [More information on Validation is available here](https://docs.microsoft.com/en-gb/azure/storage/blobs/storage-custom-domain-name)
    #[builder(into)]
    #[serde(rename = "privateLinkAccesses")]
    pub r#private_link_accesses: Option<Vec<super::super::types::storage::AccountNetworkRulesPrivateLinkAccess>>,
    /// A list of resource ids for subnets.
    #[builder(into)]
    #[serde(rename = "virtualNetworkSubnetIds")]
    pub r#virtual_network_subnet_ids: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccountNetworkRules {
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
                "bypasses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bypasses,
                )
                .await,
            );
            map.insert(
                "default_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_action,
                )
                .await,
            );
            map.insert(
                "ip_rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_rules,
                )
                .await,
            );
            map.insert(
                "private_link_accesses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_link_accesses,
                )
                .await,
            );
            map.insert(
                "virtual_network_subnet_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#virtual_network_subnet_ids,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccountNetworkRules {
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
                    r#bypasses: {
                        let field_value = match fields_map.get("bypasses") {
                            Some(value) => value,
                            None => bail!("Missing field 'bypasses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_action: {
                        let field_value = match fields_map.get("default_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_rules: {
                        let field_value = match fields_map.get("ip_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_link_accesses: {
                        let field_value = match fields_map.get("private_link_accesses") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_link_accesses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_network_subnet_ids: {
                        let field_value = match fields_map.get("virtual_network_subnet_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_network_subnet_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
