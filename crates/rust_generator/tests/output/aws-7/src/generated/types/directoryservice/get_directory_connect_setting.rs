#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDirectoryConnectSetting {
    #[builder(into)]
    #[serde(rename = "availabilityZones")]
    pub r#availability_zones: Vec<String>,
    /// IP addresses of the AD Connector servers.
    #[builder(into)]
    #[serde(rename = "connectIps")]
    pub r#connect_ips: Vec<String>,
    /// DNS IP addresses of the domain to connect to.
    #[builder(into)]
    #[serde(rename = "customerDnsIps")]
    pub r#customer_dns_ips: Vec<String>,
    /// Username corresponding to the password provided.
    #[builder(into)]
    #[serde(rename = "customerUsername")]
    pub r#customer_username: String,
    /// Identifiers of the subnets for the connector servers (2 subnets in 2 different AZs).
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Vec<String>,
    /// ID of the VPC that the connector is in.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDirectoryConnectSetting {
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
                    "availability_zones",
                    &self.r#availability_zones,
                ),
                to_pulumi_object_field(
                    "connect_ips",
                    &self.r#connect_ips,
                ),
                to_pulumi_object_field(
                    "customer_dns_ips",
                    &self.r#customer_dns_ips,
                ),
                to_pulumi_object_field(
                    "customer_username",
                    &self.r#customer_username,
                ),
                to_pulumi_object_field(
                    "subnet_ids",
                    &self.r#subnet_ids,
                ),
                to_pulumi_object_field(
                    "vpc_id",
                    &self.r#vpc_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDirectoryConnectSetting {
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
                    r#availability_zones: {
                        let field_value = match fields_map.get("availability_zones") {
                            Some(value) => value,
                            None => bail!("Missing field 'availability_zones' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connect_ips: {
                        let field_value = match fields_map.get("connect_ips") {
                            Some(value) => value,
                            None => bail!("Missing field 'connect_ips' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#customer_dns_ips: {
                        let field_value = match fields_map.get("customer_dns_ips") {
                            Some(value) => value,
                            None => bail!("Missing field 'customer_dns_ips' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#customer_username: {
                        let field_value = match fields_map.get("customer_username") {
                            Some(value) => value,
                            None => bail!("Missing field 'customer_username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_ids: {
                        let field_value = match fields_map.get("subnet_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_id: {
                        let field_value = match fields_map.get("vpc_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
