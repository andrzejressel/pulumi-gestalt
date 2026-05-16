#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualNodeSpecServiceDiscoveryAwsCloudMap {
    /// String map that contains attributes with values that you can use to filter instances by any custom attribute that you specified when you registered the instance. Only instances that match all of the specified key/value pairs will be returned.
    #[builder(into)]
    #[serde(rename = "attributes")]
    pub r#attributes: Option<std::collections::HashMap<String, String>>,
    /// Name of the AWS Cloud Map namespace to use.
    /// Use the `aws.servicediscovery.HttpNamespace` resource to configure a Cloud Map namespace. Must be between 1 and 1024 characters in length.
    #[builder(into)]
    #[serde(rename = "namespaceName")]
    pub r#namespace_name: String,
    /// Name of the AWS Cloud Map service to use. Use the `aws.servicediscovery.Service` resource to configure a Cloud Map service. Must be between 1 and 1024 characters in length.
    #[builder(into)]
    #[serde(rename = "serviceName")]
    pub r#service_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualNodeSpecServiceDiscoveryAwsCloudMap {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("attributes".to_string(), self.r#attributes.to_pulumi_value().await);
            map.insert("namespace_name".to_string(), self.r#namespace_name.to_pulumi_value().await);
            map.insert("service_name".to_string(), self.r#service_name.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualNodeSpecServiceDiscoveryAwsCloudMap {
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
                    r#attributes: {
                        let field_value = match fields_map.get("attributes") {
                            Some(value) => value,
                            None => bail!("Missing field 'attributes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<std::collections::HashMap<String, String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#namespace_name: {
                        let field_value = match fields_map.get("namespace_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'namespace_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#service_name: {
                        let field_value = match fields_map.get("service_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
