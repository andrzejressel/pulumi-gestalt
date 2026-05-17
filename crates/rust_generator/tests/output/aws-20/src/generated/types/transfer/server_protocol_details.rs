#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServerProtocolDetails {
    /// Indicates the transport method for the AS2 messages. Currently, only `HTTP` is supported.
    #[builder(into)]
    #[serde(rename = "as2Transports")]
    pub r#as_2_transports: Option<Vec<String>>,
    /// Indicates passive mode, for FTP and FTPS protocols. Enter a single IPv4 address, such as the public IP address of a firewall, router, or load balancer.
    #[builder(into)]
    #[serde(rename = "passiveIp")]
    pub r#passive_ip: Option<String>,
    /// Use to ignore the error that is generated when the client attempts to use `SETSTAT` on a file you are uploading to an S3 bucket. Valid values: `DEFAULT`, `ENABLE_NO_OP`.
    #[builder(into)]
    #[serde(rename = "setStatOption")]
    pub r#set_stat_option: Option<String>,
    /// A property used with Transfer Family servers that use the FTPS protocol. Provides a mechanism to resume or share a negotiated secret key between the control and data connection for an FTPS session. Valid values: `DISABLED`, `ENABLED`, `ENFORCED`.
    #[builder(into)]
    #[serde(rename = "tlsSessionResumptionMode")]
    pub r#tls_session_resumption_mode: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServerProtocolDetails {
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
                    "as_2_transports",
                    &self.r#as_2_transports,
                ),
                to_pulumi_object_field(
                    "passive_ip",
                    &self.r#passive_ip,
                ),
                to_pulumi_object_field(
                    "set_stat_option",
                    &self.r#set_stat_option,
                ),
                to_pulumi_object_field(
                    "tls_session_resumption_mode",
                    &self.r#tls_session_resumption_mode,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServerProtocolDetails {
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
                    r#as_2_transports: {
                        let field_value = match fields_map.get("as_2_transports") {
                            Some(value) => value,
                            None => bail!("Missing field 'as_2_transports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#passive_ip: {
                        let field_value = match fields_map.get("passive_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'passive_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#set_stat_option: {
                        let field_value = match fields_map.get("set_stat_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'set_stat_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_session_resumption_mode: {
                        let field_value = match fields_map.get("tls_session_resumption_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_session_resumption_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
