#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
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
    pub r#connection_type: Option<String>,
    /// (Output)
    /// Output only. The URI of the consumer side forwarding rule.
    /// Format:
    /// projects/{project}/regions/{region}/forwardingRules/{forwarding_rule}
    #[builder(into)]
    pub r#forwarding_rule: Option<String>,
    /// (Output)
    /// Output only. The IP allocated on the consumer network for the PSC forwarding rule.
    #[builder(into)]
    pub r#ip_address: Option<String>,
    /// (Output)
    /// Output only. The consumer network where the IP address resides, in the form of
    /// projects/{project_id}/global/networks/{network_id}.
    #[builder(into)]
    pub r#network: Option<String>,
    /// (Output)
    /// Output only. Ports of the exposed endpoint.
    #[builder(into)]
    pub r#port: Option<i32>,
    /// (Output)
    /// Output only. The consumer project_id where the forwarding rule is created from.
    #[builder(into)]
    pub r#project_id: Option<String>,
    /// (Output)
    /// Output only. The PSC connection id of the forwarding rule connected to the
    /// service attachment.
    #[builder(into)]
    pub r#psc_connection_id: Option<String>,
    /// (Output)
    /// Output Only. The status of the PSC connection: whether a connection exists and ACTIVE or it no longer exists.
    /// Possible values:
    /// ACTIVE
    /// NOT_FOUND
    #[builder(into)]
    pub r#psc_connection_status: Option<String>,
    /// (Output)
    /// Output only. The service attachment which is the target of the PSC connection, in the form of projects/{project-id}/regions/{region}/serviceAttachments/{service-attachment-id}.
    #[builder(into)]
    pub r#service_attachment: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstancePscAutoConnection {
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
                    "connectionType",
                    &self.r#connection_type,
                ),
                to_pulumi_object_field(
                    "forwardingRule",
                    &self.r#forwarding_rule,
                ),
                to_pulumi_object_field(
                    "ipAddress",
                    &self.r#ip_address,
                ),
                to_pulumi_object_field(
                    "network",
                    &self.r#network,
                ),
                to_pulumi_object_field(
                    "port",
                    &self.r#port,
                ),
                to_pulumi_object_field(
                    "projectId",
                    &self.r#project_id,
                ),
                to_pulumi_object_field(
                    "pscConnectionId",
                    &self.r#psc_connection_id,
                ),
                to_pulumi_object_field(
                    "pscConnectionStatus",
                    &self.r#psc_connection_status,
                ),
                to_pulumi_object_field(
                    "serviceAttachment",
                    &self.r#service_attachment,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("connectionType") {
                            Some(value) => value,
                            None => bail!("Missing field 'connectionType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forwarding_rule: {
                        let field_value = match fields_map.get("forwardingRule") {
                            Some(value) => value,
                            None => bail!("Missing field 'forwardingRule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_address: {
                        let field_value = match fields_map.get("ipAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("projectId") {
                            Some(value) => value,
                            None => bail!("Missing field 'projectId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#psc_connection_id: {
                        let field_value = match fields_map.get("pscConnectionId") {
                            Some(value) => value,
                            None => bail!("Missing field 'pscConnectionId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#psc_connection_status: {
                        let field_value = match fields_map.get("pscConnectionStatus") {
                            Some(value) => value,
                            None => bail!("Missing field 'pscConnectionStatus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_attachment: {
                        let field_value = match fields_map.get("serviceAttachment") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceAttachment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
