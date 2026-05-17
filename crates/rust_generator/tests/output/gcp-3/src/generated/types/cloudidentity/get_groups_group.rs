#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetGroupsGroup {
    /// Additional group keys associated with the Group
    #[builder(into)]
    #[serde(rename = "additionalGroupKeys")]
    pub r#additional_group_keys: Vec<super::super::types::cloudidentity::GetGroupsGroupAdditionalGroupKey>,
    /// The time when the Group was created.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: String,
    /// An extended description to help users determine the purpose of a Group.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// The display name of the Group.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: String,
    /// EntityKey of the Group.  Structure is documented below.
    #[builder(into)]
    #[serde(rename = "groupKeys")]
    pub r#group_keys: Vec<super::super::types::cloudidentity::GetGroupsGroupGroupKey>,
    /// The initial configuration options for creating a Group.
    /// 
    /// See the
    /// [API reference](https://cloud.google.com/identity/docs/reference/rest/v1beta1/groups/create#initialgroupconfig)
    /// for possible values. Default value: "EMPTY" Possible values: ["INITIAL_GROUP_CONFIG_UNSPECIFIED", "WITH_INITIAL_OWNER", "EMPTY"]
    #[builder(into)]
    #[serde(rename = "initialGroupConfig")]
    pub r#initial_group_config: String,
    /// The labels that apply to the Group.
    /// Contains 'cloudidentity.googleapis.com/groups.discussion_forum': '' if the Group is a Google Group or
    /// 'system/groups/external': '' if the Group is an external-identity-mapped group.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: std::collections::HashMap<String, String>,
    /// Resource name of the Group in the format: groups/{group_id}, where `group_id` is the unique ID assigned to the Group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The parent resource under which to list all Groups. Must be of the form identitysources/{identity_source_id} for external- identity-mapped groups or customers/{customer_id} for Google Groups.
    #[builder(into)]
    #[serde(rename = "parent")]
    pub r#parent: String,
    /// The time when the Group was last updated.
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetGroupsGroup {
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
                "additional_group_keys".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#additional_group_keys,
                )
                .await,
            );
            map.insert(
                "create_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#create_time,
                )
                .await,
            );
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
                "group_keys".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#group_keys,
                )
                .await,
            );
            map.insert(
                "initial_group_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#initial_group_config,
                )
                .await,
            );
            map.insert(
                "labels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#labels,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "parent".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#parent,
                )
                .await,
            );
            map.insert(
                "update_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#update_time,
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
                    r#additional_group_keys: {
                        let field_value = match fields_map.get("additional_group_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_group_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_time: {
                        let field_value = match fields_map.get("create_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'create_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
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
                    r#group_keys: {
                        let field_value = match fields_map.get("group_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'group_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#initial_group_config: {
                        let field_value = match fields_map.get("initial_group_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'initial_group_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parent: {
                        let field_value = match fields_map.get("parent") {
                            Some(value) => value,
                            None => bail!("Missing field 'parent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#update_time: {
                        let field_value = match fields_map.get("update_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'update_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
