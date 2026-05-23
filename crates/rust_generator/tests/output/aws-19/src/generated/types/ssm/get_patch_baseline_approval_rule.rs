#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPatchBaselineApprovalRule {
    /// Number of days after the release date of each patch matched by the rule the patch is marked as approved in the patch baseline.
    #[builder(into)]
    pub r#approve_after_days: i32,
    /// Cutoff date for auto approval of released patches. Any patches released on or before this date are installed automatically. Date is formatted as `YYYY-MM-DD`. Conflicts with `approve_after_days`
    #[builder(into)]
    pub r#approve_until_date: String,
    /// Compliance level for patches approved by this rule.
    #[builder(into)]
    pub r#compliance_level: String,
    /// Boolean enabling the application of non-security updates.
    #[builder(into)]
    pub r#enable_non_security: bool,
    /// Patch filter group that defines the criteria for the rule.
    #[builder(into)]
    pub r#patch_filters: Vec<super::super::types::ssm::GetPatchBaselineApprovalRulePatchFilter>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPatchBaselineApprovalRule {
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
                    "approveAfterDays",
                    &self.r#approve_after_days,
                ),
                to_pulumi_object_field(
                    "approveUntilDate",
                    &self.r#approve_until_date,
                ),
                to_pulumi_object_field(
                    "complianceLevel",
                    &self.r#compliance_level,
                ),
                to_pulumi_object_field(
                    "enableNonSecurity",
                    &self.r#enable_non_security,
                ),
                to_pulumi_object_field(
                    "patchFilters",
                    &self.r#patch_filters,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPatchBaselineApprovalRule {
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
                        let field_value = match fields_map.get("approveAfterDays") {
                            Some(value) => value,
                            None => bail!("Missing field 'approveAfterDays' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#approve_until_date: {
                        let field_value = match fields_map.get("approveUntilDate") {
                            Some(value) => value,
                            None => bail!("Missing field 'approveUntilDate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compliance_level: {
                        let field_value = match fields_map.get("complianceLevel") {
                            Some(value) => value,
                            None => bail!("Missing field 'complianceLevel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_non_security: {
                        let field_value = match fields_map.get("enableNonSecurity") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableNonSecurity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#patch_filters: {
                        let field_value = match fields_map.get("patchFilters") {
                            Some(value) => value,
                            None => bail!("Missing field 'patchFilters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
