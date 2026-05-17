#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BrokerLdapServerMetadata {
    /// List of a fully qualified domain name of the LDAP server and an optional failover server.
    #[builder(into)]
    #[serde(rename = "hosts")]
    pub r#hosts: Option<Vec<String>>,
    /// Fully qualified name of the directory to search for a user’s groups.
    #[builder(into)]
    #[serde(rename = "roleBase")]
    pub r#role_base: Option<String>,
    /// Specifies the LDAP attribute that identifies the group name attribute in the object returned from the group membership query.
    #[builder(into)]
    #[serde(rename = "roleName")]
    pub r#role_name: Option<String>,
    /// Search criteria for groups.
    #[builder(into)]
    #[serde(rename = "roleSearchMatching")]
    pub r#role_search_matching: Option<String>,
    /// Whether the directory search scope is the entire sub-tree.
    #[builder(into)]
    #[serde(rename = "roleSearchSubtree")]
    pub r#role_search_subtree: Option<bool>,
    /// Service account password.
    #[builder(into)]
    #[serde(rename = "serviceAccountPassword")]
    pub r#service_account_password: Option<String>,
    /// Service account username.
    #[builder(into)]
    #[serde(rename = "serviceAccountUsername")]
    pub r#service_account_username: Option<String>,
    /// Fully qualified name of the directory where you want to search for users.
    #[builder(into)]
    #[serde(rename = "userBase")]
    pub r#user_base: Option<String>,
    /// Specifies the name of the LDAP attribute for the user group membership.
    #[builder(into)]
    #[serde(rename = "userRoleName")]
    pub r#user_role_name: Option<String>,
    /// Search criteria for users.
    #[builder(into)]
    #[serde(rename = "userSearchMatching")]
    pub r#user_search_matching: Option<String>,
    /// Whether the directory search scope is the entire sub-tree.
    #[builder(into)]
    #[serde(rename = "userSearchSubtree")]
    pub r#user_search_subtree: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BrokerLdapServerMetadata {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "hosts",
                    &self.r#hosts,
                ),
                to_pulumi_object_field(
                    "role_base",
                    &self.r#role_base,
                ),
                to_pulumi_object_field(
                    "role_name",
                    &self.r#role_name,
                ),
                to_pulumi_object_field(
                    "role_search_matching",
                    &self.r#role_search_matching,
                ),
                to_pulumi_object_field(
                    "role_search_subtree",
                    &self.r#role_search_subtree,
                ),
                to_pulumi_object_field(
                    "service_account_password",
                    &self.r#service_account_password,
                ),
                to_pulumi_object_field(
                    "service_account_username",
                    &self.r#service_account_username,
                ),
                to_pulumi_object_field(
                    "user_base",
                    &self.r#user_base,
                ),
                to_pulumi_object_field(
                    "user_role_name",
                    &self.r#user_role_name,
                ),
                to_pulumi_object_field(
                    "user_search_matching",
                    &self.r#user_search_matching,
                ),
                to_pulumi_object_field(
                    "user_search_subtree",
                    &self.r#user_search_subtree,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BrokerLdapServerMetadata {
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
