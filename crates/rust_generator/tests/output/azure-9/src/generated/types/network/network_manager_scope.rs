#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkManagerScope {
    /// A list of management group IDs.
    /// 
    /// > **NOTE:** When specifying a scope at the management group level, you need to register the `Microsoft.Network` at the management group scope before deploying a Network Manager, more information can be found in the [Azure document](https://learn.microsoft.com/en-us/azure/virtual-network-manager/concept-network-manager-scope#scope).
    #[builder(into)]
    #[serde(rename = "managementGroupIds")]
    pub r#management_group_ids: Option<Vec<String>>,
    /// A list of subscription IDs.
    #[builder(into)]
    #[serde(rename = "subscriptionIds")]
    pub r#subscription_ids: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkManagerScope {
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
                "management_group_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#management_group_ids,
                )
                .await,
            );
            map.insert(
                "subscription_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subscription_ids,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkManagerScope {
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
                    r#management_group_ids: {
                        let field_value = match fields_map.get("management_group_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'management_group_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subscription_ids: {
                        let field_value = match fields_map.get("subscription_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'subscription_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
