#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetStandardSiteConfig {
    /// Should the Logic App be loaded at all times?
    #[builder(into)]
    pub r#always_on: Option<bool>,
    /// The number of workers this Logic App can scale out to. Only applicable to apps on the Consumption and Premium plan.
    #[builder(into)]
    pub r#app_scale_limit: i32,
    /// The Auto-swap slot name.
    #[builder(into)]
    pub r#auto_swap_slot_name: String,
    /// A `cors` block as defined below.
    #[builder(into)]
    pub r#cors: Box<super::super::types::logicapps::GetStandardSiteConfigCors>,
    /// The version of the .NET framework's CLR used in this Logic App.
    #[builder(into)]
    pub r#dotnet_framework_version: Option<String>,
    /// The number of minimum instances for this Logic App Only affects apps on the Premium plan.
    #[builder(into)]
    pub r#elastic_instance_minimum: i32,
    /// The state of FTP / FTPS service for this Logic App.
    #[builder(into)]
    pub r#ftps_state: String,
    /// Path which will be checked for this Logic App health.
    #[builder(into)]
    pub r#health_check_path: Option<String>,
    /// Specifies whether the HTTP2 protocol should be enabled.
    #[builder(into)]
    pub r#http_2_enabled: Option<bool>,
    /// A list of `ip_restriction` objects representing IP restrictions as defined below.
    #[builder(into)]
    pub r#ip_restrictions: Vec<super::super::types::logicapps::GetStandardSiteConfigIpRestriction>,
    /// Linux App Framework and version for the Logic App.
    #[builder(into)]
    pub r#linux_fx_version: String,
    /// The minimum supported TLS version for the Logic App.
    #[builder(into)]
    pub r#min_tls_version: String,
    /// The number of pre-warmed instances for this Logic App Only affects apps on the Premium plan.
    #[builder(into)]
    pub r#pre_warmed_instance_count: i32,
    #[builder(into)]
    pub r#public_network_access_enabled: bool,
    /// Should Runtime Scale Monitoring be enabled?. Only applicable to apps on the Premium plan.
    #[builder(into)]
    pub r#runtime_scale_monitoring_enabled: Option<bool>,
    /// A list of `scm_ip_restriction` objects representing SCM IP restrictions as defined below.
    #[builder(into)]
    pub r#scm_ip_restrictions: Vec<super::super::types::logicapps::GetStandardSiteConfigScmIpRestriction>,
    /// The minimum version of TLS required for SSL requests to the SCM site.
    #[builder(into)]
    pub r#scm_min_tls_version: String,
    /// The type of Source Control used by the Logic App in use by the Windows Function App.
    #[builder(into)]
    pub r#scm_type: String,
    /// Should the Logic App `ip_restriction` configuration be used for the SCM too.
    #[builder(into)]
    pub r#scm_use_main_ip_restriction: Option<bool>,
    /// Should the Logic App run in 32 bit mode, rather than 64 bit mode?
    #[builder(into)]
    pub r#use_32_bit_worker_process: Option<bool>,
    /// Should all outbound traffic to have Virtual Network Security Groups and User Defined Routes applied.
    #[builder(into)]
    pub r#vnet_route_all_enabled: bool,
    /// Should WebSockets be enabled?
    #[builder(into)]
    pub r#websockets_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetStandardSiteConfig {
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
                    "publicNetworkAccessEnabled",
                    &self.r#public_network_access_enabled,
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
                    "scmMinTlsVersion",
                    &self.r#scm_min_tls_version,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetStandardSiteConfig {
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
                    r#public_network_access_enabled: {
                        let field_value = match fields_map.get("publicNetworkAccessEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicNetworkAccessEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#scm_min_tls_version: {
                        let field_value = match fields_map.get("scmMinTlsVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'scmMinTlsVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
