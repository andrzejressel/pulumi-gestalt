#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct QueueHttpTarget {
    /// HTTP target headers.
    /// This map contains the header field names and values.
    /// Headers will be set when running the CreateTask and/or BufferTask.
    /// These headers represent a subset of the headers that will be configured for the task's HTTP request.
    /// Some HTTP request headers will be ignored or replaced.
    /// Headers which can have multiple values (according to RFC2616) can be specified using comma-separated values.
    /// The size of the headers must be less than 80KB. Queue-level headers to override headers of all the tasks in the queue.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "headerOverrides")]
    pub r#header_overrides: Option<Vec<super::super::types::cloudtasks::QueueHttpTargetHeaderOverride>>,
    /// The HTTP method to use for the request.
    /// When specified, it overrides HttpRequest for the task.
    /// Note that if the value is set to GET the body of the task will be ignored at execution time.
    /// Possible values are: `HTTP_METHOD_UNSPECIFIED`, `POST`, `GET`, `HEAD`, `PUT`, `DELETE`, `PATCH`, `OPTIONS`.
    #[builder(into)]
    #[serde(rename = "httpMethod")]
    pub r#http_method: Option<String>,
    /// If specified, an OAuth token is generated and attached as the Authorization header in the HTTP request.
    /// This type of authorization should generally be used only when calling Google APIs hosted on *.googleapis.com.
    /// Note that both the service account email and the scope MUST be specified when using the queue-level authorization override.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oauthToken")]
    pub r#oauth_token: Option<Box<super::super::types::cloudtasks::QueueHttpTargetOauthToken>>,
    /// If specified, an OIDC token is generated and attached as an Authorization header in the HTTP request.
    /// This type of authorization can be used for many scenarios, including calling Cloud Run, or endpoints where you intend to validate the token yourself.
    /// Note that both the service account email and the audience MUST be specified when using the queue-level authorization override.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oidcToken")]
    pub r#oidc_token: Option<Box<super::super::types::cloudtasks::QueueHttpTargetOidcToken>>,
    /// URI override.
    /// When specified, overrides the execution URI for all the tasks in the queue.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "uriOverride")]
    pub r#uri_override: Option<Box<super::super::types::cloudtasks::QueueHttpTargetUriOverride>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for QueueHttpTarget {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "header_overrides",
                    &self.r#header_overrides,
                ),
                to_pulumi_object_field(
                    "http_method",
                    &self.r#http_method,
                ),
                to_pulumi_object_field(
                    "oauth_token",
                    &self.r#oauth_token,
                ),
                to_pulumi_object_field(
                    "oidc_token",
                    &self.r#oidc_token,
                ),
                to_pulumi_object_field(
                    "uri_override",
                    &self.r#uri_override,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for QueueHttpTarget {
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
                    r#header_overrides: {
                        let field_value = match fields_map.get("header_overrides") {
                            Some(value) => value,
                            None => bail!("Missing field 'header_overrides' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_method: {
                        let field_value = match fields_map.get("http_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_token: {
                        let field_value = match fields_map.get("oauth_token") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth_token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oidc_token: {
                        let field_value = match fields_map.get("oidc_token") {
                            Some(value) => value,
                            None => bail!("Missing field 'oidc_token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#uri_override: {
                        let field_value = match fields_map.get("uri_override") {
                            Some(value) => value,
                            None => bail!("Missing field 'uri_override' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
