#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointGlobalDeliveryRuleUrlRedirectAction {
    /// Specifies the fragment part of the URL. This value must not start with a `#`.
    #[builder(into)]
    #[serde(rename = "fragment")]
    pub r#fragment: Option<String>,
    /// Specifies the hostname part of the URL.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Option<String>,
    /// Specifies the path part of the URL. This value must begin with a `/`.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// Specifies the protocol part of the URL. Valid values are `MatchRequest`, `Http` and `Https`. Defaults to `MatchRequest`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
    /// Specifies the query string part of the URL. This value must not start with a `?` or `&` and must be in `<key>=<value>` format separated by `&`.
    #[builder(into)]
    #[serde(rename = "queryString")]
    pub r#query_string: Option<String>,
    /// Type of the redirect. Valid values are `Found`, `Moved`, `PermanentRedirect` and `TemporaryRedirect`.
    #[builder(into)]
    #[serde(rename = "redirectType")]
    pub r#redirect_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointGlobalDeliveryRuleUrlRedirectAction {
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
                "fragment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fragment,
                )
                .await,
            );
            map.insert(
                "hostname".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hostname,
                )
                .await,
            );
            map.insert(
                "path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#path,
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
                "query_string".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_string,
                )
                .await,
            );
            map.insert(
                "redirect_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#redirect_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointGlobalDeliveryRuleUrlRedirectAction {
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
                    r#fragment: {
                        let field_value = match fields_map.get("fragment") {
                            Some(value) => value,
                            None => bail!("Missing field 'fragment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hostname: {
                        let field_value = match fields_map.get("hostname") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostname' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#query_string: {
                        let field_value = match fields_map.get("query_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect_type: {
                        let field_value = match fields_map.get("redirect_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirect_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
