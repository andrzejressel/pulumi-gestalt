#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceReplicaSet {
    /// A list of subnet IP addresses for the domain controllers in the replica set, typically two.
    #[builder(into)]
    #[serde(rename = "domainControllerIpAddresses")]
    pub r#domain_controller_ip_addresses: Vec<String>,
    /// The publicly routable IP address for the domain controllers in the replica set.
    #[builder(into)]
    #[serde(rename = "externalAccessIpAddress")]
    pub r#external_access_ip_address: String,
    /// The ID of the Domain Service.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The Azure location in which the replica set resides.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
    /// The current service status for the replica set.
    #[builder(into)]
    #[serde(rename = "serviceStatus")]
    pub r#service_status: String,
    /// The ID of the subnet in which the replica set resides.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetServiceReplicaSet {
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
                    "domain_controller_ip_addresses",
                    &self.r#domain_controller_ip_addresses,
                ),
                to_pulumi_object_field(
                    "external_access_ip_address",
                    &self.r#external_access_ip_address,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "location",
                    &self.r#location,
                ),
                to_pulumi_object_field(
                    "service_status",
                    &self.r#service_status,
                ),
                to_pulumi_object_field(
                    "subnet_id",
                    &self.r#subnet_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetServiceReplicaSet {
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
                    r#domain_controller_ip_addresses: {
                        let field_value = match fields_map.get("domain_controller_ip_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_controller_ip_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#external_access_ip_address: {
                        let field_value = match fields_map.get("external_access_ip_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'external_access_ip_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#location: {
                        let field_value = match fields_map.get("location") {
                            Some(value) => value,
                            None => bail!("Missing field 'location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_status: {
                        let field_value = match fields_map.get("service_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_id: {
                        let field_value = match fields_map.get("subnet_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
