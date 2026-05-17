#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TeamsRuleRuleSettings {
    /// Add custom headers to allowed requests in the form of key-value pairs.
    #[builder(into)]
    #[serde(rename = "addHeaders")]
    pub r#add_headers: Option<std::collections::HashMap<String, String>>,
    /// Allow parent MSP accounts to enable bypass their children's rules.
    #[builder(into)]
    #[serde(rename = "allowChildBypass")]
    pub r#allow_child_bypass: Option<bool>,
    /// Settings for auditing SSH usage.
    #[builder(into)]
    #[serde(rename = "auditSsh")]
    pub r#audit_ssh: Option<Box<super::types::TeamsRuleRuleSettingsAuditSsh>>,
    /// Configure how browser isolation behaves.
    #[builder(into)]
    #[serde(rename = "bisoAdminControls")]
    pub r#biso_admin_controls: Option<Box<super::types::TeamsRuleRuleSettingsBisoAdminControls>>,
    /// Indicator of block page enablement.
    #[builder(into)]
    #[serde(rename = "blockPageEnabled")]
    pub r#block_page_enabled: Option<bool>,
    /// The displayed reason for a user being blocked.
    #[builder(into)]
    #[serde(rename = "blockPageReason")]
    pub r#block_page_reason: Option<String>,
    /// Allow child MSP accounts to bypass their parent's rule.
    #[builder(into)]
    #[serde(rename = "bypassParentRule")]
    pub r#bypass_parent_rule: Option<bool>,
    /// Configure how session check behaves.
    #[builder(into)]
    #[serde(rename = "checkSession")]
    pub r#check_session: Option<Box<super::types::TeamsRuleRuleSettingsCheckSession>>,
    /// Add your own custom resolvers to route queries that match the resolver policy. Cannot be used when resolve*dns*through*cloudflare is set. DNS queries will route to the address closest to their origin.
    #[builder(into)]
    #[serde(rename = "dnsResolvers")]
    pub r#dns_resolvers: Option<Box<super::types::TeamsRuleRuleSettingsDnsResolvers>>,
    /// Configure how Proxy traffic egresses. Can be set for rules with Egress action and Egress filter. Can be omitted to indicate local egress via Warp IPs.
    #[builder(into)]
    #[serde(rename = "egress")]
    pub r#egress: Option<Box<super::types::TeamsRuleRuleSettingsEgress>>,
    /// Set to true, to ignore the category matches at CNAME domains in a response.
    #[builder(into)]
    #[serde(rename = "ignoreCnameCategoryMatches")]
    pub r#ignore_cname_category_matches: Option<bool>,
    /// Disable DNSSEC validation (must be Allow rule).
    #[builder(into)]
    #[serde(rename = "insecureDisableDnssecValidation")]
    pub r#insecure_disable_dnssec_validation: Option<bool>,
    /// Turns on IP category based filter on dns if the rule contains dns category checks.
    #[builder(into)]
    #[serde(rename = "ipCategories")]
    pub r#ip_categories: Option<bool>,
    /// Settings to forward layer 4 traffic.
    #[builder(into)]
    #[serde(rename = "l4override")]
    pub r#l_4_override: Option<Box<super::types::TeamsRuleRuleSettingsL4Override>>,
    /// Notification settings on a block rule.
    #[builder(into)]
    #[serde(rename = "notificationSettings")]
    pub r#notification_settings: Option<Box<super::types::TeamsRuleRuleSettingsNotificationSettings>>,
    /// The host to override matching DNS queries with.
    #[builder(into)]
    #[serde(rename = "overrideHost")]
    pub r#override_host: Option<String>,
    /// The IPs to override matching DNS queries with.
    #[builder(into)]
    #[serde(rename = "overrideIps")]
    pub r#override_ips: Option<Vec<String>>,
    /// Configure DLP Payload Logging settings for this rule.
    #[builder(into)]
    #[serde(rename = "payloadLog")]
    pub r#payload_log: Option<Box<super::types::TeamsRuleRuleSettingsPayloadLog>>,
    /// Enable sending queries that match the resolver policy to Cloudflare's default 1.1.1.1 DNS resolver. Cannot be set when `dns_resolvers` are specified.
    #[builder(into)]
    #[serde(rename = "resolveDnsThroughCloudflare")]
    pub r#resolve_dns_through_cloudflare: Option<bool>,
    /// Configure untrusted certificate settings for this rule.
    #[builder(into)]
    #[serde(rename = "untrustedCert")]
    pub r#untrusted_cert: Option<Box<super::types::TeamsRuleRuleSettingsUntrustedCert>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TeamsRuleRuleSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "add_headers",
                    &self.r#add_headers,
                ),
                to_pulumi_object_field(
                    "allow_child_bypass",
                    &self.r#allow_child_bypass,
                ),
                to_pulumi_object_field(
                    "audit_ssh",
                    &self.r#audit_ssh,
                ),
                to_pulumi_object_field(
                    "biso_admin_controls",
                    &self.r#biso_admin_controls,
                ),
                to_pulumi_object_field(
                    "block_page_enabled",
                    &self.r#block_page_enabled,
                ),
                to_pulumi_object_field(
                    "block_page_reason",
                    &self.r#block_page_reason,
                ),
                to_pulumi_object_field(
                    "bypass_parent_rule",
                    &self.r#bypass_parent_rule,
                ),
                to_pulumi_object_field(
                    "check_session",
                    &self.r#check_session,
                ),
                to_pulumi_object_field(
                    "dns_resolvers",
                    &self.r#dns_resolvers,
                ),
                to_pulumi_object_field(
                    "egress",
                    &self.r#egress,
                ),
                to_pulumi_object_field(
                    "ignore_cname_category_matches",
                    &self.r#ignore_cname_category_matches,
                ),
                to_pulumi_object_field(
                    "insecure_disable_dnssec_validation",
                    &self.r#insecure_disable_dnssec_validation,
                ),
                to_pulumi_object_field(
                    "ip_categories",
                    &self.r#ip_categories,
                ),
                to_pulumi_object_field(
                    "l_4_override",
                    &self.r#l_4_override,
                ),
                to_pulumi_object_field(
                    "notification_settings",
                    &self.r#notification_settings,
                ),
                to_pulumi_object_field(
                    "override_host",
                    &self.r#override_host,
                ),
                to_pulumi_object_field(
                    "override_ips",
                    &self.r#override_ips,
                ),
                to_pulumi_object_field(
                    "payload_log",
                    &self.r#payload_log,
                ),
                to_pulumi_object_field(
                    "resolve_dns_through_cloudflare",
                    &self.r#resolve_dns_through_cloudflare,
                ),
                to_pulumi_object_field(
                    "untrusted_cert",
                    &self.r#untrusted_cert,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TeamsRuleRuleSettings {
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
                    r#add_headers: {
                        let field_value = match fields_map.get("add_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'add_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_child_bypass: {
                        let field_value = match fields_map.get("allow_child_bypass") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_child_bypass' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audit_ssh: {
                        let field_value = match fields_map.get("audit_ssh") {
                            Some(value) => value,
                            None => bail!("Missing field 'audit_ssh' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#biso_admin_controls: {
                        let field_value = match fields_map.get("biso_admin_controls") {
                            Some(value) => value,
                            None => bail!("Missing field 'biso_admin_controls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#block_page_enabled: {
                        let field_value = match fields_map.get("block_page_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'block_page_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#block_page_reason: {
                        let field_value = match fields_map.get("block_page_reason") {
                            Some(value) => value,
                            None => bail!("Missing field 'block_page_reason' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bypass_parent_rule: {
                        let field_value = match fields_map.get("bypass_parent_rule") {
                            Some(value) => value,
                            None => bail!("Missing field 'bypass_parent_rule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#check_session: {
                        let field_value = match fields_map.get("check_session") {
                            Some(value) => value,
                            None => bail!("Missing field 'check_session' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_resolvers: {
                        let field_value = match fields_map.get("dns_resolvers") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_resolvers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#egress: {
                        let field_value = match fields_map.get("egress") {
                            Some(value) => value,
                            None => bail!("Missing field 'egress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ignore_cname_category_matches: {
                        let field_value = match fields_map.get("ignore_cname_category_matches") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignore_cname_category_matches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#insecure_disable_dnssec_validation: {
                        let field_value = match fields_map.get("insecure_disable_dnssec_validation") {
                            Some(value) => value,
                            None => bail!("Missing field 'insecure_disable_dnssec_validation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_categories: {
                        let field_value = match fields_map.get("ip_categories") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_categories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#l_4_override: {
                        let field_value = match fields_map.get("l_4_override") {
                            Some(value) => value,
                            None => bail!("Missing field 'l_4_override' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notification_settings: {
                        let field_value = match fields_map.get("notification_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'notification_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#override_host: {
                        let field_value = match fields_map.get("override_host") {
                            Some(value) => value,
                            None => bail!("Missing field 'override_host' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#override_ips: {
                        let field_value = match fields_map.get("override_ips") {
                            Some(value) => value,
                            None => bail!("Missing field 'override_ips' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#payload_log: {
                        let field_value = match fields_map.get("payload_log") {
                            Some(value) => value,
                            None => bail!("Missing field 'payload_log' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resolve_dns_through_cloudflare: {
                        let field_value = match fields_map.get("resolve_dns_through_cloudflare") {
                            Some(value) => value,
                            None => bail!("Missing field 'resolve_dns_through_cloudflare' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#untrusted_cert: {
                        let field_value = match fields_map.get("untrusted_cert") {
                            Some(value) => value,
                            None => bail!("Missing field 'untrusted_cert' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
