#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpringCloudAppIngressSettings {
    /// Specifies how ingress should communicate with this app backend service. Allowed values are `GRPC` and `Default`. Defaults to `Default`.
    #[builder(into)]
    #[serde(rename = "backendProtocol")]
    pub r#backend_protocol: Option<String>,
    /// Specifies the ingress read time out in seconds. Defaults to `300`.
    #[builder(into)]
    #[serde(rename = "readTimeoutInSeconds")]
    pub r#read_timeout_in_seconds: Option<i32>,
    /// Specifies the ingress send time out in seconds. Defaults to `60`.
    #[builder(into)]
    #[serde(rename = "sendTimeoutInSeconds")]
    pub r#send_timeout_in_seconds: Option<i32>,
    /// Specifies the type of the affinity, set this to `Cookie` to enable session affinity. Allowed values are `Cookie` and `None`. Defaults to `None`.
    #[builder(into)]
    #[serde(rename = "sessionAffinity")]
    pub r#session_affinity: Option<String>,
    /// Specifies the time in seconds until the cookie expires.
    #[builder(into)]
    #[serde(rename = "sessionCookieMaxAge")]
    pub r#session_cookie_max_age: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpringCloudAppIngressSettings {
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
                    "backend_protocol",
                    &self.r#backend_protocol,
                ),
                to_pulumi_object_field(
                    "read_timeout_in_seconds",
                    &self.r#read_timeout_in_seconds,
                ),
                to_pulumi_object_field(
                    "send_timeout_in_seconds",
                    &self.r#send_timeout_in_seconds,
                ),
                to_pulumi_object_field(
                    "session_affinity",
                    &self.r#session_affinity,
                ),
                to_pulumi_object_field(
                    "session_cookie_max_age",
                    &self.r#session_cookie_max_age,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SpringCloudAppIngressSettings {
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
                    r#backend_protocol: {
                        let field_value = match fields_map.get("backend_protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'backend_protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#read_timeout_in_seconds: {
                        let field_value = match fields_map.get("read_timeout_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_timeout_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#send_timeout_in_seconds: {
                        let field_value = match fields_map.get("send_timeout_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'send_timeout_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_affinity: {
                        let field_value = match fields_map.get("session_affinity") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_affinity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_cookie_max_age: {
                        let field_value = match fields_map.get("session_cookie_max_age") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_cookie_max_age' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
