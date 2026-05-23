#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetApplicationGatewayBackendHttpSetting {
    /// The name of the affinity cookie.
    #[builder(into)]
    pub r#affinity_cookie_name: String,
    /// One or more `authentication_certificate` blocks as defined below.
    #[builder(into)]
    pub r#authentication_certificates: Vec<super::super::types::network::GetApplicationGatewayBackendHttpSettingAuthenticationCertificate>,
    /// A `connection_draining` block as defined below.
    #[builder(into)]
    pub r#connection_drainings: Vec<super::super::types::network::GetApplicationGatewayBackendHttpSettingConnectionDraining>,
    /// Is Cookie-Based Affinity enabled?
    #[builder(into)]
    pub r#cookie_based_affinity: String,
    /// The Hostname which is used for this HTTP Listener.
    #[builder(into)]
    pub r#host_name: String,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    pub r#id: String,
    /// The name of this Application Gateway.
    #[builder(into)]
    pub r#name: String,
    /// The URL path to rewrite.
    #[builder(into)]
    pub r#path: String,
    /// Whether host header will be picked from the host name of the backend server.
    #[builder(into)]
    pub r#pick_host_name_from_backend_address: bool,
    /// Custom port which is used for probing the backend servers.
    #[builder(into)]
    pub r#port: i32,
    /// The ID of the associated Probe.
    #[builder(into)]
    pub r#probe_id: String,
    /// The name of the associated HTTP Probe.
    #[builder(into)]
    pub r#probe_name: String,
    /// The Protocol used for this Probe.
    #[builder(into)]
    pub r#protocol: String,
    /// The request timeout in seconds.
    #[builder(into)]
    pub r#request_timeout: i32,
    /// A list of `trusted_root_certificate` names.
    #[builder(into)]
    pub r#trusted_root_certificate_names: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetApplicationGatewayBackendHttpSetting {
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
                    "affinityCookieName",
                    &self.r#affinity_cookie_name,
                ),
                to_pulumi_object_field(
                    "authenticationCertificates",
                    &self.r#authentication_certificates,
                ),
                to_pulumi_object_field(
                    "connectionDrainings",
                    &self.r#connection_drainings,
                ),
                to_pulumi_object_field(
                    "cookieBasedAffinity",
                    &self.r#cookie_based_affinity,
                ),
                to_pulumi_object_field(
                    "hostName",
                    &self.r#host_name,
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
                    "path",
                    &self.r#path,
                ),
                to_pulumi_object_field(
                    "pickHostNameFromBackendAddress",
                    &self.r#pick_host_name_from_backend_address,
                ),
                to_pulumi_object_field(
                    "port",
                    &self.r#port,
                ),
                to_pulumi_object_field(
                    "probeId",
                    &self.r#probe_id,
                ),
                to_pulumi_object_field(
                    "probeName",
                    &self.r#probe_name,
                ),
                to_pulumi_object_field(
                    "protocol",
                    &self.r#protocol,
                ),
                to_pulumi_object_field(
                    "requestTimeout",
                    &self.r#request_timeout,
                ),
                to_pulumi_object_field(
                    "trustedRootCertificateNames",
                    &self.r#trusted_root_certificate_names,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("affinityCookieName") {
                            Some(value) => value,
                            None => bail!("Missing field 'affinityCookieName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#authentication_certificates: {
                        let field_value = match fields_map.get("authenticationCertificates") {
                            Some(value) => value,
                            None => bail!("Missing field 'authenticationCertificates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connection_drainings: {
                        let field_value = match fields_map.get("connectionDrainings") {
                            Some(value) => value,
                            None => bail!("Missing field 'connectionDrainings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cookie_based_affinity: {
                        let field_value = match fields_map.get("cookieBasedAffinity") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookieBasedAffinity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("pickHostNameFromBackendAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'pickHostNameFromBackendAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("probeId") {
                            Some(value) => value,
                            None => bail!("Missing field 'probeId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#probe_name: {
                        let field_value = match fields_map.get("probeName") {
                            Some(value) => value,
                            None => bail!("Missing field 'probeName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("requestTimeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestTimeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trusted_root_certificate_names: {
                        let field_value = match fields_map.get("trustedRootCertificateNames") {
                            Some(value) => value,
                            None => bail!("Missing field 'trustedRootCertificateNames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
