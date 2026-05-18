#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineConfigurationAssignmentConfiguration {
    /// The assignment type for the Guest Configuration Assignment. Possible values are `Audit`, `ApplyAndAutoCorrect`, `ApplyAndMonitor` and `DeployAndAutoCorrect`.
    #[builder(into)]
    #[serde(rename = "assignmentType")]
    pub r#assignment_type: Option<String>,
    /// The content hash for the Guest Configuration package.
    #[builder(into)]
    #[serde(rename = "contentHash")]
    pub r#content_hash: Option<String>,
    /// The content URI where the Guest Configuration package is stored.
    /// 
    /// > **NOTE:** When deploying a Custom Guest Configuration package the `content_hash` and `content_uri` fields must be defined. For Built-in Guest Configuration packages, such as the `AzureWindowsBaseline` package, the `content_hash` and `content_uri` should not be defined, rather these fields will be returned after the Built-in Guest Configuration package has been provisioned. For more information on guest configuration assignments please see the [product documentation](https://docs.microsoft.com/azure/governance/policy/concepts/guest-configuration-assignments).
    #[builder(into)]
    #[serde(rename = "contentUri")]
    pub r#content_uri: Option<String>,
    /// One or more `parameter` blocks as defined below which define what configuration parameters and values against.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<Vec<super::super::types::policy::VirtualMachineConfigurationAssignmentConfigurationParameter>>,
    /// The version of the Guest Configuration that will be assigned in this Guest Configuration Assignment.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualMachineConfigurationAssignmentConfiguration {
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
                    "assignment_type",
                    &self.r#assignment_type,
                ),
                to_pulumi_object_field(
                    "content_hash",
                    &self.r#content_hash,
                ),
                to_pulumi_object_field(
                    "content_uri",
                    &self.r#content_uri,
                ),
                to_pulumi_object_field(
                    "parameters",
                    &self.r#parameters,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualMachineConfigurationAssignmentConfiguration {
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
                    r#assignment_type: {
                        let field_value = match fields_map.get("assignment_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'assignment_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#content_hash: {
                        let field_value = match fields_map.get("content_hash") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_hash' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#content_uri: {
                        let field_value = match fields_map.get("content_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parameters: {
                        let field_value = match fields_map.get("parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
