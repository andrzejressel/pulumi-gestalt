#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SubscriptionPushConfig {
    /// Endpoint configuration attributes.
    /// Every endpoint has a set of API supported attributes that can
    /// be used to control different aspects of the message delivery.
    /// The currently supported attribute is x-goog-version, which you
    /// can use to change the format of the pushed message. This
    /// attribute indicates the version of the data expected by
    /// the endpoint. This controls the shape of the pushed message
    /// (i.e., its fields and metadata). The endpoint version is
    /// based on the version of the Pub/Sub API.
    /// If not present during the subscriptions.create call,
    /// it will default to the version of the API used to make
    /// such call. If not present during a subscriptions.modifyPushConfig
    /// call, its value will not be changed. subscriptions.get
    /// calls will always return a valid version, even if the
    /// subscription was created without this attribute.
    /// The possible values for this attribute are:
    /// - v1beta1: uses the push format defined in the v1beta1 Pub/Sub API.
    /// - v1 or v1beta2: uses the push format defined in the v1 Pub/Sub API.
    #[builder(into)]
    #[serde(rename = "attributes")]
    pub r#attributes: Option<std::collections::HashMap<String, String>>,
    /// When set, the payload to the push endpoint is not wrapped.Sets the
    /// `data` field as the HTTP body for delivery.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "noWrapper")]
    pub r#no_wrapper: Option<Box<super::super::types::pubsub::SubscriptionPushConfigNoWrapper>>,
    /// If specified, Pub/Sub will generate and attach an OIDC JWT token as
    /// an Authorization header in the HTTP request for every pushed message.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oidcToken")]
    pub r#oidc_token: Option<Box<super::super::types::pubsub::SubscriptionPushConfigOidcToken>>,
    /// A URL locating the endpoint to which messages should be pushed.
    /// For example, a Webhook endpoint might use
    /// "https://example.com/push".
    #[builder(into)]
    #[serde(rename = "pushEndpoint")]
    pub r#push_endpoint: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SubscriptionPushConfig {
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
                    "attributes",
                    &self.r#attributes,
                ),
                to_pulumi_object_field(
                    "no_wrapper",
                    &self.r#no_wrapper,
                ),
                to_pulumi_object_field(
                    "oidc_token",
                    &self.r#oidc_token,
                ),
                to_pulumi_object_field(
                    "push_endpoint",
                    &self.r#push_endpoint,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SubscriptionPushConfig {
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
                    r#attributes: {
                        let field_value = match fields_map.get("attributes") {
                            Some(value) => value,
                            None => bail!("Missing field 'attributes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#no_wrapper: {
                        let field_value = match fields_map.get("no_wrapper") {
                            Some(value) => value,
                            None => bail!("Missing field 'no_wrapper' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#push_endpoint: {
                        let field_value = match fields_map.get("push_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'push_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
