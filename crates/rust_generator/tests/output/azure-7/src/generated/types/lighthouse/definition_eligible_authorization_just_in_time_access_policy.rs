#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DefinitionEligibleAuthorizationJustInTimeAccessPolicy {
    /// An `approver` block as defined below.
    #[builder(into)]
    #[serde(rename = "approvers")]
    pub r#approvers: Option<Vec<super::super::types::lighthouse::DefinitionEligibleAuthorizationJustInTimeAccessPolicyApprover>>,
    /// The maximum access duration in ISO 8601 format for just-in-time access requests. Defaults to `PT8H`.
    #[builder(into)]
    #[serde(rename = "maximumActivationDuration")]
    pub r#maximum_activation_duration: Option<String>,
    /// The multi-factor authorization provider to be used for just-in-time access requests. Possible value is `Azure`.
    /// 
    /// > **Note:** When this property isn't set, it would be set to `None`.
    #[builder(into)]
    #[serde(rename = "multiFactorAuthProvider")]
    pub r#multi_factor_auth_provider: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DefinitionEligibleAuthorizationJustInTimeAccessPolicy {
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
                "approvers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#approvers,
                )
                .await,
            );
            map.insert(
                "maximum_activation_duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maximum_activation_duration,
                )
                .await,
            );
            map.insert(
                "multi_factor_auth_provider".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#multi_factor_auth_provider,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DefinitionEligibleAuthorizationJustInTimeAccessPolicy {
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
                    r#approvers: {
                        let field_value = match fields_map.get("approvers") {
                            Some(value) => value,
                            None => bail!("Missing field 'approvers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_activation_duration: {
                        let field_value = match fields_map.get("maximum_activation_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_activation_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#multi_factor_auth_provider: {
                        let field_value = match fields_map.get("multi_factor_auth_provider") {
                            Some(value) => value,
                            None => bail!("Missing field 'multi_factor_auth_provider' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
