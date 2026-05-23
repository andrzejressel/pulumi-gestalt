#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkConnectionMonitorTestConfigurationHttpConfiguration {
    /// The HTTP method for the HTTP request. Possible values are `Get` and `Post`. Defaults to `Get`.
    #[builder(into)]
    pub r#method: Option<String>,
    /// The path component of the URI. It only accepts the absolute path.
    #[builder(into)]
    pub r#path: Option<String>,
    /// The port for the HTTP connection.
    #[builder(into)]
    pub r#port: Option<i32>,
    /// Should HTTPS be preferred over HTTP in cases where the choice is not explicit? Defaults to `false`.
    #[builder(into)]
    pub r#prefer_https: Option<bool>,
    /// A `request_header` block as defined below.
    #[builder(into)]
    pub r#request_headers: Option<Vec<super::super::types::network::NetworkConnectionMonitorTestConfigurationHttpConfigurationRequestHeader>>,
    /// The HTTP status codes to consider successful. For instance, `2xx`, `301-304` and `418`.
    #[builder(into)]
    pub r#valid_status_code_ranges: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkConnectionMonitorTestConfigurationHttpConfiguration {
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
                    "method",
                    &self.r#method,
                ),
                to_pulumi_object_field(
                    "path",
                    &self.r#path,
                ),
                to_pulumi_object_field(
                    "port",
                    &self.r#port,
                ),
                to_pulumi_object_field(
                    "preferHttps",
                    &self.r#prefer_https,
                ),
                to_pulumi_object_field(
                    "requestHeaders",
                    &self.r#request_headers,
                ),
                to_pulumi_object_field(
                    "validStatusCodeRanges",
                    &self.r#valid_status_code_ranges,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkConnectionMonitorTestConfigurationHttpConfiguration {
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
                    r#method: {
                        let field_value = match fields_map.get("method") {
                            Some(value) => value,
                            None => bail!("Missing field 'method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#port: {
                        let field_value = match fields_map.get("port") {
                            Some(value) => value,
                            None => bail!("Missing field 'port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefer_https: {
                        let field_value = match fields_map.get("preferHttps") {
                            Some(value) => value,
                            None => bail!("Missing field 'preferHttps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_headers: {
                        let field_value = match fields_map.get("requestHeaders") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestHeaders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#valid_status_code_ranges: {
                        let field_value = match fields_map.get("validStatusCodeRanges") {
                            Some(value) => value,
                            None => bail!("Missing field 'validStatusCodeRanges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
