#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainDomainSettingsRStudioServerProDomainSettings {
    /// The default instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance. see `default_resource_spec` Block above.
    #[builder(into)]
    #[serde(rename = "defaultResourceSpec")]
    pub r#default_resource_spec: Option<Box<super::super::types::sagemaker::DomainDomainSettingsRStudioServerProDomainSettingsDefaultResourceSpec>>,
    /// The ARN of the execution role for the RStudioServerPro Domain-level app.
    #[builder(into)]
    #[serde(rename = "domainExecutionRoleArn")]
    pub r#domain_execution_role_arn: String,
    /// A URL pointing to an RStudio Connect server.
    #[builder(into)]
    #[serde(rename = "rStudioConnectUrl")]
    pub r#r_studio_connect_url: Option<String>,
    /// A URL pointing to an RStudio Package Manager server.
    #[builder(into)]
    #[serde(rename = "rStudioPackageManagerUrl")]
    pub r#r_studio_package_manager_url: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainDomainSettingsRStudioServerProDomainSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "default_resource_spec",
                    &self.r#default_resource_spec,
                ),
                to_pulumi_object_field(
                    "domain_execution_role_arn",
                    &self.r#domain_execution_role_arn,
                ),
                to_pulumi_object_field(
                    "r_studio_connect_url",
                    &self.r#r_studio_connect_url,
                ),
                to_pulumi_object_field(
                    "r_studio_package_manager_url",
                    &self.r#r_studio_package_manager_url,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainDomainSettingsRStudioServerProDomainSettings {
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
                    r#default_resource_spec: {
                        let field_value = match fields_map.get("default_resource_spec") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_resource_spec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domain_execution_role_arn: {
                        let field_value = match fields_map.get("domain_execution_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_execution_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#r_studio_connect_url: {
                        let field_value = match fields_map.get("r_studio_connect_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'r_studio_connect_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#r_studio_package_manager_url: {
                        let field_value = match fields_map.get("r_studio_package_manager_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'r_studio_package_manager_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
