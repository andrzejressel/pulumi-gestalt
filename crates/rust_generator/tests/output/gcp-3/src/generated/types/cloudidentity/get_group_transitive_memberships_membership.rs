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
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "member",
                    &self.r#member,
                ),
                to_pulumi_object_field(
                    "preferred_member_keys",
                    &self.r#preferred_member_keys,
                ),
                to_pulumi_object_field(
                    "relation_type",
                    &self.r#relation_type,
                ),
                to_pulumi_object_field(
                    "roles",
                    &self.r#roles,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
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
