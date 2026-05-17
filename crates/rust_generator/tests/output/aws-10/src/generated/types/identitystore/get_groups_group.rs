#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetGroupsGroup {
    /// Description of the specified group.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// Group's display name.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: String,
    /// List of identifiers issued to this resource by an external identity provider.
    #[builder(into)]
    #[serde(rename = "externalIds")]
    pub r#external_ids: Vec<super::super::types::identitystore::GetGroupsGroupExternalId>,
    /// Identifier of the group in the Identity Store.
    #[builder(into)]
    #[serde(rename = "groupId")]
    pub r#group_id: String,
    /// Identity Store ID associated with the Single Sign-On (SSO) Instance.
    #[builder(into)]
    #[serde(rename = "identityStoreId")]
    pub r#identity_store_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetGroupsGroup {
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
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "display_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#display_name,
                )
                .await,
            );
            map.insert(
                "external_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#external_ids,
                )
                .await,
            );
            map.insert(
                "group_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#group_id,
                )
                .await,
            );
            map.insert(
                "identity_store_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#identity_store_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetGroupsGroup {
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
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#display_name: {
                        let field_value = match fields_map.get("display_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'display_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#external_ids: {
                        let field_value = match fields_map.get("external_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'external_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#group_id: {
                        let field_value = match fields_map.get("group_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'group_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#identity_store_id: {
                        let field_value = match fields_map.get("identity_store_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'identity_store_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
