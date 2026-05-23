#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAgent {
    /// A `extensions_allow_list` block as defined below.
    #[builder(into)]
    pub r#extensions_allow_lists: Vec<super::super::types::arcmachine::GetAgentExtensionsAllowList>,
    /// A `extensions_block_list` block as defined below.
    #[builder(into)]
    pub r#extensions_block_lists: Vec<super::super::types::arcmachine::GetAgentExtensionsBlockList>,
    /// Specifies whether the extension service is enabled or disabled.
    #[builder(into)]
    pub r#extensions_enabled: bool,
    /// Specified whether the guest configuration service is enabled or disabled.
    #[builder(into)]
    pub r#guest_configuration_enabled: bool,
    /// Specifies the list of ports that the agent will be able to listen on.
    #[builder(into)]
    pub r#incoming_connections_ports: Vec<String>,
    /// List of service names which should not use the specified proxy server.
    #[builder(into)]
    pub r#proxy_bypasses: Vec<String>,
    /// Specifies the URL of the proxy to be used.
    #[builder(into)]
    pub r#proxy_url: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAgent {
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
                    "extensionsAllowLists",
                    &self.r#extensions_allow_lists,
                ),
                to_pulumi_object_field(
                    "extensionsBlockLists",
                    &self.r#extensions_block_lists,
                ),
                to_pulumi_object_field(
                    "extensionsEnabled",
                    &self.r#extensions_enabled,
                ),
                to_pulumi_object_field(
                    "guestConfigurationEnabled",
                    &self.r#guest_configuration_enabled,
                ),
                to_pulumi_object_field(
                    "incomingConnectionsPorts",
                    &self.r#incoming_connections_ports,
                ),
                to_pulumi_object_field(
                    "proxyBypasses",
                    &self.r#proxy_bypasses,
                ),
                to_pulumi_object_field(
                    "proxyUrl",
                    &self.r#proxy_url,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAgent {
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
                    r#extensions_allow_lists: {
                        let field_value = match fields_map.get("extensionsAllowLists") {
                            Some(value) => value,
                            None => bail!("Missing field 'extensionsAllowLists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#extensions_block_lists: {
                        let field_value = match fields_map.get("extensionsBlockLists") {
                            Some(value) => value,
                            None => bail!("Missing field 'extensionsBlockLists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#extensions_enabled: {
                        let field_value = match fields_map.get("extensionsEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'extensionsEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#guest_configuration_enabled: {
                        let field_value = match fields_map.get("guestConfigurationEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'guestConfigurationEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#incoming_connections_ports: {
                        let field_value = match fields_map.get("incomingConnectionsPorts") {
                            Some(value) => value,
                            None => bail!("Missing field 'incomingConnectionsPorts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#proxy_bypasses: {
                        let field_value = match fields_map.get("proxyBypasses") {
                            Some(value) => value,
                            None => bail!("Missing field 'proxyBypasses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#proxy_url: {
                        let field_value = match fields_map.get("proxyUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'proxyUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
