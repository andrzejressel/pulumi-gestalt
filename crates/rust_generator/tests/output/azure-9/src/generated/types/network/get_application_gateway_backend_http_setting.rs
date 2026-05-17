#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetApplicationGatewayBackendHttpSetting {
    /// The name of the affinity cookie.
    #[builder(into)]
    #[serde(rename = "affinityCookieName")]
    pub r#affinity_cookie_name: String,
    /// One or more `authentication_certificate` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "authenticationCertificates")]
    pub r#authentication_certificates: Vec<super::super::types::network::GetApplicationGatewayBackendHttpSettingAuthenticationCertificate>,
    /// A `connection_draining` block as defined below.
    #[builder(into)]
    #[serde(rename = "connectionDrainings")]
    pub r#connection_drainings: Vec<super::super::types::network::GetApplicationGatewayBackendHttpSettingConnectionDraining>,
    /// Is Cookie-Based Affinity enabled?
    #[builder(into)]
    #[serde(rename = "cookieBasedAffinity")]
    pub r#cookie_based_affinity: String,
    /// The Hostname which is used for this HTTP Listener.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: String,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The URL path to rewrite.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
    /// Whether host header will be picked from the host name of the backend server.
    #[builder(into)]
    #[serde(rename = "pickHostNameFromBackendAddress")]
    pub r#pick_host_name_from_backend_address: bool,
    /// Custom port which is used for probing the backend servers.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
    /// The ID of the associated Probe.
    #[builder(into)]
    #[serde(rename = "probeId")]
    pub r#probe_id: String,
    /// The name of the associated HTTP Probe.
    #[builder(into)]
    #[serde(rename = "probeName")]
    pub r#probe_name: String,
    /// The Protocol used for this Probe.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// The request timeout in seconds.
    #[builder(into)]
    #[serde(rename = "requestTimeout")]
    pub r#request_timeout: i32,
    /// A list of `trusted_root_certificate` names.
    #[builder(into)]
    #[serde(rename = "trustedRootCertificateNames")]
    pub r#trusted_root_certificate_names: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetApplicationGatewayBackendHttpSetting {
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
                "affinity_cookie_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#affinity_cookie_name,
                )
                .await,
            );
            map.insert(
                "authentication_certificates".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authentication_certificates,
                )
                .await,
            );
            map.insert(
                "connection_drainings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connection_drainings,
                )
                .await,
            );
            map.insert(
                "cookie_based_affinity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cookie_based_affinity,
                )
                .await,
            );
            map.insert(
                "host_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host_name,
                )
                .await,
            );
            map.insert(
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
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
                "path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#path,
                )
                .await,
            );
            map.insert(
                "pick_host_name_from_backend_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pick_host_name_from_backend_address,
                )
                .await,
            );
            map.insert(
                "port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#port,
                )
                .await,
            );
            map.insert(
                "probe_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#probe_id,
                )
                .await,
            );
            map.insert(
                "probe_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#probe_name,
                )
                .await,
            );
            map.insert(
                "protocol".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#protocol,
                )
                .await,
            );
            map.insert(
                "request_timeout".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_timeout,
                )
                .await,
            );
            map.insert(
                "trusted_root_certificate_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#trusted_root_certificate_names,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetApplicationGatewayBackendHttpSetting {
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
                    r#affinity_cookie_name: {
                        let field_value = match fields_map.get("affinity_cookie_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'affinity_cookie_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#authentication_certificates: {
                        let field_value = match fields_map.get("authentication_certificates") {
                            Some(value) => value,
                            None => bail!("Missing field 'authentication_certificates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connection_drainings: {
                        let field_value = match fields_map.get("connection_drainings") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_drainings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cookie_based_affinity: {
                        let field_value = match fields_map.get("cookie_based_affinity") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookie_based_affinity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_name: {
                        let field_value = match fields_map.get("host_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pick_host_name_from_backend_address: {
                        let field_value = match fields_map.get("pick_host_name_from_backend_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'pick_host_name_from_backend_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port: {
                        let field_value = match fields_map.get("port") {
                            Some(value) => value,
                            None => bail!("Missing field 'port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#probe_id: {
                        let field_value = match fields_map.get("probe_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'probe_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#probe_name: {
                        let field_value = match fields_map.get("probe_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'probe_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#request_timeout: {
                        let field_value = match fields_map.get("request_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trusted_root_certificate_names: {
                        let field_value = match fields_map.get("trusted_root_certificate_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'trusted_root_certificate_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
