#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkConnectionMonitorTestConfiguration {
    /// A `http_configuration` block as defined below.
    #[builder(into)]
    #[serde(rename = "httpConfiguration")]
    pub r#http_configuration: Option<Box<super::super::types::network::NetworkConnectionMonitorTestConfigurationHttpConfiguration>>,
    /// A `icmp_configuration` block as defined below.
    #[builder(into)]
    #[serde(rename = "icmpConfiguration")]
    pub r#icmp_configuration: Option<Box<super::super::types::network::NetworkConnectionMonitorTestConfigurationIcmpConfiguration>>,
    /// The name of test configuration for the Network Connection Monitor.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The preferred IP version which is used in the test evaluation. Possible values are `IPv4` and `IPv6`.
    #[builder(into)]
    #[serde(rename = "preferredIpVersion")]
    pub r#preferred_ip_version: Option<String>,
    /// The protocol used to evaluate tests. Possible values are `Tcp`, `Http` and `Icmp`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// A `success_threshold` block as defined below.
    #[builder(into)]
    #[serde(rename = "successThreshold")]
    pub r#success_threshold: Option<Box<super::super::types::network::NetworkConnectionMonitorTestConfigurationSuccessThreshold>>,
    /// A `tcp_configuration` block as defined below.
    #[builder(into)]
    #[serde(rename = "tcpConfiguration")]
    pub r#tcp_configuration: Option<Box<super::super::types::network::NetworkConnectionMonitorTestConfigurationTcpConfiguration>>,
    /// The time interval in seconds at which the test evaluation will happen. Defaults to `60`.
    #[builder(into)]
    #[serde(rename = "testFrequencyInSeconds")]
    pub r#test_frequency_in_seconds: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkConnectionMonitorTestConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "http_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_configuration,
                )
                .await,
            );
            map.insert(
                "icmp_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#icmp_configuration,
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
                "preferred_ip_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preferred_ip_version,
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
                "success_threshold".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#success_threshold,
                )
                .await,
            );
            map.insert(
                "tcp_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tcp_configuration,
                )
                .await,
            );
            map.insert(
                "test_frequency_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#test_frequency_in_seconds,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkConnectionMonitorTestConfiguration {
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
                    r#http_configuration: {
                        let field_value = match fields_map.get("http_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#icmp_configuration: {
                        let field_value = match fields_map.get("icmp_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'icmp_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#preferred_ip_version: {
                        let field_value = match fields_map.get("preferred_ip_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'preferred_ip_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#success_threshold: {
                        let field_value = match fields_map.get("success_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'success_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tcp_configuration: {
                        let field_value = match fields_map.get("tcp_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'tcp_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#test_frequency_in_seconds: {
                        let field_value = match fields_map.get("test_frequency_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'test_frequency_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
