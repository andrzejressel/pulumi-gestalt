#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DenyPolicyRuleDenyRule {
    /// User defined CEVAL expression. A CEVAL expression is used to specify match criteria such as origin.ip, source.region_code and contents in the request header.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "denialCondition")]
    pub r#denial_condition: Option<Box<super::super::types::iam::DenyPolicyRuleDenyRuleDenialCondition>>,
    /// The permissions that are explicitly denied by this rule. Each permission uses the format `{service-fqdn}/{resource}.{verb}`,
    /// where `{service-fqdn}` is the fully qualified domain name for the service. For example, `iam.googleapis.com/roles.list`.
    #[builder(into)]
    #[serde(rename = "deniedPermissions")]
    pub r#denied_permissions: Option<Vec<String>>,
    /// The identities that are prevented from using one or more permissions on Google Cloud resources.
    #[builder(into)]
    #[serde(rename = "deniedPrincipals")]
    pub r#denied_principals: Option<Vec<String>>,
    /// Specifies the permissions that this rule excludes from the set of denied permissions given by deniedPermissions.
    /// If a permission appears in deniedPermissions and in exceptionPermissions then it will not be denied.
    /// The excluded permissions can be specified using the same syntax as deniedPermissions.
    #[builder(into)]
    #[serde(rename = "exceptionPermissions")]
    pub r#exception_permissions: Option<Vec<String>>,
    /// The identities that are excluded from the deny rule, even if they are listed in the deniedPrincipals.
    /// For example, you could add a Google group to the deniedPrincipals, then exclude specific users who belong to that group.
    #[builder(into)]
    #[serde(rename = "exceptionPrincipals")]
    pub r#exception_principals: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DenyPolicyRuleDenyRule {
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
                    "denial_condition",
                    &self.r#denial_condition,
                ),
                to_pulumi_object_field(
                    "denied_permissions",
                    &self.r#denied_permissions,
                ),
                to_pulumi_object_field(
                    "denied_principals",
                    &self.r#denied_principals,
                ),
                to_pulumi_object_field(
                    "exception_permissions",
                    &self.r#exception_permissions,
                ),
                to_pulumi_object_field(
                    "exception_principals",
                    &self.r#exception_principals,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DenyPolicyRuleDenyRule {
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
                    r#denial_condition: {
                        let field_value = match fields_map.get("denial_condition") {
                            Some(value) => value,
                            None => bail!("Missing field 'denial_condition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#denied_permissions: {
                        let field_value = match fields_map.get("denied_permissions") {
                            Some(value) => value,
                            None => bail!("Missing field 'denied_permissions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#denied_principals: {
                        let field_value = match fields_map.get("denied_principals") {
                            Some(value) => value,
                            None => bail!("Missing field 'denied_principals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exception_permissions: {
                        let field_value = match fields_map.get("exception_permissions") {
                            Some(value) => value,
                            None => bail!("Missing field 'exception_permissions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exception_principals: {
                        let field_value = match fields_map.get("exception_principals") {
                            Some(value) => value,
                            None => bail!("Missing field 'exception_principals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
