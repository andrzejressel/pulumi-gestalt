#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReportPlanReportSetting {
    /// Specifies the list of accounts a report covers.
    #[builder(into)]
    #[serde(rename = "accounts")]
    pub r#accounts: Option<Vec<String>>,
    /// Specifies the Amazon Resource Names (ARNs) of the frameworks a report covers.
    #[builder(into)]
    #[serde(rename = "frameworkArns")]
    pub r#framework_arns: Option<Vec<String>>,
    /// Specifies the number of frameworks a report covers.
    #[builder(into)]
    #[serde(rename = "numberOfFrameworks")]
    pub r#number_of_frameworks: Option<i32>,
    /// Specifies the list of Organizational Units a report covers.
    #[builder(into)]
    #[serde(rename = "organizationUnits")]
    pub r#organization_units: Option<Vec<String>>,
    /// Specifies the list of regions a report covers.
    #[builder(into)]
    #[serde(rename = "regions")]
    pub r#regions: Option<Vec<String>>,
    /// Identifies the report template for the report. Reports are built using a report template. The report templates are: `RESOURCE_COMPLIANCE_REPORT` | `CONTROL_COMPLIANCE_REPORT` | `BACKUP_JOB_REPORT` | `COPY_JOB_REPORT` | `RESTORE_JOB_REPORT`.
    #[builder(into)]
    #[serde(rename = "reportTemplate")]
    pub r#report_template: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ReportPlanReportSetting {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("accounts".to_string(), self.r#accounts.to_pulumi_value().await);
            map.insert("framework_arns".to_string(), self.r#framework_arns.to_pulumi_value().await);
            map.insert("number_of_frameworks".to_string(), self.r#number_of_frameworks.to_pulumi_value().await);
            map.insert("organization_units".to_string(), self.r#organization_units.to_pulumi_value().await);
            map.insert("regions".to_string(), self.r#regions.to_pulumi_value().await);
            map.insert("report_template".to_string(), self.r#report_template.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ReportPlanReportSetting {
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
                    r#accounts: {
                        let field_value = match fields_map.get("accounts") {
                            Some(value) => value,
                            None => bail!("Missing field 'accounts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#framework_arns: {
                        let field_value = match fields_map.get("framework_arns") {
                            Some(value) => value,
                            None => bail!("Missing field 'framework_arns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#number_of_frameworks: {
                        let field_value = match fields_map.get("number_of_frameworks") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_of_frameworks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#organization_units: {
                        let field_value = match fields_map.get("organization_units") {
                            Some(value) => value,
                            None => bail!("Missing field 'organization_units' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#regions: {
                        let field_value = match fields_map.get("regions") {
                            Some(value) => value,
                            None => bail!("Missing field 'regions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#report_template: {
                        let field_value = match fields_map.get("report_template") {
                            Some(value) => value,
                            None => bail!("Missing field 'report_template' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
