#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorRuleActionsUrlRedirectAction {
    /// The fragment to use in the redirect. The value must be a string between `0` and `1024` characters in length, leave blank to preserve the incoming fragment. Defaults to `""`.
    #[builder(into)]
    pub r#destination_fragment: Option<String>,
    /// The host name you want the request to be redirected to. The value must be a string between `0` and `2048` characters in length, leave blank to preserve the incoming host.
    #[builder(into)]
    pub r#destination_hostname: String,
    /// The path to use in the redirect. The value must be a string and include the leading `/`, leave blank to preserve the incoming path. Defaults to `""`.
    #[builder(into)]
    pub r#destination_path: Option<String>,
    /// The query string used in the redirect URL. The value must be in the &lt;key>=&lt;value> or &lt;key>={`action_server_variable`} format and must not include the leading `?`, leave blank to preserve the incoming query string. Maximum allowed length for this field is `2048` characters. Defaults to `""`.
    #[builder(into)]
    pub r#query_string: Option<String>,
    /// The protocol the request will be redirected as. Possible values include `MatchRequest`, `Http` or `Https`. Defaults to `MatchRequest`.
    #[builder(into)]
    pub r#redirect_protocol: Option<String>,
    /// The response type to return to the requestor. Possible values include `Moved`, `Found` , `TemporaryRedirect` or `PermanentRedirect`.
    #[builder(into)]
    pub r#redirect_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FrontdoorRuleActionsUrlRedirectAction {
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
                    "destinationFragment",
                    &self.r#destination_fragment,
                ),
                to_pulumi_object_field(
                    "destinationHostname",
                    &self.r#destination_hostname,
                ),
                to_pulumi_object_field(
                    "destinationPath",
                    &self.r#destination_path,
                ),
                to_pulumi_object_field(
                    "queryString",
                    &self.r#query_string,
                ),
                to_pulumi_object_field(
                    "redirectProtocol",
                    &self.r#redirect_protocol,
                ),
                to_pulumi_object_field(
                    "redirectType",
                    &self.r#redirect_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FrontdoorRuleActionsUrlRedirectAction {
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
                    r#destination_fragment: {
                        let field_value = match fields_map.get("destinationFragment") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationFragment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_hostname: {
                        let field_value = match fields_map.get("destinationHostname") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationHostname' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_path: {
                        let field_value = match fields_map.get("destinationPath") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationPath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_string: {
                        let field_value = match fields_map.get("queryString") {
                            Some(value) => value,
                            None => bail!("Missing field 'queryString' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect_protocol: {
                        let field_value = match fields_map.get("redirectProtocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirectProtocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect_type: {
                        let field_value = match fields_map.get("redirectType") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirectType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
