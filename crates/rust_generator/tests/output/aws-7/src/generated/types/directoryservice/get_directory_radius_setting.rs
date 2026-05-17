#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDirectoryRadiusSetting {
    /// The protocol specified for your RADIUS endpoints.
    #[builder(into)]
    #[serde(rename = "authenticationProtocol")]
    pub r#authentication_protocol: String,
    /// Display label.
    #[builder(into)]
    #[serde(rename = "displayLabel")]
    pub r#display_label: String,
    /// Port that your RADIUS server is using for communications.
    #[builder(into)]
    #[serde(rename = "radiusPort")]
    pub r#radius_port: i32,
    /// Maximum number of times that communication with the RADIUS server is attempted.
    #[builder(into)]
    #[serde(rename = "radiusRetries")]
    pub r#radius_retries: i32,
    /// Set of strings that contains the fully qualified domain name (FQDN) or IP addresses of the RADIUS server endpoints, or the FQDN or IP addresses of your RADIUS server load balancer.
    #[builder(into)]
    #[serde(rename = "radiusServers")]
    pub r#radius_servers: Vec<String>,
    /// Amount of time, in seconds, to wait for the RADIUS server to respond.
    #[builder(into)]
    #[serde(rename = "radiusTimeout")]
    pub r#radius_timeout: i32,
    /// Not currently used.
    #[builder(into)]
    #[serde(rename = "useSameUsername")]
    pub r#use_same_username: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDirectoryRadiusSetting {
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
                "authentication_protocol".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authentication_protocol,
                )
                .await,
            );
            map.insert(
                "display_label".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#display_label,
                )
                .await,
            );
            map.insert(
                "radius_port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#radius_port,
                )
                .await,
            );
            map.insert(
                "radius_retries".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#radius_retries,
                )
                .await,
            );
            map.insert(
                "radius_servers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#radius_servers,
                )
                .await,
            );
            map.insert(
                "radius_timeout".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#radius_timeout,
                )
                .await,
            );
            map.insert(
                "use_same_username".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_same_username,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDirectoryRadiusSetting {
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
                    r#authentication_protocol: {
                        let field_value = match fields_map.get("authentication_protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'authentication_protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#display_label: {
                        let field_value = match fields_map.get("display_label") {
                            Some(value) => value,
                            None => bail!("Missing field 'display_label' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#radius_port: {
                        let field_value = match fields_map.get("radius_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'radius_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#radius_retries: {
                        let field_value = match fields_map.get("radius_retries") {
                            Some(value) => value,
                            None => bail!("Missing field 'radius_retries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#radius_servers: {
                        let field_value = match fields_map.get("radius_servers") {
                            Some(value) => value,
                            None => bail!("Missing field 'radius_servers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#radius_timeout: {
                        let field_value = match fields_map.get("radius_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'radius_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_same_username: {
                        let field_value = match fields_map.get("use_same_username") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_same_username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
