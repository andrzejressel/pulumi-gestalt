#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecuritySolutionRecommendationsEnabled {
    /// Is Principal Authentication enabled for the ACR repository? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "acrAuthentication")]
    pub r#acr_authentication: Option<bool>,
    /// Is Agent send underutilized messages enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "agentSendUnutilizedMsg")]
    pub r#agent_send_unutilized_msg: Option<bool>,
    /// Is Security related system configuration issues identified? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "baseline")]
    pub r#baseline: Option<bool>,
    /// Is IoT Edge Hub memory optimized? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "edgeHubMemOptimize")]
    pub r#edge_hub_mem_optimize: Option<bool>,
    /// Is logging configured for IoT Edge module? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "edgeLoggingOption")]
    pub r#edge_logging_option: Option<bool>,
    /// Is inconsistent module settings enabled for SecurityGroup? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "inconsistentModuleSettings")]
    pub r#inconsistent_module_settings: Option<bool>,
    /// is Azure IoT Security agent installed? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "installAgent")]
    pub r#install_agent: Option<bool>,
    /// Is Default IP filter policy denied? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "ipFilterDenyAll")]
    pub r#ip_filter_deny_all: Option<bool>,
    /// Is IP filter rule source allowable IP range too large? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "ipFilterPermissiveRule")]
    pub r#ip_filter_permissive_rule: Option<bool>,
    /// Is any ports open on the device? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "openPorts")]
    pub r#open_ports: Option<bool>,
    /// Does firewall policy exist which allow necessary communication to/from the device? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "permissiveFirewallPolicy")]
    pub r#permissive_firewall_policy: Option<bool>,
    /// Is only necessary addresses or ports are permitted in? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "permissiveInputFirewallRules")]
    pub r#permissive_input_firewall_rules: Option<bool>,
    /// Is only necessary addresses or ports are permitted out? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "permissiveOutputFirewallRules")]
    pub r#permissive_output_firewall_rules: Option<bool>,
    /// Is high level permissions are needed for the module? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "privilegedDockerOptions")]
    pub r#privileged_docker_options: Option<bool>,
    /// Is any credentials shared among devices? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "sharedCredentials")]
    pub r#shared_credentials: Option<bool>,
    /// Does TLS cipher suite need to be updated? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "vulnerableTlsCipherSuite")]
    pub r#vulnerable_tls_cipher_suite: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SecuritySolutionRecommendationsEnabled {
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
                "acr_authentication".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#acr_authentication,
                )
                .await,
            );
            map.insert(
                "agent_send_unutilized_msg".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#agent_send_unutilized_msg,
                )
                .await,
            );
            map.insert(
                "baseline".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#baseline,
                )
                .await,
            );
            map.insert(
                "edge_hub_mem_optimize".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#edge_hub_mem_optimize,
                )
                .await,
            );
            map.insert(
                "edge_logging_option".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#edge_logging_option,
                )
                .await,
            );
            map.insert(
                "inconsistent_module_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#inconsistent_module_settings,
                )
                .await,
            );
            map.insert(
                "install_agent".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#install_agent,
                )
                .await,
            );
            map.insert(
                "ip_filter_deny_all".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_filter_deny_all,
                )
                .await,
            );
            map.insert(
                "ip_filter_permissive_rule".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_filter_permissive_rule,
                )
                .await,
            );
            map.insert(
                "open_ports".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#open_ports,
                )
                .await,
            );
            map.insert(
                "permissive_firewall_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#permissive_firewall_policy,
                )
                .await,
            );
            map.insert(
                "permissive_input_firewall_rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#permissive_input_firewall_rules,
                )
                .await,
            );
            map.insert(
                "permissive_output_firewall_rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#permissive_output_firewall_rules,
                )
                .await,
            );
            map.insert(
                "privileged_docker_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#privileged_docker_options,
                )
                .await,
            );
            map.insert(
                "shared_credentials".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#shared_credentials,
                )
                .await,
            );
            map.insert(
                "vulnerable_tls_cipher_suite".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vulnerable_tls_cipher_suite,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SecuritySolutionRecommendationsEnabled {
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
                    r#acr_authentication: {
                        let field_value = match fields_map.get("acr_authentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'acr_authentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#agent_send_unutilized_msg: {
                        let field_value = match fields_map.get("agent_send_unutilized_msg") {
                            Some(value) => value,
                            None => bail!("Missing field 'agent_send_unutilized_msg' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#baseline: {
                        let field_value = match fields_map.get("baseline") {
                            Some(value) => value,
                            None => bail!("Missing field 'baseline' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#edge_hub_mem_optimize: {
                        let field_value = match fields_map.get("edge_hub_mem_optimize") {
                            Some(value) => value,
                            None => bail!("Missing field 'edge_hub_mem_optimize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#edge_logging_option: {
                        let field_value = match fields_map.get("edge_logging_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'edge_logging_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inconsistent_module_settings: {
                        let field_value = match fields_map.get("inconsistent_module_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'inconsistent_module_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#install_agent: {
                        let field_value = match fields_map.get("install_agent") {
                            Some(value) => value,
                            None => bail!("Missing field 'install_agent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_filter_deny_all: {
                        let field_value = match fields_map.get("ip_filter_deny_all") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_filter_deny_all' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_filter_permissive_rule: {
                        let field_value = match fields_map.get("ip_filter_permissive_rule") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_filter_permissive_rule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#open_ports: {
                        let field_value = match fields_map.get("open_ports") {
                            Some(value) => value,
                            None => bail!("Missing field 'open_ports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#permissive_firewall_policy: {
                        let field_value = match fields_map.get("permissive_firewall_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'permissive_firewall_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#permissive_input_firewall_rules: {
                        let field_value = match fields_map.get("permissive_input_firewall_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'permissive_input_firewall_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#permissive_output_firewall_rules: {
                        let field_value = match fields_map.get("permissive_output_firewall_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'permissive_output_firewall_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#privileged_docker_options: {
                        let field_value = match fields_map.get("privileged_docker_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'privileged_docker_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shared_credentials: {
                        let field_value = match fields_map.get("shared_credentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'shared_credentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vulnerable_tls_cipher_suite: {
                        let field_value = match fields_map.get("vulnerable_tls_cipher_suite") {
                            Some(value) => value,
                            None => bail!("Missing field 'vulnerable_tls_cipher_suite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
