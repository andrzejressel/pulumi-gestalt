#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetResolverFirewallRulesFirewallRule {
    /// The action that DNS Firewall should take on a DNS query when it matches one of the domains in the rule's domain list.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// The DNS record's type.
    #[builder(into)]
    #[serde(rename = "blockOverrideDnsType")]
    pub r#block_override_dns_type: String,
    /// The custom DNS record to send back in response to the query.
    #[builder(into)]
    #[serde(rename = "blockOverrideDomain")]
    pub r#block_override_domain: String,
    /// The recommended amount of time, in seconds, for the DNS resolver or web browser to cache the provided override record.
    #[builder(into)]
    #[serde(rename = "blockOverrideTtl")]
    pub r#block_override_ttl: i32,
    /// The way that you want DNS Firewall to block the request.
    #[builder(into)]
    #[serde(rename = "blockResponse")]
    pub r#block_response: String,
    /// The date and time that the rule was created, in Unix time format and Coordinated Universal Time (UTC).
    #[builder(into)]
    #[serde(rename = "creationTime")]
    pub r#creation_time: String,
    /// A unique string defined by you to identify the request.
    #[builder(into)]
    #[serde(rename = "creatorRequestId")]
    pub r#creator_request_id: String,
    /// The ID of the domain list that's used in the rule.
    #[builder(into)]
    #[serde(rename = "firewallDomainListId")]
    pub r#firewall_domain_list_id: String,
    /// The unique identifier of the firewall rule group that you want to retrieve the rules for.
    #[builder(into)]
    #[serde(rename = "firewallRuleGroupId")]
    pub r#firewall_rule_group_id: String,
    /// The date and time that the rule was last modified, in Unix time format and Coordinated Universal Time (UTC).
    #[builder(into)]
    #[serde(rename = "modificationTime")]
    pub r#modification_time: String,
    /// The name of the rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The setting that determines the processing order of the rules in a rule group.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetResolverFirewallRulesFirewallRule {
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
                "action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#action,
                )
                .await,
            );
            map.insert(
                "block_override_dns_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#block_override_dns_type,
                )
                .await,
            );
            map.insert(
                "block_override_domain".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#block_override_domain,
                )
                .await,
            );
            map.insert(
                "block_override_ttl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#block_override_ttl,
                )
                .await,
            );
            map.insert(
                "block_response".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#block_response,
                )
                .await,
            );
            map.insert(
                "creation_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#creation_time,
                )
                .await,
            );
            map.insert(
                "creator_request_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#creator_request_id,
                )
                .await,
            );
            map.insert(
                "firewall_domain_list_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#firewall_domain_list_id,
                )
                .await,
            );
            map.insert(
                "firewall_rule_group_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#firewall_rule_group_id,
                )
                .await,
            );
            map.insert(
                "modification_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#modification_time,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "priority".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#priority,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetResolverFirewallRulesFirewallRule {
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
                    r#action: {
                        let field_value = match fields_map.get("action") {
                            Some(value) => value,
                            None => bail!("Missing field 'action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#block_override_dns_type: {
                        let field_value = match fields_map.get("block_override_dns_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'block_override_dns_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#block_override_domain: {
                        let field_value = match fields_map.get("block_override_domain") {
                            Some(value) => value,
                            None => bail!("Missing field 'block_override_domain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#block_override_ttl: {
                        let field_value = match fields_map.get("block_override_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'block_override_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#block_response: {
                        let field_value = match fields_map.get("block_response") {
                            Some(value) => value,
                            None => bail!("Missing field 'block_response' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#creation_time: {
                        let field_value = match fields_map.get("creation_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'creation_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#creator_request_id: {
                        let field_value = match fields_map.get("creator_request_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'creator_request_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#firewall_domain_list_id: {
                        let field_value = match fields_map.get("firewall_domain_list_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'firewall_domain_list_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#firewall_rule_group_id: {
                        let field_value = match fields_map.get("firewall_rule_group_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'firewall_rule_group_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#modification_time: {
                        let field_value = match fields_map.get("modification_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'modification_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#priority: {
                        let field_value = match fields_map.get("priority") {
                            Some(value) => value,
                            None => bail!("Missing field 'priority' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
