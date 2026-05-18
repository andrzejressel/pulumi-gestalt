#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpringCloudServiceNetwork {
    /// Specifies the Name of the resource group containing network resources of Azure Spring Cloud Apps. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "appNetworkResourceGroup")]
    pub r#app_network_resource_group: Option<String>,
    /// Specifies the ID of the Subnet which should host the Spring Boot Applications deployed in this Spring Cloud Service. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "appSubnetId")]
    pub r#app_subnet_id: String,
    /// A list of (at least 3) CIDR ranges (at least /16) which are used to host the Spring Cloud infrastructure, which must not overlap with any existing CIDR ranges in the Subnet. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "cidrRanges")]
    pub r#cidr_ranges: Vec<String>,
    /// Specifies the egress traffic type of the Spring Cloud Service. Possible values are `loadBalancer` and `userDefinedRouting`. Defaults to `loadBalancer`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "outboundType")]
    pub r#outbound_type: Option<String>,
    /// Ingress read time out in seconds.
    #[builder(into)]
    #[serde(rename = "readTimeoutSeconds")]
    pub r#read_timeout_seconds: Option<i32>,
    /// Specifies the Name of the resource group containing network resources of Azure Spring Cloud Service Runtime. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "serviceRuntimeNetworkResourceGroup")]
    pub r#service_runtime_network_resource_group: Option<String>,
    /// Specifies the ID of the Subnet where the Service Runtime components of the Spring Cloud Service will exist. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "serviceRuntimeSubnetId")]
    pub r#service_runtime_subnet_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpringCloudServiceNetwork {
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
                    "app_network_resource_group",
                    &self.r#app_network_resource_group,
                ),
                to_pulumi_object_field(
                    "app_subnet_id",
                    &self.r#app_subnet_id,
                ),
                to_pulumi_object_field(
                    "cidr_ranges",
                    &self.r#cidr_ranges,
                ),
                to_pulumi_object_field(
                    "outbound_type",
                    &self.r#outbound_type,
                ),
                to_pulumi_object_field(
                    "read_timeout_seconds",
                    &self.r#read_timeout_seconds,
                ),
                to_pulumi_object_field(
                    "service_runtime_network_resource_group",
                    &self.r#service_runtime_network_resource_group,
                ),
                to_pulumi_object_field(
                    "service_runtime_subnet_id",
                    &self.r#service_runtime_subnet_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SpringCloudServiceNetwork {
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
                    r#app_network_resource_group: {
                        let field_value = match fields_map.get("app_network_resource_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_network_resource_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#app_subnet_id: {
                        let field_value = match fields_map.get("app_subnet_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_subnet_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cidr_ranges: {
                        let field_value = match fields_map.get("cidr_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'cidr_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#outbound_type: {
                        let field_value = match fields_map.get("outbound_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'outbound_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#read_timeout_seconds: {
                        let field_value = match fields_map.get("read_timeout_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_timeout_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_runtime_network_resource_group: {
                        let field_value = match fields_map.get("service_runtime_network_resource_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_runtime_network_resource_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_runtime_subnet_id: {
                        let field_value = match fields_map.get("service_runtime_subnet_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_runtime_subnet_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
