#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegistrationManagementSettings {
    /// The desired renewal method for this Registration. The actual renewalMethod is automatically updated to reflect this choice.
    /// If unset or equal to RENEWAL_METHOD_UNSPECIFIED, the actual renewalMethod is treated as if it were set to AUTOMATIC_RENEWAL.
    /// You cannot use RENEWAL_DISABLED during resource creation, and you can update the renewal status only when the Registration
    /// resource has state ACTIVE or SUSPENDED.
    /// When preferredRenewalMethod is set to AUTOMATIC_RENEWAL, the actual renewalMethod can be set to RENEWAL_DISABLED in case of
    /// problems with the billing account or reported domain abuse. In such cases, check the issues field on the Registration. After
    /// the problem is resolved, the renewalMethod is automatically updated to preferredRenewalMethod in a few hours.
    #[builder(into)]
    #[serde(rename = "preferredRenewalMethod")]
    pub r#preferred_renewal_method: Option<String>,
    /// (Output)
    /// Output only. The actual renewal method for this Registration. When preferredRenewalMethod is set to AUTOMATIC_RENEWAL,
    /// the actual renewalMethod can be equal to RENEWAL_DISABLED—for example, when there are problems with the billing account
    /// or reported domain abuse. In such cases, check the issues field on the Registration. After the problem is resolved, the
    /// renewalMethod is automatically updated to preferredRenewalMethod in a few hours.
    #[builder(into)]
    #[serde(rename = "renewalMethod")]
    pub r#renewal_method: Option<String>,
    /// Controls whether the domain can be transferred to another registrar. Values are UNLOCKED or LOCKED.
    #[builder(into)]
    #[serde(rename = "transferLockState")]
    pub r#transfer_lock_state: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegistrationManagementSettings {
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
                "preferred_renewal_method".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preferred_renewal_method,
                )
                .await,
            );
            map.insert(
                "renewal_method".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#renewal_method,
                )
                .await,
            );
            map.insert(
                "transfer_lock_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transfer_lock_state,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegistrationManagementSettings {
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
                    r#preferred_renewal_method: {
                        let field_value = match fields_map.get("preferred_renewal_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'preferred_renewal_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#renewal_method: {
                        let field_value = match fields_map.get("renewal_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'renewal_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transfer_lock_state: {
                        let field_value = match fields_map.get("transfer_lock_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'transfer_lock_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
