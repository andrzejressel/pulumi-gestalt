#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetGroupMembershipsMembership {
    /// The time when the Membership was created.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: String,
    /// The parent Group resource under which to lookup the Membership names. Must be of the form groups/{group_id}.
    #[builder(into)]
    #[serde(rename = "group")]
    pub r#group: String,
    /// EntityKey of the member.  Structure is documented below.
    #[builder(into)]
    #[serde(rename = "memberKeys")]
    pub r#member_keys: Vec<super::super::types::cloudidentity::GetGroupMembershipsMembershipMemberKey>,
    /// The name of the MembershipRole. One of OWNER, MANAGER, MEMBER.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// EntityKey of the member.  Structure is documented below.
    #[builder(into)]
    #[serde(rename = "preferredMemberKeys")]
    pub r#preferred_member_keys: Vec<super::super::types::cloudidentity::GetGroupMembershipsMembershipPreferredMemberKey>,
    /// The MembershipRoles that apply to the Membership. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "roles")]
    pub r#roles: Vec<super::super::types::cloudidentity::GetGroupMembershipsMembershipRole>,
    /// The type of the membership.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// The time when the Membership was last updated.
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetGroupMembershipsMembership {
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
                "create_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#create_time,
                )
                .await,
            );
            map.insert(
                "group".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#group,
                )
                .await,
            );
            map.insert(
                "member_keys".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#member_keys,
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
                "preferred_member_keys".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preferred_member_keys,
                )
                .await,
            );
            map.insert(
                "roles".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#roles,
                )
                .await,
            );
            map.insert(
                "type_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetGroupMembershipsMembership {
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
                    r#create_time: {
                        let field_value = match fields_map.get("create_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'create_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#group: {
                        let field_value = match fields_map.get("group") {
                            Some(value) => value,
                            None => bail!("Missing field 'group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#member_keys: {
                        let field_value = match fields_map.get("member_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'member_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#preferred_member_keys: {
                        let field_value = match fields_map.get("preferred_member_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'preferred_member_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#roles: {
                        let field_value = match fields_map.get("roles") {
                            Some(value) => value,
                            None => bail!("Missing field 'roles' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
