#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstancePscAutoConnection {
    /// (Output)
    /// Output Only. Type of a PSC Connection.
    /// Possible values:
    /// CONNECTION_TYPE_DISCOVERY
    /// CONNECTION_TYPE_PRIMARY
    /// CONNECTION_TYPE_READER
    #[builder(into)]
    #[serde(rename = "connectionType")]
    pub r#connection_type: Option<String>,
    /// (Output)
    /// Output only. The URI of the consumer side forwarding rule.
    /// Format:
    /// projects/{project}/regions/{region}/forwardingRules/{forwarding_rule}
    #[builder(into)]
    #[serde(rename = "forwardingRule")]
    pub r#forwarding_rule: Option<String>,
    /// (Output)
    /// Output only. The IP allocated on the consumer network for the PSC forwarding rule.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Option<String>,
    /// (Output)
    /// Output only. The consumer network where the IP address resides, in the form of
    /// projects/{project_id}/global/networks/{network_id}.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// (Output)
    /// Output only. Ports of the exposed endpoint.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// (Output)
    /// Output only. The consumer project_id where the forwarding rule is created from.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: Option<String>,
    /// (Output)
    /// Output only. The PSC connection id of the forwarding rule connected to the
    /// service attachment.
    #[builder(into)]
    #[serde(rename = "pscConnectionId")]
    pub r#psc_connection_id: Option<String>,
    /// (Output)
    /// Output Only. The status of the PSC connection: whether a connection exists and ACTIVE or it no longer exists.
    /// Possible values:
    /// ACTIVE
    /// NOT_FOUND
    #[builder(into)]
    #[serde(rename = "pscConnectionStatus")]
    pub r#psc_connection_status: Option<String>,
    /// (Output)
    /// Output only. The service attachment which is the target of the PSC connection, in the form of projects/{project-id}/regions/{region}/serviceAttachments/{service-attachment-id}.
    #[builder(into)]
    #[serde(rename = "serviceAttachment")]
    pub r#service_attachment: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstancePscAutoConnection {
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
                "connection_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connection_type,
                )
                .await,
            );
            map.insert(
                "forwarding_rule".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#forwarding_rule,
                )
                .await,
            );
            map.insert(
                "ip_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_address,
                )
                .await,
            );
            map.insert(
                "network".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network,
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
                "project_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#project_id,
                )
                .await,
            );
            map.insert(
                "psc_connection_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#psc_connection_id,
                )
                .await,
            );
            map.insert(
                "psc_connection_status".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#psc_connection_status,
                )
                .await,
            );
            map.insert(
                "service_attachment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_attachment,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstancePscAutoConnection {
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
                    r#connection_type: {
                        let field_value = match fields_map.get("connection_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forwarding_rule: {
                        let field_value = match fields_map.get("forwarding_rule") {
                            Some(value) => value,
                            None => bail!("Missing field 'forwarding_rule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_address: {
                        let field_value = match fields_map.get("ip_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network: {
                        let field_value = match fields_map.get("network") {
                            Some(value) => value,
                            None => bail!("Missing field 'network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#project_id: {
                        let field_value = match fields_map.get("project_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'project_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#psc_connection_id: {
                        let field_value = match fields_map.get("psc_connection_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'psc_connection_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#psc_connection_status: {
                        let field_value = match fields_map.get("psc_connection_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'psc_connection_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_attachment: {
                        let field_value = match fields_map.get("service_attachment") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_attachment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
