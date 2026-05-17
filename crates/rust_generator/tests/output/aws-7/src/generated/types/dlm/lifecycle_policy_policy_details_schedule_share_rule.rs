#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LifecyclePolicyPolicyDetailsScheduleShareRule {
    /// The IDs of the AWS accounts with which to share the snapshots.
    #[builder(into)]
    #[serde(rename = "targetAccounts")]
    pub r#target_accounts: Vec<String>,
    /// The period after which snapshots that are shared with other AWS accounts are automatically unshared.
    #[builder(into)]
    #[serde(rename = "unshareInterval")]
    pub r#unshare_interval: Option<i32>,
    /// The unit of time for the automatic unsharing interval. Valid values are `DAYS`, `WEEKS`, `MONTHS`, `YEARS`.
    #[builder(into)]
    #[serde(rename = "unshareIntervalUnit")]
    pub r#unshare_interval_unit: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LifecyclePolicyPolicyDetailsScheduleShareRule {
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
                "target_accounts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_accounts,
                )
                .await,
            );
            map.insert(
                "unshare_interval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#unshare_interval,
                )
                .await,
            );
            map.insert(
                "unshare_interval_unit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#unshare_interval_unit,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LifecyclePolicyPolicyDetailsScheduleShareRule {
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
                    r#target_accounts: {
                        let field_value = match fields_map.get("target_accounts") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_accounts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unshare_interval: {
                        let field_value = match fields_map.get("unshare_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'unshare_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unshare_interval_unit: {
                        let field_value = match fields_map.get("unshare_interval_unit") {
                            Some(value) => value,
                            None => bail!("Missing field 'unshare_interval_unit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
