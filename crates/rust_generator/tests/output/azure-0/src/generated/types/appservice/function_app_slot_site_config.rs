#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FunctionAppSlotSiteConfig {
    /// Should the Function App be loaded at all times? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "alwaysOn")]
    pub r#always_on: Option<bool>,
    /// The number of workers this function app can scale out to. Only applicable to apps on the Consumption and Premium plan.
    #[builder(into)]
    #[serde(rename = "appScaleLimit")]
    pub r#app_scale_limit: Option<i32>,
    /// The name of the slot to automatically swap to during deployment
    #[builder(into)]
    #[serde(rename = "autoSwapSlotName")]
    pub r#auto_swap_slot_name: Option<String>,
    /// A `cors` block as defined below.
    #[builder(into)]
    #[serde(rename = "cors")]
    pub r#cors: Option<Box<super::super::types::appservice::FunctionAppSlotSiteConfigCors>>,
    /// The version of the .NET framework's CLR used in this function app. Possible values are `v4.0` (including .NET Core 2.1 and 3.1), `v5.0` and `v6.0`. [For more information on which .NET Framework version to use based on the runtime version you're targeting - please see this table](https://docs.microsoft.com/azure/azure-functions/functions-dotnet-class-library#supported-versions). Defaults to `v4.0`.
    #[builder(into)]
    #[serde(rename = "dotnetFrameworkVersion")]
    pub r#dotnet_framework_version: Option<String>,
    /// The number of minimum instances for this function app. Only applicable to apps on the Premium plan.
    #[builder(into)]
    #[serde(rename = "elasticInstanceMinimum")]
    pub r#elastic_instance_minimum: Option<i32>,
    /// State of FTP / FTPS service for this function app. Possible values include: `AllAllowed`, `FtpsOnly` and `Disabled`.
    #[builder(into)]
    #[serde(rename = "ftpsState")]
    pub r#ftps_state: Option<String>,
    /// Path which will be checked for this function app health.
    #[builder(into)]
    #[serde(rename = "healthCheckPath")]
    pub r#health_check_path: Option<String>,
    /// Specifies whether or not the HTTP2 protocol should be enabled. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "http2Enabled")]
    pub r#http_2_enabled: Option<bool>,
    /// A list of `ip_restriction` objects representing IP restrictions as defined below.
    #[builder(into)]
    #[serde(rename = "ipRestrictions")]
    pub r#ip_restrictions: Option<Vec<super::super::types::appservice::FunctionAppSlotSiteConfigIpRestriction>>,
    /// Java version hosted by the function app in Azure. Possible values are `1.8`, `11` & `17` (In-Preview).
    #[builder(into)]
    #[serde(rename = "javaVersion")]
    pub r#java_version: Option<String>,
    /// Linux App Framework and version for the AppService, e.g. `DOCKER|(golang:latest)`.
    #[builder(into)]
    #[serde(rename = "linuxFxVersion")]
    pub r#linux_fx_version: Option<String>,
    /// The minimum supported TLS version for the function app. Possible values are `1.0`, `1.1`, and `1.2`. Defaults to `1.2` for new function apps.
    #[builder(into)]
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Option<String>,
    /// The number of pre-warmed instances for this function app. Only affects apps on the Premium plan.
    #[builder(into)]
    #[serde(rename = "preWarmedInstanceCount")]
    pub r#pre_warmed_instance_count: Option<i32>,
    /// Should Runtime Scale Monitoring be enabled?. Only applicable to apps on the Premium plan. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "runtimeScaleMonitoringEnabled")]
    pub r#runtime_scale_monitoring_enabled: Option<bool>,
    /// A list of `scm_ip_restriction` objects representing IP restrictions as defined below.
    /// 
    /// > **NOTE** User has to explicitly set `scm_ip_restriction` to empty slice (`[]`) to remove it.
    #[builder(into)]
    #[serde(rename = "scmIpRestrictions")]
    pub r#scm_ip_restrictions: Option<Vec<super::super::types::appservice::FunctionAppSlotSiteConfigScmIpRestriction>>,
    /// The type of Source Control used by this function App. Valid values include: `BitBucketGit`, `BitBucketHg`, `CodePlexGit`, `CodePlexHg`, `Dropbox`, `ExternalGit`, `ExternalHg`, `GitHub`, `LocalGit`, `None` (default), `OneDrive`, `Tfs`, `VSO`, and `VSTSRM`.
    /// 
    /// > **NOTE:** This setting is incompatible with the `source_control` block which updates this value based on the setting provided.
    #[builder(into)]
    #[serde(rename = "scmType")]
    pub r#scm_type: Option<String>,
    /// IP security restrictions for scm to use main. Defaults to `false`.
    /// 
    /// > **NOTE** Any `scm_ip_restriction` blocks configured are ignored by the service when `scm_use_main_ip_restriction` is set to `true`. Any scm restrictions will become active if this is subsequently set to `false` or removed.
    #[builder(into)]
    #[serde(rename = "scmUseMainIpRestriction")]
    pub r#scm_use_main_ip_restriction: Option<bool>,
    /// Should the Function App run in 32 bit mode, rather than 64 bit mode? Defaults to `true`.
    /// 
    /// > **Note:** when using an App Service Plan in the `Free` or `Shared` Tiers `use_32_bit_worker_process` must be set to `true`.
    #[builder(into)]
    #[serde(rename = "use32BitWorkerProcess")]
    pub r#use_32_bit_worker_process: Option<bool>,
    #[builder(into)]
    #[serde(rename = "vnetRouteAllEnabled")]
    pub r#vnet_route_all_enabled: Option<bool>,
    /// Should WebSockets be enabled?
    #[builder(into)]
    #[serde(rename = "websocketsEnabled")]
    pub r#websockets_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FunctionAppSlotSiteConfig {
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
                    "alwaysOn",
                    &self.r#always_on,
                ),
                to_pulumi_object_field(
                    "appScaleLimit",
                    &self.r#app_scale_limit,
                ),
                to_pulumi_object_field(
                    "autoSwapSlotName",
                    &self.r#auto_swap_slot_name,
                ),
                to_pulumi_object_field(
                    "cors",
                    &self.r#cors,
                ),
                to_pulumi_object_field(
                    "dotnetFrameworkVersion",
                    &self.r#dotnet_framework_version,
                ),
                to_pulumi_object_field(
                    "elasticInstanceMinimum",
                    &self.r#elastic_instance_minimum,
                ),
                to_pulumi_object_field(
                    "ftpsState",
                    &self.r#ftps_state,
                ),
                to_pulumi_object_field(
                    "healthCheckPath",
                    &self.r#health_check_path,
                ),
                to_pulumi_object_field(
                    "http2Enabled",
                    &self.r#http_2_enabled,
                ),
                to_pulumi_object_field(
                    "ipRestrictions",
                    &self.r#ip_restrictions,
                ),
                to_pulumi_object_field(
                    "javaVersion",
                    &self.r#java_version,
                ),
                to_pulumi_object_field(
                    "linuxFxVersion",
                    &self.r#linux_fx_version,
                ),
                to_pulumi_object_field(
                    "minTlsVersion",
                    &self.r#min_tls_version,
                ),
                to_pulumi_object_field(
                    "preWarmedInstanceCount",
                    &self.r#pre_warmed_instance_count,
                ),
                to_pulumi_object_field(
                    "runtimeScaleMonitoringEnabled",
                    &self.r#runtime_scale_monitoring_enabled,
                ),
                to_pulumi_object_field(
                    "scmIpRestrictions",
                    &self.r#scm_ip_restrictions,
                ),
                to_pulumi_object_field(
                    "scmType",
                    &self.r#scm_type,
                ),
                to_pulumi_object_field(
                    "scmUseMainIpRestriction",
                    &self.r#scm_use_main_ip_restriction,
                ),
                to_pulumi_object_field(
                    "use32BitWorkerProcess",
                    &self.r#use_32_bit_worker_process,
                ),
                to_pulumi_object_field(
                    "vnetRouteAllEnabled",
                    &self.r#vnet_route_all_enabled,
                ),
                to_pulumi_object_field(
                    "websocketsEnabled",
                    &self.r#websockets_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FunctionAppSlotSiteConfig {
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
                    r#always_on: {
                        let field_value = match fields_map.get("alwaysOn") {
                            Some(value) => value,
                            None => bail!("Missing field 'alwaysOn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#app_scale_limit: {
                        let field_value = match fields_map.get("appScaleLimit") {
                            Some(value) => value,
                            None => bail!("Missing field 'appScaleLimit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auto_swap_slot_name: {
                        let field_value = match fields_map.get("autoSwapSlotName") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoSwapSlotName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cors: {
                        let field_value = match fields_map.get("cors") {
                            Some(value) => value,
                            None => bail!("Missing field 'cors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dotnet_framework_version: {
                        let field_value = match fields_map.get("dotnetFrameworkVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'dotnetFrameworkVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#elastic_instance_minimum: {
                        let field_value = match fields_map.get("elasticInstanceMinimum") {
                            Some(value) => value,
                            None => bail!("Missing field 'elasticInstanceMinimum' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ftps_state: {
                        let field_value = match fields_map.get("ftpsState") {
                            Some(value) => value,
                            None => bail!("Missing field 'ftpsState' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#health_check_path: {
                        let field_value = match fields_map.get("healthCheckPath") {
                            Some(value) => value,
                            None => bail!("Missing field 'healthCheckPath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_2_enabled: {
                        let field_value = match fields_map.get("http2Enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'http2Enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_restrictions: {
                        let field_value = match fields_map.get("ipRestrictions") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipRestrictions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#linux_fx_version: {
                        let field_value = match fields_map.get("linuxFxVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'linuxFxVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_tls_version: {
                        let field_value = match fields_map.get("minTlsVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'minTlsVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pre_warmed_instance_count: {
                        let field_value = match fields_map.get("preWarmedInstanceCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'preWarmedInstanceCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#runtime_scale_monitoring_enabled: {
                        let field_value = match fields_map.get("runtimeScaleMonitoringEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'runtimeScaleMonitoringEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scm_ip_restrictions: {
                        let field_value = match fields_map.get("scmIpRestrictions") {
                            Some(value) => value,
                            None => bail!("Missing field 'scmIpRestrictions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scm_type: {
                        let field_value = match fields_map.get("scmType") {
                            Some(value) => value,
                            None => bail!("Missing field 'scmType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scm_use_main_ip_restriction: {
                        let field_value = match fields_map.get("scmUseMainIpRestriction") {
                            Some(value) => value,
                            None => bail!("Missing field 'scmUseMainIpRestriction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_32_bit_worker_process: {
                        let field_value = match fields_map.get("use32BitWorkerProcess") {
                            Some(value) => value,
                            None => bail!("Missing field 'use32BitWorkerProcess' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vnet_route_all_enabled: {
                        let field_value = match fields_map.get("vnetRouteAllEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'vnetRouteAllEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#websockets_enabled: {
                        let field_value = match fields_map.get("websocketsEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'websocketsEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
