#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualNetworkGatewayPolicyGroup {
    /// Is this a Default Virtual Network Gateway Policy Group? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "isDefault")]
    pub r#is_default: Option<bool>,
    /// The name of the Virtual Network Gateway Policy Group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// One or more `policy_member` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "policyMembers")]
    pub r#policy_members: Vec<super::super::types::network::VirtualNetworkGatewayPolicyGroupPolicyMember>,
    /// The priority for the Virtual Network Gateway Policy Group. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualNetworkGatewayPolicyGroup {
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
                    "is_default",
                    &self.r#is_default,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "policy_members",
                    &self.r#policy_members,
                ),
                to_pulumi_object_field(
                    "priority",
                    &self.r#priority,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualNetworkGatewayPolicyGroup {
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
                    r#is_default: {
                        let field_value = match fields_map.get("is_default") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_default' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#policy_members: {
                        let field_value = match fields_map.get("policy_members") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_members' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#priority: {
                        let field_value = match fields_map.get("priority") {
                            Some(value) => value,
                            None => bail!("Missing field 'priority' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
