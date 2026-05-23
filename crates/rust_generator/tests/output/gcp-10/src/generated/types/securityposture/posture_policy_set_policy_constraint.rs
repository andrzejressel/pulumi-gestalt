#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PosturePolicySetPolicyConstraint {
    /// Organization policy canned constraint definition.
    /// Structure is documented below.
    #[builder(into)]
    pub r#org_policy_constraint: Option<Box<super::super::types::securityposture::PosturePolicySetPolicyConstraintOrgPolicyConstraint>>,
    /// Organization policy custom constraint policy definition.
    /// Structure is documented below.
    #[builder(into)]
    pub r#org_policy_constraint_custom: Option<Box<super::super::types::securityposture::PosturePolicySetPolicyConstraintOrgPolicyConstraintCustom>>,
    /// Definition of Security Health Analytics Custom Module.
    /// Structure is documented below.
    #[builder(into)]
    pub r#security_health_analytics_custom_module: Option<Box<super::super::types::securityposture::PosturePolicySetPolicyConstraintSecurityHealthAnalyticsCustomModule>>,
    /// Security Health Analytics built-in detector definition.
    /// Structure is documented below.
    #[builder(into)]
    pub r#security_health_analytics_module: Option<Box<super::super::types::securityposture::PosturePolicySetPolicyConstraintSecurityHealthAnalyticsModule>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PosturePolicySetPolicyConstraint {
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
                    "orgPolicyConstraint",
                    &self.r#org_policy_constraint,
                ),
                to_pulumi_object_field(
                    "orgPolicyConstraintCustom",
                    &self.r#org_policy_constraint_custom,
                ),
                to_pulumi_object_field(
                    "securityHealthAnalyticsCustomModule",
                    &self.r#security_health_analytics_custom_module,
                ),
                to_pulumi_object_field(
                    "securityHealthAnalyticsModule",
                    &self.r#security_health_analytics_module,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PosturePolicySetPolicyConstraint {
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
                    r#org_policy_constraint: {
                        let field_value = match fields_map.get("orgPolicyConstraint") {
                            Some(value) => value,
                            None => bail!("Missing field 'orgPolicyConstraint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#org_policy_constraint_custom: {
                        let field_value = match fields_map.get("orgPolicyConstraintCustom") {
                            Some(value) => value,
                            None => bail!("Missing field 'orgPolicyConstraintCustom' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_health_analytics_custom_module: {
                        let field_value = match fields_map.get("securityHealthAnalyticsCustomModule") {
                            Some(value) => value,
                            None => bail!("Missing field 'securityHealthAnalyticsCustomModule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_health_analytics_module: {
                        let field_value = match fields_map.get("securityHealthAnalyticsModule") {
                            Some(value) => value,
                            None => bail!("Missing field 'securityHealthAnalyticsModule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
