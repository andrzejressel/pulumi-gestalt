#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualNetworkAllowedSubnet {
    /// Indicates if this subnet allows public IP addresses. Possible values are `Allow`, `Default` and `Deny`.
    #[builder(into)]
    #[serde(rename = "allowPublicIp")]
    pub r#allow_public_ip: String,
    /// The name of the subnet.
    #[builder(into)]
    #[serde(rename = "labSubnetName")]
    pub r#lab_subnet_name: String,
    /// The resource identifier for the subnet.
    #[builder(into)]
    #[serde(rename = "resourceId")]
    pub r#resource_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVirtualNetworkAllowedSubnet {
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
                "allow_public_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_public_ip,
                )
                .await,
            );
            map.insert(
                "lab_subnet_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lab_subnet_name,
                )
                .await,
            );
            map.insert(
                "resource_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVirtualNetworkAllowedSubnet {
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
                    r#allow_public_ip: {
                        let field_value = match fields_map.get("allow_public_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_public_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lab_subnet_name: {
                        let field_value = match fields_map.get("lab_subnet_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'lab_subnet_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_id: {
                        let field_value = match fields_map.get("resource_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
