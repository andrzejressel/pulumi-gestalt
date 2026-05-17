#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SubscriptionClientScopedSubscription {
    /// Specifies the Client ID of the application that created the client-scoped subscription. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** Client ID can be null or empty, but it must match the client ID set on the JMS client application. From the Azure Service Bus perspective, a null client ID and an empty client id have the same behavior. If the client ID is set to null or empty, it is only accessible to client applications whose client ID is also set to null or empty.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    /// Whether the client scoped subscription is durable. This property can only be controlled from the application side.
    #[builder(into)]
    #[serde(rename = "isClientScopedSubscriptionDurable")]
    pub r#is_client_scoped_subscription_durable: Option<bool>,
    /// Whether the client scoped subscription is shareable. Defaults to `true` Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "isClientScopedSubscriptionShareable")]
    pub r#is_client_scoped_subscription_shareable: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SubscriptionClientScopedSubscription {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "client_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_id,
                )
                .await,
            );
            map.insert(
                "is_client_scoped_subscription_durable".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_client_scoped_subscription_durable,
                )
                .await,
            );
            map.insert(
                "is_client_scoped_subscription_shareable".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_client_scoped_subscription_shareable,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SubscriptionClientScopedSubscription {
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
                    r#client_id: {
                        let field_value = match fields_map.get("client_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_client_scoped_subscription_durable: {
                        let field_value = match fields_map.get("is_client_scoped_subscription_durable") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_client_scoped_subscription_durable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_client_scoped_subscription_shareable: {
                        let field_value = match fields_map.get("is_client_scoped_subscription_shareable") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_client_scoped_subscription_shareable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
