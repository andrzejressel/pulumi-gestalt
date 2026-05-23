#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBrokerLdapServerMetadata {
    #[builder(into)]
    pub r#hosts: Vec<String>,
    #[builder(into)]
    pub r#role_base: String,
    #[builder(into)]
    pub r#role_name: String,
    #[builder(into)]
    pub r#role_search_matching: String,
    #[builder(into)]
    pub r#role_search_subtree: bool,
    #[builder(into)]
    pub r#service_account_password: String,
    #[builder(into)]
    pub r#service_account_username: String,
    #[builder(into)]
    pub r#user_base: String,
    #[builder(into)]
    pub r#user_role_name: String,
    #[builder(into)]
    pub r#user_search_matching: String,
    #[builder(into)]
    pub r#user_search_subtree: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetBrokerLdapServerMetadata {
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
                    "hosts",
                    &self.r#hosts,
                ),
                to_pulumi_object_field(
                    "roleBase",
                    &self.r#role_base,
                ),
                to_pulumi_object_field(
                    "roleName",
                    &self.r#role_name,
                ),
                to_pulumi_object_field(
                    "roleSearchMatching",
                    &self.r#role_search_matching,
                ),
                to_pulumi_object_field(
                    "roleSearchSubtree",
                    &self.r#role_search_subtree,
                ),
                to_pulumi_object_field(
                    "serviceAccountPassword",
                    &self.r#service_account_password,
                ),
                to_pulumi_object_field(
                    "serviceAccountUsername",
                    &self.r#service_account_username,
                ),
                to_pulumi_object_field(
                    "userBase",
                    &self.r#user_base,
                ),
                to_pulumi_object_field(
                    "userRoleName",
                    &self.r#user_role_name,
                ),
                to_pulumi_object_field(
                    "userSearchMatching",
                    &self.r#user_search_matching,
                ),
                to_pulumi_object_field(
                    "userSearchSubtree",
                    &self.r#user_search_subtree,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("roleBase") {
                            Some(value) => value,
                            None => bail!("Missing field 'roleBase' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role_name: {
                        let field_value = match fields_map.get("roleName") {
                            Some(value) => value,
                            None => bail!("Missing field 'roleName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role_search_matching: {
                        let field_value = match fields_map.get("roleSearchMatching") {
                            Some(value) => value,
                            None => bail!("Missing field 'roleSearchMatching' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role_search_subtree: {
                        let field_value = match fields_map.get("roleSearchSubtree") {
                            Some(value) => value,
                            None => bail!("Missing field 'roleSearchSubtree' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_password: {
                        let field_value = match fields_map.get("serviceAccountPassword") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceAccountPassword' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_username: {
                        let field_value = match fields_map.get("serviceAccountUsername") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceAccountUsername' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_base: {
                        let field_value = match fields_map.get("userBase") {
                            Some(value) => value,
                            None => bail!("Missing field 'userBase' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_role_name: {
                        let field_value = match fields_map.get("userRoleName") {
                            Some(value) => value,
                            None => bail!("Missing field 'userRoleName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_search_matching: {
                        let field_value = match fields_map.get("userSearchMatching") {
                            Some(value) => value,
                            None => bail!("Missing field 'userSearchMatching' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_search_subtree: {
                        let field_value = match fields_map.get("userSearchSubtree") {
                            Some(value) => value,
                            None => bail!("Missing field 'userSearchSubtree' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
