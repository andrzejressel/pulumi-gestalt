#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceConnectionPolicyPscConnection {
    /// The resource reference of the consumer address.
    #[builder(into)]
    #[serde(rename = "consumerAddress")]
    pub r#consumer_address: Option<String>,
    /// The resource reference of the PSC Forwarding Rule within the consumer VPC.
    #[builder(into)]
    #[serde(rename = "consumerForwardingRule")]
    pub r#consumer_forwarding_rule: Option<String>,
    /// The project where the PSC connection is created.
    #[builder(into)]
    #[serde(rename = "consumerTargetProject")]
    pub r#consumer_target_project: Option<String>,
    /// The most recent error during operating this connection.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "error")]
    pub r#error: Option<Box<super::super::types::networkconnectivity::ServiceConnectionPolicyPscConnectionError>>,
    /// The error info for the latest error during operating this connection.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "errorInfo")]
    pub r#error_info: Option<Box<super::super::types::networkconnectivity::ServiceConnectionPolicyPscConnectionErrorInfo>>,
    /// The error type indicates whether the error is consumer facing, producer
    /// facing or system internal.
    /// Possible values are: `CONNECTION_ERROR_TYPE_UNSPECIFIED`, `ERROR_INTERNAL`, `ERROR_CONSUMER_SIDE`, `ERROR_PRODUCER_SIDE`.
    #[builder(into)]
    #[serde(rename = "errorType")]
    pub r#error_type: Option<String>,
    /// The last Compute Engine operation to setup PSC connection.
    #[builder(into)]
    #[serde(rename = "gceOperation")]
    pub r#gce_operation: Option<String>,
    /// The PSC connection id of the PSC forwarding rule.
    #[builder(into)]
    #[serde(rename = "pscConnectionId")]
    pub r#psc_connection_id: Option<String>,
    /// The state of the PSC connection.
    /// Possible values are: `STATE_UNSPECIFIED`, `ACTIVE`, `CREATING`, `DELETING`, `FAILED`.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceConnectionPolicyPscConnection {
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
                    "consumer_address",
                    &self.r#consumer_address,
                ),
                to_pulumi_object_field(
                    "consumer_forwarding_rule",
                    &self.r#consumer_forwarding_rule,
                ),
                to_pulumi_object_field(
                    "consumer_target_project",
                    &self.r#consumer_target_project,
                ),
                to_pulumi_object_field(
                    "error",
                    &self.r#error,
                ),
                to_pulumi_object_field(
                    "error_info",
                    &self.r#error_info,
                ),
                to_pulumi_object_field(
                    "error_type",
                    &self.r#error_type,
                ),
                to_pulumi_object_field(
                    "gce_operation",
                    &self.r#gce_operation,
                ),
                to_pulumi_object_field(
                    "psc_connection_id",
                    &self.r#psc_connection_id,
                ),
                to_pulumi_object_field(
                    "state",
                    &self.r#state,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceConnectionPolicyPscConnection {
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
                    r#consumer_address: {
                        let field_value = match fields_map.get("consumer_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'consumer_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#consumer_forwarding_rule: {
                        let field_value = match fields_map.get("consumer_forwarding_rule") {
                            Some(value) => value,
                            None => bail!("Missing field 'consumer_forwarding_rule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#consumer_target_project: {
                        let field_value = match fields_map.get("consumer_target_project") {
                            Some(value) => value,
                            None => bail!("Missing field 'consumer_target_project' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#error: {
                        let field_value = match fields_map.get("error") {
                            Some(value) => value,
                            None => bail!("Missing field 'error' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#error_info: {
                        let field_value = match fields_map.get("error_info") {
                            Some(value) => value,
                            None => bail!("Missing field 'error_info' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#error_type: {
                        let field_value = match fields_map.get("error_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'error_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gce_operation: {
                        let field_value = match fields_map.get("gce_operation") {
                            Some(value) => value,
                            None => bail!("Missing field 'gce_operation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
