#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VpnConnectionVgwTelemetry {
    /// The number of accepted routes.
    #[builder(into)]
    #[serde(rename = "acceptedRouteCount")]
    pub r#accepted_route_count: Option<i32>,
    /// The Amazon Resource Name (ARN) of the VPN tunnel endpoint certificate.
    #[builder(into)]
    #[serde(rename = "certificateArn")]
    pub r#certificate_arn: Option<String>,
    /// The date and time of the last change in status.
    #[builder(into)]
    #[serde(rename = "lastStatusChange")]
    pub r#last_status_change: Option<String>,
    /// The Internet-routable IP address of the virtual private gateway's outside interface.
    #[builder(into)]
    #[serde(rename = "outsideIpAddress")]
    pub r#outside_ip_address: Option<String>,
    /// The status of the VPN tunnel.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// If an error occurs, a description of the error.
    #[builder(into)]
    #[serde(rename = "statusMessage")]
    pub r#status_message: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VpnConnectionVgwTelemetry {
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
                "accepted_route_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#accepted_route_count,
                )
                .await,
            );
            map.insert(
                "certificate_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#certificate_arn,
                )
                .await,
            );
            map.insert(
                "last_status_change".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_status_change,
                )
                .await,
            );
            map.insert(
                "outside_ip_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#outside_ip_address,
                )
                .await,
            );
            map.insert(
                "status".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#status,
                )
                .await,
            );
            map.insert(
                "status_message".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#status_message,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VpnConnectionVgwTelemetry {
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
                    r#accepted_route_count: {
                        let field_value = match fields_map.get("accepted_route_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'accepted_route_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#certificate_arn: {
                        let field_value = match fields_map.get("certificate_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_status_change: {
                        let field_value = match fields_map.get("last_status_change") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_status_change' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#outside_ip_address: {
                        let field_value = match fields_map.get("outside_ip_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'outside_ip_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status_message: {
                        let field_value = match fields_map.get("status_message") {
                            Some(value) => value,
                            None => bail!("Missing field 'status_message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
