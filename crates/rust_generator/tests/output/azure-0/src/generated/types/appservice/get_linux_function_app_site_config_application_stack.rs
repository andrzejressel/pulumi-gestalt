#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLinuxFunctionAppSiteConfigApplicationStack {
    /// One or more `docker` blocks as defined below.
    #[builder(into)]
    pub r#dockers: Vec<super::super::types::appservice::GetLinuxFunctionAppSiteConfigApplicationStackDocker>,
    /// The version of .NET used.
    #[builder(into)]
    pub r#dotnet_version: String,
    /// The Version of Java used.
    #[builder(into)]
    pub r#java_version: String,
    /// The version of Node used.
    #[builder(into)]
    pub r#node_version: String,
    /// The version of PowerShell Core used.
    #[builder(into)]
    pub r#powershell_core_version: String,
    /// The version of Python used.
    #[builder(into)]
    pub r#python_version: String,
    /// Does the Linux Function App use a custom runtime?
    #[builder(into)]
    pub r#use_custom_runtime: bool,
    #[builder(into)]
    pub r#use_dotnet_isolated_runtime: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLinuxFunctionAppSiteConfigApplicationStack {
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
                    "dockers",
                    &self.r#dockers,
                ),
                to_pulumi_object_field(
                    "dotnetVersion",
                    &self.r#dotnet_version,
                ),
                to_pulumi_object_field(
                    "javaVersion",
                    &self.r#java_version,
                ),
                to_pulumi_object_field(
                    "nodeVersion",
                    &self.r#node_version,
                ),
                to_pulumi_object_field(
                    "powershellCoreVersion",
                    &self.r#powershell_core_version,
                ),
                to_pulumi_object_field(
                    "pythonVersion",
                    &self.r#python_version,
                ),
                to_pulumi_object_field(
                    "useCustomRuntime",
                    &self.r#use_custom_runtime,
                ),
                to_pulumi_object_field(
                    "useDotnetIsolatedRuntime",
                    &self.r#use_dotnet_isolated_runtime,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLinuxFunctionAppSiteConfigApplicationStack {
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
                    r#dockers: {
                        let field_value = match fields_map.get("dockers") {
                            Some(value) => value,
                            None => bail!("Missing field 'dockers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dotnet_version: {
                        let field_value = match fields_map.get("dotnetVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'dotnetVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#java_version: {
                        let field_value = match fields_map.get("javaVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'javaVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_version: {
                        let field_value = match fields_map.get("nodeVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'nodeVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#powershell_core_version: {
                        let field_value = match fields_map.get("powershellCoreVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'powershellCoreVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#python_version: {
                        let field_value = match fields_map.get("pythonVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'pythonVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_custom_runtime: {
                        let field_value = match fields_map.get("useCustomRuntime") {
                            Some(value) => value,
                            None => bail!("Missing field 'useCustomRuntime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_dotnet_isolated_runtime: {
                        let field_value = match fields_map.get("useDotnetIsolatedRuntime") {
                            Some(value) => value,
                            None => bail!("Missing field 'useDotnetIsolatedRuntime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
