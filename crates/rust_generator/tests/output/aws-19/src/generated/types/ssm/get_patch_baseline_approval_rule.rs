#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPatchBaselineApprovalRule {
    /// Number of days after the release date of each patch matched by the rule the patch is marked as approved in the patch baseline.
    #[builder(into)]
    #[serde(rename = "approveAfterDays")]
    pub r#approve_after_days: i32,
    /// Cutoff date for auto approval of released patches. Any patches released on or before this date are installed automatically. Date is formatted as `YYYY-MM-DD`. Conflicts with `approve_after_days`
    #[builder(into)]
    #[serde(rename = "approveUntilDate")]
    pub r#approve_until_date: String,
    /// Compliance level for patches approved by this rule.
    #[builder(into)]
    #[serde(rename = "complianceLevel")]
    pub r#compliance_level: String,
    /// Boolean enabling the application of non-security updates.
    #[builder(into)]
    #[serde(rename = "enableNonSecurity")]
    pub r#enable_non_security: bool,
    /// Patch filter group that defines the criteria for the rule.
    #[builder(into)]
    #[serde(rename = "patchFilters")]
    pub r#patch_filters: Vec<super::super::types::ssm::GetPatchBaselineApprovalRulePatchFilter>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPatchBaselineApprovalRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("approve_after_days".to_string(), self.r#approve_after_days.to_pulumi_value().await);
            map.insert("approve_until_date".to_string(), self.r#approve_until_date.to_pulumi_value().await);
            map.insert("compliance_level".to_string(), self.r#compliance_level.to_pulumi_value().await);
            map.insert("enable_non_security".to_string(), self.r#enable_non_security.to_pulumi_value().await);
            map.insert("patch_filters".to_string(), self.r#patch_filters.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPatchBaselineApprovalRule {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#approve_after_days: {
                        let field_value = match fields_map.get("approve_after_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'approve_after_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#approve_until_date: {
                        let field_value = match fields_map.get("approve_until_date") {
                            Some(value) => value,
                            None => bail!("Missing field 'approve_until_date' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#compliance_level: {
                        let field_value = match fields_map.get("compliance_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'compliance_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#enable_non_security: {
                        let field_value = match fields_map.get("enable_non_security") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_non_security' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#patch_filters: {
                        let field_value = match fields_map.get("patch_filters") {
                            Some(value) => value,
                            None => bail!("Missing field 'patch_filters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::ssm::GetPatchBaselineApprovalRulePatchFilter> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
