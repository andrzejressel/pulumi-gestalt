#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetReportPlanReportSetting {
    /// (Optional) Specifies the list of accounts a report covers.
    #[builder(into)]
    pub r#accounts: Vec<String>,
    /// ARNs of the frameworks a report covers.
    #[builder(into)]
    pub r#framework_arns: Vec<String>,
    /// Specifies the number of frameworks a report covers.
    #[builder(into)]
    pub r#number_of_frameworks: i32,
    /// (Optional) Specifies the list of Organizational Units a report covers.
    #[builder(into)]
    pub r#organization_units: Vec<String>,
    /// (Optional) Specifies the list of regions a report covers.
    #[builder(into)]
    pub r#regions: Vec<String>,
    /// Identifies the report template for the report. Reports are built using a report template.
    #[builder(into)]
    pub r#report_template: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetReportPlanReportSetting {
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
                    "accounts",
                    &self.r#accounts,
                ),
                to_pulumi_object_field(
                    "frameworkArns",
                    &self.r#framework_arns,
                ),
                to_pulumi_object_field(
                    "numberOfFrameworks",
                    &self.r#number_of_frameworks,
                ),
                to_pulumi_object_field(
                    "organizationUnits",
                    &self.r#organization_units,
                ),
                to_pulumi_object_field(
                    "regions",
                    &self.r#regions,
                ),
                to_pulumi_object_field(
                    "reportTemplate",
                    &self.r#report_template,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetReportPlanReportSetting {
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
                    r#accounts: {
                        let field_value = match fields_map.get("accounts") {
                            Some(value) => value,
                            None => bail!("Missing field 'accounts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#framework_arns: {
                        let field_value = match fields_map.get("frameworkArns") {
                            Some(value) => value,
                            None => bail!("Missing field 'frameworkArns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_of_frameworks: {
                        let field_value = match fields_map.get("numberOfFrameworks") {
                            Some(value) => value,
                            None => bail!("Missing field 'numberOfFrameworks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#organization_units: {
                        let field_value = match fields_map.get("organizationUnits") {
                            Some(value) => value,
                            None => bail!("Missing field 'organizationUnits' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#regions: {
                        let field_value = match fields_map.get("regions") {
                            Some(value) => value,
                            None => bail!("Missing field 'regions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#report_template: {
                        let field_value = match fields_map.get("reportTemplate") {
                            Some(value) => value,
                            None => bail!("Missing field 'reportTemplate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
