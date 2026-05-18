#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetConfigurationSetDeliveryOption {
    /// The maximum amount of time, in seconds, that Amazon SES API v2 will attempt delivery of email. If specified, the value must greater than or equal to 300 seconds (5 minutes) and less than or equal to 50400 seconds (840 minutes).
    #[builder(into)]
    #[serde(rename = "maxDeliverySeconds")]
    pub r#max_delivery_seconds: i32,
    /// The name of the dedicated IP pool to associate with the configuration set.
    #[builder(into)]
    #[serde(rename = "sendingPoolName")]
    pub r#sending_pool_name: String,
    /// Specifies whether messages that use the configuration set are required to use Transport Layer Security (TLS).
    #[builder(into)]
    #[serde(rename = "tlsPolicy")]
    pub r#tls_policy: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetConfigurationSetDeliveryOption {
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
                    "max_delivery_seconds",
                    &self.r#max_delivery_seconds,
                ),
                to_pulumi_object_field(
                    "sending_pool_name",
                    &self.r#sending_pool_name,
                ),
                to_pulumi_object_field(
                    "tls_policy",
                    &self.r#tls_policy,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetConfigurationSetDeliveryOption {
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
                    r#max_delivery_seconds: {
                        let field_value = match fields_map.get("max_delivery_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_delivery_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sending_pool_name: {
                        let field_value = match fields_map.get("sending_pool_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'sending_pool_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_policy: {
                        let field_value = match fields_map.get("tls_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
