#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct QueueHttpTargetUriOverride {
    /// Host override.
    /// When specified, replaces the host part of the task URL.
    /// For example, if the task URL is "https://www.google.com", and host value
    /// is set to "example.net", the overridden URI will be changed to "https://example.net".
    /// Host value cannot be an empty string (INVALID_ARGUMENT).
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Option<String>,
    /// URI path.
    /// When specified, replaces the existing path of the task URL.
    /// Setting the path value to an empty string clears the URI path segment.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pathOverride")]
    pub r#path_override: Option<Box<super::super::types::cloudtasks::QueueHttpTargetUriOverridePathOverride>>,
    /// Port override.
    /// When specified, replaces the port part of the task URI.
    /// For instance, for a URI http://www.google.com/foo and port=123, the overridden URI becomes http://www.google.com:123/foo.
    /// Note that the port value must be a positive integer.
    /// Setting the port to 0 (Zero) clears the URI port.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<String>,
    /// URI query.
    /// When specified, replaces the query part of the task URI. Setting the query value to an empty string clears the URI query segment.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "queryOverride")]
    pub r#query_override: Option<Box<super::super::types::cloudtasks::QueueHttpTargetUriOverrideQueryOverride>>,
    /// Scheme override.
    /// When specified, the task URI scheme is replaced by the provided value (HTTP or HTTPS).
    /// Possible values are: `HTTP`, `HTTPS`.
    #[builder(into)]
    #[serde(rename = "scheme")]
    pub r#scheme: Option<String>,
    /// URI Override Enforce Mode
    /// When specified, determines the Target UriOverride mode. If not specified, it defaults to ALWAYS.
    /// Possible values are: `ALWAYS`, `IF_NOT_EXISTS`.
    #[builder(into)]
    #[serde(rename = "uriOverrideEnforceMode")]
    pub r#uri_override_enforce_mode: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for QueueHttpTargetUriOverride {
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
                    "host",
                    &self.r#host,
                ),
                to_pulumi_object_field(
                    "path_override",
                    &self.r#path_override,
                ),
                to_pulumi_object_field(
                    "port",
                    &self.r#port,
                ),
                to_pulumi_object_field(
                    "query_override",
                    &self.r#query_override,
                ),
                to_pulumi_object_field(
                    "scheme",
                    &self.r#scheme,
                ),
                to_pulumi_object_field(
                    "uri_override_enforce_mode",
                    &self.r#uri_override_enforce_mode,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for QueueHttpTargetUriOverride {
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
                    r#host: {
                        let field_value = match fields_map.get("host") {
                            Some(value) => value,
                            None => bail!("Missing field 'host' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path_override: {
                        let field_value = match fields_map.get("path_override") {
                            Some(value) => value,
                            None => bail!("Missing field 'path_override' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#query_override: {
                        let field_value = match fields_map.get("query_override") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_override' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scheme: {
                        let field_value = match fields_map.get("scheme") {
                            Some(value) => value,
                            None => bail!("Missing field 'scheme' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#uri_override_enforce_mode: {
                        let field_value = match fields_map.get("uri_override_enforce_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'uri_override_enforce_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
