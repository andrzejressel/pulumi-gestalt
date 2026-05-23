#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DefinitionEligibleAuthorization {
    /// A `just_in_time_access_policy` block as defined below.
    #[builder(into)]
    pub r#just_in_time_access_policy: Option<Box<super::super::types::lighthouse::DefinitionEligibleAuthorizationJustInTimeAccessPolicy>>,
    /// The display name of the Azure Active Directory Principal.
    #[builder(into)]
    pub r#principal_display_name: Option<String>,
    /// Principal ID of the security group/service principal/user that would be assigned permissions to the projected subscription.
    #[builder(into)]
    pub r#principal_id: String,
    /// The Principal ID of the Azure built-in role that defines the permissions that the Azure Active Directory will have on the projected scope.
    #[builder(into)]
    pub r#role_definition_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DefinitionEligibleAuthorization {
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
                    "justInTimeAccessPolicy",
                    &self.r#just_in_time_access_policy,
                ),
                to_pulumi_object_field(
                    "principalDisplayName",
                    &self.r#principal_display_name,
                ),
                to_pulumi_object_field(
                    "principalId",
                    &self.r#principal_id,
                ),
                to_pulumi_object_field(
                    "roleDefinitionId",
                    &self.r#role_definition_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DefinitionEligibleAuthorization {
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
                    r#just_in_time_access_policy: {
                        let field_value = match fields_map.get("justInTimeAccessPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'justInTimeAccessPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#principal_display_name: {
                        let field_value = match fields_map.get("principalDisplayName") {
                            Some(value) => value,
                            None => bail!("Missing field 'principalDisplayName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#principal_id: {
                        let field_value = match fields_map.get("principalId") {
                            Some(value) => value,
                            None => bail!("Missing field 'principalId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role_definition_id: {
                        let field_value = match fields_map.get("roleDefinitionId") {
                            Some(value) => value,
                            None => bail!("Missing field 'roleDefinitionId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
