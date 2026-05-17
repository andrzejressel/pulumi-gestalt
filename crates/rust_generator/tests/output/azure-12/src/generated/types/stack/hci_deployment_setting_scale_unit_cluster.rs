#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HciDeploymentSettingScaleUnitCluster {
    /// Specifies the Azure blob service endpoint, for example, `core.windows.net`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "azureServiceEndpoint")]
    pub r#azure_service_endpoint: String,
    /// Specifies the Azure Storage account name of the cloud witness for the Azure Stack HCI cluster. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "cloudAccountName")]
    pub r#cloud_account_name: String,
    /// Specifies the name of the cluster. It must be 3-15 characters long and contain only letters, numbers and hyphens. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specifies the fileshare path of the local witness for the Azure Stack HCI cluster. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "witnessPath")]
    pub r#witness_path: String,
    /// Specifies the type of the witness. Possible values are `Cloud`, `FileShare`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "witnessType")]
    pub r#witness_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HciDeploymentSettingScaleUnitCluster {
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
                "azure_service_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#azure_service_endpoint,
                )
                .await,
            );
            map.insert(
                "cloud_account_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloud_account_name,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "witness_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#witness_path,
                )
                .await,
            );
            map.insert(
                "witness_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#witness_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HciDeploymentSettingScaleUnitCluster {
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
                    r#azure_service_endpoint: {
                        let field_value = match fields_map.get("azure_service_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'azure_service_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloud_account_name: {
                        let field_value = match fields_map.get("cloud_account_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_account_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#witness_path: {
                        let field_value = match fields_map.get("witness_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'witness_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#witness_type: {
                        let field_value = match fields_map.get("witness_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'witness_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
