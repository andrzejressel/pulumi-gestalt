#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceEndpointConnectionsPrivateEndpointConnection {
    /// A message indicating if changes on the service provider require any updates or not.
    #[builder(into)]
    #[serde(rename = "actionRequired")]
    pub r#action_required: String,
    /// The resource id of the private link service connection between the private link service and the private link endpoint.
    #[builder(into)]
    #[serde(rename = "connectionId")]
    pub r#connection_id: String,
    /// The name of the connection between the private link service and the private link endpoint.
    #[builder(into)]
    #[serde(rename = "connectionName")]
    pub r#connection_name: String,
    /// The request for approval message or the reason for rejection message.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// The resource id of the private link endpoint.
    #[builder(into)]
    #[serde(rename = "privateEndpointId")]
    pub r#private_endpoint_id: String,
    /// The name of the private link endpoint.
    #[builder(into)]
    #[serde(rename = "privateEndpointName")]
    pub r#private_endpoint_name: String,
    /// Indicates the state of the connection between the private link service and the private link endpoint, possible values are `Pending`, `Approved` or `Rejected`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetServiceEndpointConnectionsPrivateEndpointConnection {
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
                "action_required".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#action_required,
                )
                .await,
            );
            map.insert(
                "connection_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connection_id,
                )
                .await,
            );
            map.insert(
                "connection_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connection_name,
                )
                .await,
            );
            map.insert(
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "private_endpoint_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_endpoint_id,
                )
                .await,
            );
            map.insert(
                "private_endpoint_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_endpoint_name,
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

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetServiceEndpointConnectionsPrivateEndpointConnection {
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
                    r#action_required: {
                        let field_value = match fields_map.get("action_required") {
                            Some(value) => value,
                            None => bail!("Missing field 'action_required' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connection_id: {
                        let field_value = match fields_map.get("connection_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connection_name: {
                        let field_value = match fields_map.get("connection_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_endpoint_id: {
                        let field_value = match fields_map.get("private_endpoint_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_endpoint_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_endpoint_name: {
                        let field_value = match fields_map.get("private_endpoint_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_endpoint_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
