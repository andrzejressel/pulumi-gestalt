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
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "accepted_route_count",
                    &self.r#accepted_route_count,
                ),
                to_pulumi_object_field(
                    "certificate_arn",
                    &self.r#certificate_arn,
                ),
                to_pulumi_object_field(
                    "last_status_change",
                    &self.r#last_status_change,
                ),
                to_pulumi_object_field(
                    "outside_ip_address",
                    &self.r#outside_ip_address,
                ),
                to_pulumi_object_field(
                    "status",
                    &self.r#status,
                ),
                to_pulumi_object_field(
                    "status_message",
                    &self.r#status_message,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
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
