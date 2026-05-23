#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecuritySolutionRecommendationsEnabled {
    /// Is Principal Authentication enabled for the ACR repository? Defaults to `true`.
    #[builder(into)]
    pub r#acr_authentication: Option<bool>,
    /// Is Agent send underutilized messages enabled? Defaults to `true`.
    #[builder(into)]
    pub r#agent_send_unutilized_msg: Option<bool>,
    /// Is Security related system configuration issues identified? Defaults to `true`.
    #[builder(into)]
    pub r#baseline: Option<bool>,
    /// Is IoT Edge Hub memory optimized? Defaults to `true`.
    #[builder(into)]
    pub r#edge_hub_mem_optimize: Option<bool>,
    /// Is logging configured for IoT Edge module? Defaults to `true`.
    #[builder(into)]
    pub r#edge_logging_option: Option<bool>,
    /// Is inconsistent module settings enabled for SecurityGroup? Defaults to `true`.
    #[builder(into)]
    pub r#inconsistent_module_settings: Option<bool>,
    /// is Azure IoT Security agent installed? Defaults to `true`.
    #[builder(into)]
    pub r#install_agent: Option<bool>,
    /// Is Default IP filter policy denied? Defaults to `true`.
    #[builder(into)]
    pub r#ip_filter_deny_all: Option<bool>,
    /// Is IP filter rule source allowable IP range too large? Defaults to `true`.
    #[builder(into)]
    pub r#ip_filter_permissive_rule: Option<bool>,
    /// Is any ports open on the device? Defaults to `true`.
    #[builder(into)]
    pub r#open_ports: Option<bool>,
    /// Does firewall policy exist which allow necessary communication to/from the device? Defaults to `true`.
    #[builder(into)]
    pub r#permissive_firewall_policy: Option<bool>,
    /// Is only necessary addresses or ports are permitted in? Defaults to `true`.
    #[builder(into)]
    pub r#permissive_input_firewall_rules: Option<bool>,
    /// Is only necessary addresses or ports are permitted out? Defaults to `true`.
    #[builder(into)]
    pub r#permissive_output_firewall_rules: Option<bool>,
    /// Is high level permissions are needed for the module? Defaults to `true`.
    #[builder(into)]
    pub r#privileged_docker_options: Option<bool>,
    /// Is any credentials shared among devices? Defaults to `true`.
    #[builder(into)]
    pub r#shared_credentials: Option<bool>,
    /// Does TLS cipher suite need to be updated? Defaults to `true`.
    #[builder(into)]
    pub r#vulnerable_tls_cipher_suite: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SecuritySolutionRecommendationsEnabled {
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
                    "acrAuthentication",
                    &self.r#acr_authentication,
                ),
                to_pulumi_object_field(
                    "agentSendUnutilizedMsg",
                    &self.r#agent_send_unutilized_msg,
                ),
                to_pulumi_object_field(
                    "baseline",
                    &self.r#baseline,
                ),
                to_pulumi_object_field(
                    "edgeHubMemOptimize",
                    &self.r#edge_hub_mem_optimize,
                ),
                to_pulumi_object_field(
                    "edgeLoggingOption",
                    &self.r#edge_logging_option,
                ),
                to_pulumi_object_field(
                    "inconsistentModuleSettings",
                    &self.r#inconsistent_module_settings,
                ),
                to_pulumi_object_field(
                    "installAgent",
                    &self.r#install_agent,
                ),
                to_pulumi_object_field(
                    "ipFilterDenyAll",
                    &self.r#ip_filter_deny_all,
                ),
                to_pulumi_object_field(
                    "ipFilterPermissiveRule",
                    &self.r#ip_filter_permissive_rule,
                ),
                to_pulumi_object_field(
                    "openPorts",
                    &self.r#open_ports,
                ),
                to_pulumi_object_field(
                    "permissiveFirewallPolicy",
                    &self.r#permissive_firewall_policy,
                ),
                to_pulumi_object_field(
                    "permissiveInputFirewallRules",
                    &self.r#permissive_input_firewall_rules,
                ),
                to_pulumi_object_field(
                    "permissiveOutputFirewallRules",
                    &self.r#permissive_output_firewall_rules,
                ),
                to_pulumi_object_field(
                    "privilegedDockerOptions",
                    &self.r#privileged_docker_options,
                ),
                to_pulumi_object_field(
                    "sharedCredentials",
                    &self.r#shared_credentials,
                ),
                to_pulumi_object_field(
                    "vulnerableTlsCipherSuite",
                    &self.r#vulnerable_tls_cipher_suite,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("acrAuthentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'acrAuthentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#agent_send_unutilized_msg: {
                        let field_value = match fields_map.get("agentSendUnutilizedMsg") {
                            Some(value) => value,
                            None => bail!("Missing field 'agentSendUnutilizedMsg' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("edgeHubMemOptimize") {
                            Some(value) => value,
                            None => bail!("Missing field 'edgeHubMemOptimize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#edge_logging_option: {
                        let field_value = match fields_map.get("edgeLoggingOption") {
                            Some(value) => value,
                            None => bail!("Missing field 'edgeLoggingOption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inconsistent_module_settings: {
                        let field_value = match fields_map.get("inconsistentModuleSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'inconsistentModuleSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#install_agent: {
                        let field_value = match fields_map.get("installAgent") {
                            Some(value) => value,
                            None => bail!("Missing field 'installAgent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_filter_deny_all: {
                        let field_value = match fields_map.get("ipFilterDenyAll") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipFilterDenyAll' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_filter_permissive_rule: {
                        let field_value = match fields_map.get("ipFilterPermissiveRule") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipFilterPermissiveRule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#open_ports: {
                        let field_value = match fields_map.get("openPorts") {
                            Some(value) => value,
                            None => bail!("Missing field 'openPorts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#permissive_firewall_policy: {
                        let field_value = match fields_map.get("permissiveFirewallPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'permissiveFirewallPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#permissive_input_firewall_rules: {
                        let field_value = match fields_map.get("permissiveInputFirewallRules") {
                            Some(value) => value,
                            None => bail!("Missing field 'permissiveInputFirewallRules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#permissive_output_firewall_rules: {
                        let field_value = match fields_map.get("permissiveOutputFirewallRules") {
                            Some(value) => value,
                            None => bail!("Missing field 'permissiveOutputFirewallRules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#privileged_docker_options: {
                        let field_value = match fields_map.get("privilegedDockerOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'privilegedDockerOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shared_credentials: {
                        let field_value = match fields_map.get("sharedCredentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'sharedCredentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vulnerable_tls_cipher_suite: {
                        let field_value = match fields_map.get("vulnerableTlsCipherSuite") {
                            Some(value) => value,
                            None => bail!("Missing field 'vulnerableTlsCipherSuite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
