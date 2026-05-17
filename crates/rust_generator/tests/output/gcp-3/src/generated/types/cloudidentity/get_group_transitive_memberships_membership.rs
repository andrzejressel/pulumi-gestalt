#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetGroupTransitiveMembershipsMembership {
    /// Resource name for this member.
    #[builder(into)]
    #[serde(rename = "member")]
    pub r#member: String,
    /// EntityKey of the member. Entity key has an id and a namespace. In case of discussion forums, the id will be an email address without a namespace.
    #[builder(into)]
    #[serde(rename = "preferredMemberKeys")]
    pub r#preferred_member_keys: Vec<super::super::types::cloudidentity::GetGroupTransitiveMembershipsMembershipPreferredMemberKey>,
    /// The relation between the group and the transitive member. The value can be DIRECT, INDIRECT, or DIRECT_AND_INDIRECT
    #[builder(into)]
    #[serde(rename = "relationType")]
    pub r#relation_type: String,
    /// The membership role details
    #[builder(into)]
    #[serde(rename = "roles")]
    pub r#roles: Vec<super::super::types::cloudidentity::GetGroupTransitiveMembershipsMembershipRole>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetGroupTransitiveMembershipsMembership {
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
                "member".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#member,
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
                "relation_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#relation_type,
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

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetGroupTransitiveMembershipsMembership {
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
                    r#member: {
                        let field_value = match fields_map.get("member") {
                            Some(value) => value,
                            None => bail!("Missing field 'member' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#relation_type: {
                        let field_value = match fields_map.get("relation_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'relation_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
