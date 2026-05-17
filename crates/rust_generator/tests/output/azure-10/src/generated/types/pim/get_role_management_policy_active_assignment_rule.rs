#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRoleManagementPolicyActiveAssignmentRule {
    /// (Boolean) Must an assignment have an expiry date.
    #[builder(into)]
    #[serde(rename = "expirationRequired")]
    pub r#expiration_required: bool,
    /// (String) The maximum length of time an assignment can be valid, as an ISO8601 duration.
    #[builder(into)]
    #[serde(rename = "expireAfter")]
    pub r#expire_after: String,
    /// (Boolean) Is a justification required to create new assignments.
    #[builder(into)]
    #[serde(rename = "requireJustification")]
    pub r#require_justification: bool,
    /// (Boolean) Is multi-factor authentication required to create new assignments.
    #[builder(into)]
    #[serde(rename = "requireMultifactorAuthentication")]
    pub r#require_multifactor_authentication: bool,
    /// (Boolean) Is ticket information required to create new assignments.
    #[builder(into)]
    #[serde(rename = "requireTicketInfo")]
    pub r#require_ticket_info: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRoleManagementPolicyActiveAssignmentRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "expiration_required".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expiration_required,
                )
                .await,
            );
            map.insert(
                "expire_after".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expire_after,
                )
                .await,
            );
            map.insert(
                "require_justification".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#require_justification,
                )
                .await,
            );
            map.insert(
                "require_multifactor_authentication".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#require_multifactor_authentication,
                )
                .await,
            );
            map.insert(
                "require_ticket_info".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#require_ticket_info,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRoleManagementPolicyActiveAssignmentRule {
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
                    r#expiration_required: {
                        let field_value = match fields_map.get("expiration_required") {
                            Some(value) => value,
                            None => bail!("Missing field 'expiration_required' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expire_after: {
                        let field_value = match fields_map.get("expire_after") {
                            Some(value) => value,
                            None => bail!("Missing field 'expire_after' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_justification: {
                        let field_value = match fields_map.get("require_justification") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_justification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_multifactor_authentication: {
                        let field_value = match fields_map.get("require_multifactor_authentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_multifactor_authentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_ticket_info: {
                        let field_value = match fields_map.get("require_ticket_info") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_ticket_info' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
