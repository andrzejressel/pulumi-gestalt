#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationGatewayHttpListener {
    /// One or more `custom_error_configuration` blocks as defined below.
    #[builder(into)]
    pub r#custom_error_configurations: Option<Vec<super::super::types::network::ApplicationGatewayHttpListenerCustomErrorConfiguration>>,
    /// The ID of the Web Application Firewall Policy which should be used for this HTTP Listener.
    #[builder(into)]
    pub r#firewall_policy_id: Option<String>,
    /// The ID of the associated Frontend Configuration.
    #[builder(into)]
    pub r#frontend_ip_configuration_id: Option<String>,
    /// The Name of the Frontend IP Configuration used for this HTTP Listener.
    #[builder(into)]
    pub r#frontend_ip_configuration_name: String,
    /// The ID of the associated Frontend Port.
    #[builder(into)]
    pub r#frontend_port_id: Option<String>,
    /// The Name of the Frontend Port use for this HTTP Listener.
    #[builder(into)]
    pub r#frontend_port_name: String,
    /// The Hostname which should be used for this HTTP Listener. Setting this value changes Listener Type to 'Multi site'.
    #[builder(into)]
    pub r#host_name: Option<String>,
    /// A list of Hostname(s) should be used for this HTTP Listener. It allows special wildcard characters.
    /// 
    /// > **NOTE** The `host_names` and `host_name` are mutually exclusive and cannot both be set.
    #[builder(into)]
    pub r#host_names: Option<Vec<String>>,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    pub r#id: Option<String>,
    /// The Name of the HTTP Listener.
    #[builder(into)]
    pub r#name: String,
    /// The Protocol to use for this HTTP Listener. Possible values are `Http` and `Https`.
    #[builder(into)]
    pub r#protocol: String,
    /// Should Server Name Indication be Required? Defaults to `false`.
    #[builder(into)]
    pub r#require_sni: Option<bool>,
    /// The ID of the associated SSL Certificate.
    #[builder(into)]
    pub r#ssl_certificate_id: Option<String>,
    /// The name of the associated SSL Certificate which should be used for this HTTP Listener.
    #[builder(into)]
    pub r#ssl_certificate_name: Option<String>,
    /// The ID of the associated SSL Profile.
    #[builder(into)]
    pub r#ssl_profile_id: Option<String>,
    /// The name of the associated SSL Profile which should be used for this HTTP Listener.
    #[builder(into)]
    pub r#ssl_profile_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationGatewayHttpListener {
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
                    "customErrorConfigurations",
                    &self.r#custom_error_configurations,
                ),
                to_pulumi_object_field(
                    "firewallPolicyId",
                    &self.r#firewall_policy_id,
                ),
                to_pulumi_object_field(
                    "frontendIpConfigurationId",
                    &self.r#frontend_ip_configuration_id,
                ),
                to_pulumi_object_field(
                    "frontendIpConfigurationName",
                    &self.r#frontend_ip_configuration_name,
                ),
                to_pulumi_object_field(
                    "frontendPortId",
                    &self.r#frontend_port_id,
                ),
                to_pulumi_object_field(
                    "frontendPortName",
                    &self.r#frontend_port_name,
                ),
                to_pulumi_object_field(
                    "hostName",
                    &self.r#host_name,
                ),
                to_pulumi_object_field(
                    "hostNames",
                    &self.r#host_names,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "protocol",
                    &self.r#protocol,
                ),
                to_pulumi_object_field(
                    "requireSni",
                    &self.r#require_sni,
                ),
                to_pulumi_object_field(
                    "sslCertificateId",
                    &self.r#ssl_certificate_id,
                ),
                to_pulumi_object_field(
                    "sslCertificateName",
                    &self.r#ssl_certificate_name,
                ),
                to_pulumi_object_field(
                    "sslProfileId",
                    &self.r#ssl_profile_id,
                ),
                to_pulumi_object_field(
                    "sslProfileName",
                    &self.r#ssl_profile_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApplicationGatewayHttpListener {
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
                    r#custom_error_configurations: {
                        let field_value = match fields_map.get("customErrorConfigurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'customErrorConfigurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#firewall_policy_id: {
                        let field_value = match fields_map.get("firewallPolicyId") {
                            Some(value) => value,
                            None => bail!("Missing field 'firewallPolicyId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#frontend_ip_configuration_id: {
                        let field_value = match fields_map.get("frontendIpConfigurationId") {
                            Some(value) => value,
                            None => bail!("Missing field 'frontendIpConfigurationId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#frontend_ip_configuration_name: {
                        let field_value = match fields_map.get("frontendIpConfigurationName") {
                            Some(value) => value,
                            None => bail!("Missing field 'frontendIpConfigurationName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#frontend_port_id: {
                        let field_value = match fields_map.get("frontendPortId") {
                            Some(value) => value,
                            None => bail!("Missing field 'frontendPortId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#frontend_port_name: {
                        let field_value = match fields_map.get("frontendPortName") {
                            Some(value) => value,
                            None => bail!("Missing field 'frontendPortName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_name: {
                        let field_value = match fields_map.get("hostName") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_names: {
                        let field_value = match fields_map.get("hostNames") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostNames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#protocol: {
                        let field_value = match fields_map.get("protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_sni: {
                        let field_value = match fields_map.get("requireSni") {
                            Some(value) => value,
                            None => bail!("Missing field 'requireSni' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_certificate_id: {
                        let field_value = match fields_map.get("sslCertificateId") {
                            Some(value) => value,
                            None => bail!("Missing field 'sslCertificateId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_certificate_name: {
                        let field_value = match fields_map.get("sslCertificateName") {
                            Some(value) => value,
                            None => bail!("Missing field 'sslCertificateName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_profile_id: {
                        let field_value = match fields_map.get("sslProfileId") {
                            Some(value) => value,
                            None => bail!("Missing field 'sslProfileId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_profile_name: {
                        let field_value = match fields_map.get("sslProfileName") {
                            Some(value) => value,
                            None => bail!("Missing field 'sslProfileName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
