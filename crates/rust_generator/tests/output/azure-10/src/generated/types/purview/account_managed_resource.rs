#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountManagedResource {
    /// The ID of the managed event hub namespace.
    #[builder(into)]
    #[serde(rename = "eventHubNamespaceId")]
    pub r#event_hub_namespace_id: Option<String>,
    /// The ID of the managed resource group.
    #[builder(into)]
    #[serde(rename = "resourceGroupId")]
    pub r#resource_group_id: Option<String>,
    /// The ID of the managed storage account.
    #[builder(into)]
    #[serde(rename = "storageAccountId")]
    pub r#storage_account_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccountManagedResource {
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
                "event_hub_namespace_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#event_hub_namespace_id,
                )
                .await,
            );
            map.insert(
                "resource_group_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_group_id,
                )
                .await,
            );
            map.insert(
                "storage_account_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_account_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccountManagedResource {
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
                    r#event_hub_namespace_id: {
                        let field_value = match fields_map.get("event_hub_namespace_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'event_hub_namespace_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_group_id: {
                        let field_value = match fields_map.get("resource_group_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_group_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_id: {
                        let field_value = match fields_map.get("storage_account_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_account_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
