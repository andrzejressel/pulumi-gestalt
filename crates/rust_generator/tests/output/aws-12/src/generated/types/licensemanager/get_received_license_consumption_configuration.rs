#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetReceivedLicenseConsumptionConfiguration {
    /// Details about a borrow configuration. Detailed below
    #[builder(into)]
    #[serde(rename = "borrowConfigurations")]
    pub r#borrow_configurations: Vec<super::super::types::licensemanager::GetReceivedLicenseConsumptionConfigurationBorrowConfiguration>,
    /// Details about a provisional configuration. Detailed below
    #[builder(into)]
    #[serde(rename = "provisionalConfigurations")]
    pub r#provisional_configurations: Vec<super::super::types::licensemanager::GetReceivedLicenseConsumptionConfigurationProvisionalConfiguration>,
    #[builder(into)]
    #[serde(rename = "renewType")]
    pub r#renew_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetReceivedLicenseConsumptionConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("borrow_configurations".to_string(), self.r#borrow_configurations.to_pulumi_value().await);
            map.insert("provisional_configurations".to_string(), self.r#provisional_configurations.to_pulumi_value().await);
            map.insert("renew_type".to_string(), self.r#renew_type.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetReceivedLicenseConsumptionConfiguration {
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
                    r#borrow_configurations: {
                        let field_value = match fields_map.get("borrow_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'borrow_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::licensemanager::GetReceivedLicenseConsumptionConfigurationBorrowConfiguration> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#provisional_configurations: {
                        let field_value = match fields_map.get("provisional_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisional_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::licensemanager::GetReceivedLicenseConsumptionConfigurationProvisionalConfiguration> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#renew_type: {
                        let field_value = match fields_map.get("renew_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'renew_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
