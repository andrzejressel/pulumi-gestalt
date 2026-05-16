#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetConfigurationSetVdmOption {
    /// Specifies additional settings for your VDM configuration as applicable to the Dashboard.
    #[builder(into)]
    #[serde(rename = "dashboardOptions")]
    pub r#dashboard_options: Vec<super::super::types::sesv2::GetConfigurationSetVdmOptionDashboardOption>,
    /// Specifies additional settings for your VDM configuration as applicable to the Guardian.
    #[builder(into)]
    #[serde(rename = "guardianOptions")]
    pub r#guardian_options: Vec<super::super::types::sesv2::GetConfigurationSetVdmOptionGuardianOption>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetConfigurationSetVdmOption {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("dashboard_options".to_string(), self.r#dashboard_options.to_pulumi_value().await);
            map.insert("guardian_options".to_string(), self.r#guardian_options.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetConfigurationSetVdmOption {
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
                    r#dashboard_options: {
                        let field_value = match fields_map.get("dashboard_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'dashboard_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::sesv2::GetConfigurationSetVdmOptionDashboardOption> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#guardian_options: {
                        let field_value = match fields_map.get("guardian_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'guardian_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::sesv2::GetConfigurationSetVdmOptionGuardianOption> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
