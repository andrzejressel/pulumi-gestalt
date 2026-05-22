#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSubscriptionsSubscription {
    /// The subscription display name.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: String,
    /// The ID of this subscription.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The subscription location placement ID.
    #[builder(into)]
    #[serde(rename = "locationPlacementId")]
    pub r#location_placement_id: String,
    /// The subscription quota ID.
    #[builder(into)]
    #[serde(rename = "quotaId")]
    pub r#quota_id: String,
    /// The subscription spending limit.
    #[builder(into)]
    #[serde(rename = "spendingLimit")]
    pub r#spending_limit: String,
    /// The subscription state. Possible values are Enabled, Warned, PastDue, Disabled, and Deleted.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: String,
    /// The subscription GUID.
    #[builder(into)]
    #[serde(rename = "subscriptionId")]
    pub r#subscription_id: String,
    /// A mapping of tags assigned to the resource.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: std::collections::HashMap<String, String>,
    /// The subscription tenant ID.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSubscriptionsSubscription {
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
                    "displayName",
                    &self.r#display_name,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "locationPlacementId",
                    &self.r#location_placement_id,
                ),
                to_pulumi_object_field(
                    "quotaId",
                    &self.r#quota_id,
                ),
                to_pulumi_object_field(
                    "spendingLimit",
                    &self.r#spending_limit,
                ),
                to_pulumi_object_field(
                    "state",
                    &self.r#state,
                ),
                to_pulumi_object_field(
                    "subscriptionId",
                    &self.r#subscription_id,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
                to_pulumi_object_field(
                    "tenantId",
                    &self.r#tenant_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSubscriptionsSubscription {
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
                    r#display_name: {
                        let field_value = match fields_map.get("displayName") {
                            Some(value) => value,
                            None => bail!("Missing field 'displayName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#location_placement_id: {
                        let field_value = match fields_map.get("locationPlacementId") {
                            Some(value) => value,
                            None => bail!("Missing field 'locationPlacementId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#quota_id: {
                        let field_value = match fields_map.get("quotaId") {
                            Some(value) => value,
                            None => bail!("Missing field 'quotaId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spending_limit: {
                        let field_value = match fields_map.get("spendingLimit") {
                            Some(value) => value,
                            None => bail!("Missing field 'spendingLimit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subscription_id: {
                        let field_value = match fields_map.get("subscriptionId") {
                            Some(value) => value,
                            None => bail!("Missing field 'subscriptionId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tenant_id: {
                        let field_value = match fields_map.get("tenantId") {
                            Some(value) => value,
                            None => bail!("Missing field 'tenantId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
