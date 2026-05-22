#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZeroTrustGatewayPolicyRuleSettings {
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
    pub r#audit_ssh: Option<Box<super::types::ZeroTrustGatewayPolicyRuleSettingsAuditSsh>>,
    /// Configure how browser isolation behaves.
    #[builder(into)]
    #[serde(rename = "bisoAdminControls")]
    pub r#biso_admin_controls: Option<Box<super::types::ZeroTrustGatewayPolicyRuleSettingsBisoAdminControls>>,
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
    pub r#check_session: Option<Box<super::types::ZeroTrustGatewayPolicyRuleSettingsCheckSession>>,
    /// Add your own custom resolvers to route queries that match the resolver policy. Cannot be used when resolve*dns*through*cloudflare is set. DNS queries will route to the address closest to their origin.
    #[builder(into)]
    #[serde(rename = "dnsResolvers")]
    pub r#dns_resolvers: Option<Box<super::types::ZeroTrustGatewayPolicyRuleSettingsDnsResolvers>>,
    /// Configure how Proxy traffic egresses. Can be set for rules with Egress action and Egress filter. Can be omitted to indicate local egress via Warp IPs.
    #[builder(into)]
    #[serde(rename = "egress")]
    pub r#egress: Option<Box<super::types::ZeroTrustGatewayPolicyRuleSettingsEgress>>,
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
    pub r#l_4_override: Option<Box<super::types::ZeroTrustGatewayPolicyRuleSettingsL4Override>>,
    /// Notification settings on a block rule.
    #[builder(into)]
    #[serde(rename = "notificationSettings")]
    pub r#notification_settings: Option<Box<super::types::ZeroTrustGatewayPolicyRuleSettingsNotificationSettings>>,
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
    pub r#payload_log: Option<Box<super::types::ZeroTrustGatewayPolicyRuleSettingsPayloadLog>>,
    /// Enable sending queries that match the resolver policy to Cloudflare's default 1.1.1.1 DNS resolver. Cannot be set when `dns_resolvers` are specified.
    #[builder(into)]
    #[serde(rename = "resolveDnsThroughCloudflare")]
    pub r#resolve_dns_through_cloudflare: Option<bool>,
    /// Configure untrusted certificate settings for this rule.
    #[builder(into)]
    #[serde(rename = "untrustedCert")]
    pub r#untrusted_cert: Option<Box<super::types::ZeroTrustGatewayPolicyRuleSettingsUntrustedCert>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ZeroTrustGatewayPolicyRuleSettings {
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
                    "addHeaders",
                    &self.r#add_headers,
                ),
                to_pulumi_object_field(
                    "allowChildBypass",
                    &self.r#allow_child_bypass,
                ),
                to_pulumi_object_field(
                    "auditSsh",
                    &self.r#audit_ssh,
                ),
                to_pulumi_object_field(
                    "bisoAdminControls",
                    &self.r#biso_admin_controls,
                ),
                to_pulumi_object_field(
                    "blockPageEnabled",
                    &self.r#block_page_enabled,
                ),
                to_pulumi_object_field(
                    "blockPageReason",
                    &self.r#block_page_reason,
                ),
                to_pulumi_object_field(
                    "bypassParentRule",
                    &self.r#bypass_parent_rule,
                ),
                to_pulumi_object_field(
                    "checkSession",
                    &self.r#check_session,
                ),
                to_pulumi_object_field(
                    "dnsResolvers",
                    &self.r#dns_resolvers,
                ),
                to_pulumi_object_field(
                    "egress",
                    &self.r#egress,
                ),
                to_pulumi_object_field(
                    "ignoreCnameCategoryMatches",
                    &self.r#ignore_cname_category_matches,
                ),
                to_pulumi_object_field(
                    "insecureDisableDnssecValidation",
                    &self.r#insecure_disable_dnssec_validation,
                ),
                to_pulumi_object_field(
                    "ipCategories",
                    &self.r#ip_categories,
                ),
                to_pulumi_object_field(
                    "l4override",
                    &self.r#l_4_override,
                ),
                to_pulumi_object_field(
                    "notificationSettings",
                    &self.r#notification_settings,
                ),
                to_pulumi_object_field(
                    "overrideHost",
                    &self.r#override_host,
                ),
                to_pulumi_object_field(
                    "overrideIps",
                    &self.r#override_ips,
                ),
                to_pulumi_object_field(
                    "payloadLog",
                    &self.r#payload_log,
                ),
                to_pulumi_object_field(
                    "resolveDnsThroughCloudflare",
                    &self.r#resolve_dns_through_cloudflare,
                ),
                to_pulumi_object_field(
                    "untrustedCert",
                    &self.r#untrusted_cert,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ZeroTrustGatewayPolicyRuleSettings {
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
                        let field_value = match fields_map.get("addHeaders") {
                            Some(value) => value,
                            None => bail!("Missing field 'addHeaders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_child_bypass: {
                        let field_value = match fields_map.get("allowChildBypass") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowChildBypass' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audit_ssh: {
                        let field_value = match fields_map.get("auditSsh") {
                            Some(value) => value,
                            None => bail!("Missing field 'auditSsh' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#biso_admin_controls: {
                        let field_value = match fields_map.get("bisoAdminControls") {
                            Some(value) => value,
                            None => bail!("Missing field 'bisoAdminControls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#block_page_enabled: {
                        let field_value = match fields_map.get("blockPageEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'blockPageEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#block_page_reason: {
                        let field_value = match fields_map.get("blockPageReason") {
                            Some(value) => value,
                            None => bail!("Missing field 'blockPageReason' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bypass_parent_rule: {
                        let field_value = match fields_map.get("bypassParentRule") {
                            Some(value) => value,
                            None => bail!("Missing field 'bypassParentRule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#check_session: {
                        let field_value = match fields_map.get("checkSession") {
                            Some(value) => value,
                            None => bail!("Missing field 'checkSession' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_resolvers: {
                        let field_value = match fields_map.get("dnsResolvers") {
                            Some(value) => value,
                            None => bail!("Missing field 'dnsResolvers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("ignoreCnameCategoryMatches") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignoreCnameCategoryMatches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#insecure_disable_dnssec_validation: {
                        let field_value = match fields_map.get("insecureDisableDnssecValidation") {
                            Some(value) => value,
                            None => bail!("Missing field 'insecureDisableDnssecValidation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_categories: {
                        let field_value = match fields_map.get("ipCategories") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipCategories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#l_4_override: {
                        let field_value = match fields_map.get("l4override") {
                            Some(value) => value,
                            None => bail!("Missing field 'l4override' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notification_settings: {
                        let field_value = match fields_map.get("notificationSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'notificationSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#override_host: {
                        let field_value = match fields_map.get("overrideHost") {
                            Some(value) => value,
                            None => bail!("Missing field 'overrideHost' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#override_ips: {
                        let field_value = match fields_map.get("overrideIps") {
                            Some(value) => value,
                            None => bail!("Missing field 'overrideIps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#payload_log: {
                        let field_value = match fields_map.get("payloadLog") {
                            Some(value) => value,
                            None => bail!("Missing field 'payloadLog' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resolve_dns_through_cloudflare: {
                        let field_value = match fields_map.get("resolveDnsThroughCloudflare") {
                            Some(value) => value,
                            None => bail!("Missing field 'resolveDnsThroughCloudflare' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#untrusted_cert: {
                        let field_value = match fields_map.get("untrustedCert") {
                            Some(value) => value,
                            None => bail!("Missing field 'untrustedCert' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
