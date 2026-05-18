#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointPrivateServiceConnection {
    /// Does the Private Endpoint require Manual Approval from the remote resource owner? Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** If you are trying to connect the Private Endpoint to a remote resource without having the correct RBAC permissions on the remote resource set this value to `true`.
    #[builder(into)]
    #[serde(rename = "isManualConnection")]
    pub r#is_manual_connection: bool,
    /// Specifies the Name of the Private Service Connection. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The Service Alias of the Private Link Enabled Remote Resource which this Private Endpoint should be connected to. One of `private_connection_resource_id` or `private_connection_resource_alias` must be specified. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "privateConnectionResourceAlias")]
    pub r#private_connection_resource_alias: Option<String>,
    /// The ID of the Private Link Enabled Remote Resource which this Private Endpoint should be connected to. One of `private_connection_resource_id` or `private_connection_resource_alias` must be specified. Changing this forces a new resource to be created. For a web app or function app slot, the parent web app should be used in this field instead of a reference to the slot itself.
    #[builder(into)]
    #[serde(rename = "privateConnectionResourceId")]
    pub r#private_connection_resource_id: Option<String>,
    /// (Required) The static IP address set by this configuration. It is recommended to use the private IP address exported in the `private_service_connection` block to obtain the address associated with the private endpoint.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Option<String>,
    /// A message passed to the owner of the remote resource when the private endpoint attempts to establish the connection to the remote resource. The provider allows a maximum request message length of `140` characters, however the request message maximum length is dependent on the service the private endpoint is connected to. Only valid if `is_manual_connection` is set to `true`.
    /// 
    /// > **NOTE:** When connected to an SQL resource the `request_message` maximum length is `128`.
    #[builder(into)]
    #[serde(rename = "requestMessage")]
    pub r#request_message: Option<String>,
    /// A list of subresource names which the Private Endpoint is able to connect to. `subresource_names` corresponds to `group_id`. Possible values are detailed in the product [documentation](https://docs.microsoft.com/azure/private-link/private-endpoint-overview#private-link-resource) in the `Subresources` column. Changing this forces a new resource to be created. 
    /// 
    /// > **NOTE:** Some resource types (such as Storage Account) only support 1 subresource per private endpoint.
    /// 
    /// > **NOTE:** For most Private Links one or more `subresource_names` will need to be specified, please see the linked documentation for details.
    #[builder(into)]
    #[serde(rename = "subresourceNames")]
    pub r#subresource_names: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointPrivateServiceConnection {
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
                    "is_manual_connection",
                    &self.r#is_manual_connection,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "private_connection_resource_alias",
                    &self.r#private_connection_resource_alias,
                ),
                to_pulumi_object_field(
                    "private_connection_resource_id",
                    &self.r#private_connection_resource_id,
                ),
                to_pulumi_object_field(
                    "private_ip_address",
                    &self.r#private_ip_address,
                ),
                to_pulumi_object_field(
                    "request_message",
                    &self.r#request_message,
                ),
                to_pulumi_object_field(
                    "subresource_names",
                    &self.r#subresource_names,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointPrivateServiceConnection {
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
                    r#is_manual_connection: {
                        let field_value = match fields_map.get("is_manual_connection") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_manual_connection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#private_connection_resource_alias: {
                        let field_value = match fields_map.get("private_connection_resource_alias") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_connection_resource_alias' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_connection_resource_id: {
                        let field_value = match fields_map.get("private_connection_resource_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_connection_resource_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_ip_address: {
                        let field_value = match fields_map.get("private_ip_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_ip_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_message: {
                        let field_value = match fields_map.get("request_message") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subresource_names: {
                        let field_value = match fields_map.get("subresource_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'subresource_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
