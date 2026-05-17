#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLaunchTemplatePrivateDnsNameOption {
    #[builder(into)]
    #[serde(rename = "enableResourceNameDnsARecord")]
    pub r#enable_resource_name_dns_a_record: bool,
    #[builder(into)]
    #[serde(rename = "enableResourceNameDnsAaaaRecord")]
    pub r#enable_resource_name_dns_aaaa_record: bool,
    #[builder(into)]
    #[serde(rename = "hostnameType")]
    pub r#hostname_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLaunchTemplatePrivateDnsNameOption {
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
                "enable_resource_name_dns_a_record".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_resource_name_dns_a_record,
                )
                .await,
            );
            map.insert(
                "enable_resource_name_dns_aaaa_record".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_resource_name_dns_aaaa_record,
                )
                .await,
            );
            map.insert(
                "hostname_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hostname_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLaunchTemplatePrivateDnsNameOption {
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
                    r#enable_resource_name_dns_a_record: {
                        let field_value = match fields_map.get("enable_resource_name_dns_a_record") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_resource_name_dns_a_record' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_resource_name_dns_aaaa_record: {
                        let field_value = match fields_map.get("enable_resource_name_dns_aaaa_record") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_resource_name_dns_aaaa_record' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hostname_type: {
                        let field_value = match fields_map.get("hostname_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostname_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
