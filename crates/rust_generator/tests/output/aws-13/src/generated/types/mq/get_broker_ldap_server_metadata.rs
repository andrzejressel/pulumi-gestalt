#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBrokerLdapServerMetadata {
    #[builder(into)]
    #[serde(rename = "hosts")]
    pub r#hosts: Vec<String>,
    #[builder(into)]
    #[serde(rename = "roleBase")]
    pub r#role_base: String,
    #[builder(into)]
    #[serde(rename = "roleName")]
    pub r#role_name: String,
    #[builder(into)]
    #[serde(rename = "roleSearchMatching")]
    pub r#role_search_matching: String,
    #[builder(into)]
    #[serde(rename = "roleSearchSubtree")]
    pub r#role_search_subtree: bool,
    #[builder(into)]
    #[serde(rename = "serviceAccountPassword")]
    pub r#service_account_password: String,
    #[builder(into)]
    #[serde(rename = "serviceAccountUsername")]
    pub r#service_account_username: String,
    #[builder(into)]
    #[serde(rename = "userBase")]
    pub r#user_base: String,
    #[builder(into)]
    #[serde(rename = "userRoleName")]
    pub r#user_role_name: String,
    #[builder(into)]
    #[serde(rename = "userSearchMatching")]
    pub r#user_search_matching: String,
    #[builder(into)]
    #[serde(rename = "userSearchSubtree")]
    pub r#user_search_subtree: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetBrokerLdapServerMetadata {
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
                "hosts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hosts,
                )
                .await,
            );
            map.insert(
                "role_base".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#role_base,
                )
                .await,
            );
            map.insert(
                "role_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#role_name,
                )
                .await,
            );
            map.insert(
                "role_search_matching".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#role_search_matching,
                )
                .await,
            );
            map.insert(
                "role_search_subtree".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#role_search_subtree,
                )
                .await,
            );
            map.insert(
                "service_account_password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_account_password,
                )
                .await,
            );
            map.insert(
                "service_account_username".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_account_username,
                )
                .await,
            );
            map.insert(
                "user_base".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_base,
                )
                .await,
            );
            map.insert(
                "user_role_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_role_name,
                )
                .await,
            );
            map.insert(
                "user_search_matching".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_search_matching,
                )
                .await,
            );
            map.insert(
                "user_search_subtree".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_search_subtree,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetBrokerLdapServerMetadata {
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
                    r#hosts: {
                        let field_value = match fields_map.get("hosts") {
                            Some(value) => value,
                            None => bail!("Missing field 'hosts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role_base: {
                        let field_value = match fields_map.get("role_base") {
                            Some(value) => value,
                            None => bail!("Missing field 'role_base' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role_name: {
                        let field_value = match fields_map.get("role_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'role_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role_search_matching: {
                        let field_value = match fields_map.get("role_search_matching") {
                            Some(value) => value,
                            None => bail!("Missing field 'role_search_matching' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role_search_subtree: {
                        let field_value = match fields_map.get("role_search_subtree") {
                            Some(value) => value,
                            None => bail!("Missing field 'role_search_subtree' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_password: {
                        let field_value = match fields_map.get("service_account_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_username: {
                        let field_value = match fields_map.get("service_account_username") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account_username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_base: {
                        let field_value = match fields_map.get("user_base") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_base' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_role_name: {
                        let field_value = match fields_map.get("user_role_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_role_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_search_matching: {
                        let field_value = match fields_map.get("user_search_matching") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_search_matching' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_search_subtree: {
                        let field_value = match fields_map.get("user_search_subtree") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_search_subtree' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
