#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAutonomousDatabasesAutonomousDatabasePropertyConnectionStringProfile {
    /// The current consumer group being used by the connection. 
    ///  Possible values:
    ///  CONSUMER_GROUP_UNSPECIFIED
    /// HIGH
    /// MEDIUM
    /// LOW
    /// TP
    /// TPURGENT
    #[builder(into)]
    #[serde(rename = "consumerGroup")]
    pub r#consumer_group: String,
    /// The display name for the database connection.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: String,
    /// The host name format being currently used in connection string. 
    ///  Possible values:
    ///  HOST_FORMAT_UNSPECIFIED
    /// FQDN
    /// IP
    #[builder(into)]
    #[serde(rename = "hostFormat")]
    pub r#host_format: String,
    /// This field indicates if the connection string is regional and is only
    /// applicable for cross-region Data Guard.
    #[builder(into)]
    #[serde(rename = "isRegional")]
    pub r#is_regional: bool,
    /// The protocol being used by the connection. 
    ///  Possible values:
    ///  PROTOCOL_UNSPECIFIED
    /// TCP
    /// TCPS
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// The current session mode of the connection. 
    ///  Possible values:
    ///  SESSION_MODE_UNSPECIFIED
    /// DIRECT
    /// INDIRECT
    #[builder(into)]
    #[serde(rename = "sessionMode")]
    pub r#session_mode: String,
    /// The syntax of the connection string. 
    ///  Possible values:
    ///  SYNTAX_FORMAT_UNSPECIFIED
    /// LONG
    /// EZCONNECT
    /// EZCONNECTPLUS
    #[builder(into)]
    #[serde(rename = "syntaxFormat")]
    pub r#syntax_format: String,
    /// This field indicates the TLS authentication type of the connection. 
    ///  Possible values:
    ///  TLS_AUTHENTICATION_UNSPECIFIED
    /// SERVER
    /// MUTUAL
    #[builder(into)]
    #[serde(rename = "tlsAuthentication")]
    pub r#tls_authentication: String,
    /// The value of the connection string.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAutonomousDatabasesAutonomousDatabasePropertyConnectionStringProfile {
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
                    "consumer_group",
                    &self.r#consumer_group,
                ),
                to_pulumi_object_field(
                    "display_name",
                    &self.r#display_name,
                ),
                to_pulumi_object_field(
                    "host_format",
                    &self.r#host_format,
                ),
                to_pulumi_object_field(
                    "is_regional",
                    &self.r#is_regional,
                ),
                to_pulumi_object_field(
                    "protocol",
                    &self.r#protocol,
                ),
                to_pulumi_object_field(
                    "session_mode",
                    &self.r#session_mode,
                ),
                to_pulumi_object_field(
                    "syntax_format",
                    &self.r#syntax_format,
                ),
                to_pulumi_object_field(
                    "tls_authentication",
                    &self.r#tls_authentication,
                ),
                to_pulumi_object_field(
                    "value",
                    &self.r#value,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAutonomousDatabasesAutonomousDatabasePropertyConnectionStringProfile {
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
                    r#consumer_group: {
                        let field_value = match fields_map.get("consumer_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'consumer_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#display_name: {
                        let field_value = match fields_map.get("display_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'display_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_format: {
                        let field_value = match fields_map.get("host_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_regional: {
                        let field_value = match fields_map.get("is_regional") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_regional' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#session_mode: {
                        let field_value = match fields_map.get("session_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#syntax_format: {
                        let field_value = match fields_map.get("syntax_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'syntax_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_authentication: {
                        let field_value = match fields_map.get("tls_authentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_authentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#value: {
                        let field_value = match fields_map.get("value") {
                            Some(value) => value,
                            None => bail!("Missing field 'value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
