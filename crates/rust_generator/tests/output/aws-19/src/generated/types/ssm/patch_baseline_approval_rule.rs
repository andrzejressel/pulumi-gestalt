#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PatchBaselineApprovalRule {
    /// Number of days after the release date of each patch matched by the rule the patch is marked as approved in the patch baseline. Valid Range: 0 to 360. Conflicts with `approve_until_date`.
    #[builder(into)]
    #[serde(rename = "approveAfterDays")]
    pub r#approve_after_days: Option<i32>,
    /// Cutoff date for auto approval of released patches. Any patches released on or before this date are installed automatically. Date is formatted as `YYYY-MM-DD`. Conflicts with `approve_after_days`
    #[builder(into)]
    #[serde(rename = "approveUntilDate")]
    pub r#approve_until_date: Option<String>,
    /// Compliance level for patches approved by this rule. Valid values are `CRITICAL`, `HIGH`, `MEDIUM`, `LOW`, `INFORMATIONAL`, and `UNSPECIFIED`. The default value is `UNSPECIFIED`.
    #[builder(into)]
    #[serde(rename = "complianceLevel")]
    pub r#compliance_level: Option<String>,
    /// Boolean enabling the application of non-security updates. The default value is `false`. Valid for Linux instances only.
    #[builder(into)]
    #[serde(rename = "enableNonSecurity")]
    pub r#enable_non_security: Option<bool>,
    /// Patch filter group that defines the criteria for the rule. Up to 5 patch filters can be specified per approval rule using Key/Value pairs. Valid combinations of these Keys and the `operating_system` value can be found in the [SSM DescribePatchProperties API Reference](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_DescribePatchProperties.html). Valid Values are exact values for the patch property given as the key, or a wildcard `*`, which matches all values. `PATCH_SET` defaults to `OS` if unspecified
    #[builder(into)]
    #[serde(rename = "patchFilters")]
    pub r#patch_filters: Vec<super::super::types::ssm::PatchBaselineApprovalRulePatchFilter>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PatchBaselineApprovalRule {
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
                "approve_after_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#approve_after_days,
                )
                .await,
            );
            map.insert(
                "approve_until_date".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#approve_until_date,
                )
                .await,
            );
            map.insert(
                "compliance_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#compliance_level,
                )
                .await,
            );
            map.insert(
                "enable_non_security".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_non_security,
                )
                .await,
            );
            map.insert(
                "patch_filters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#patch_filters,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PatchBaselineApprovalRule {
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
                    r#approve_after_days: {
                        let field_value = match fields_map.get("approve_after_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'approve_after_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#approve_until_date: {
                        let field_value = match fields_map.get("approve_until_date") {
                            Some(value) => value,
                            None => bail!("Missing field 'approve_until_date' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compliance_level: {
                        let field_value = match fields_map.get("compliance_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'compliance_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_non_security: {
                        let field_value = match fields_map.get("enable_non_security") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_non_security' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#patch_filters: {
                        let field_value = match fields_map.get("patch_filters") {
                            Some(value) => value,
                            None => bail!("Missing field 'patch_filters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
