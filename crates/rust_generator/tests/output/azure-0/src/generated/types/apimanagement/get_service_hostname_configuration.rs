#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceHostnameConfiguration {
    /// One or more `developer_portal` blocks as documented below.
    #[builder(into)]
    #[serde(rename = "developerPortals")]
    pub r#developer_portals: Vec<super::super::types::apimanagement::GetServiceHostnameConfigurationDeveloperPortal>,
    /// One or more `management` blocks as documented below.
    #[builder(into)]
    #[serde(rename = "managements")]
    pub r#managements: Vec<super::super::types::apimanagement::GetServiceHostnameConfigurationManagement>,
    /// One or more `portal` blocks as documented below.
    #[builder(into)]
    #[serde(rename = "portals")]
    pub r#portals: Vec<super::super::types::apimanagement::GetServiceHostnameConfigurationPortal>,
    /// One or more `proxy` blocks as documented below.
    #[builder(into)]
    #[serde(rename = "proxies")]
    pub r#proxies: Vec<super::super::types::apimanagement::GetServiceHostnameConfigurationProxy>,
    /// One or more `scm` blocks as documented below.
    #[builder(into)]
    #[serde(rename = "scms")]
    pub r#scms: Vec<super::super::types::apimanagement::GetServiceHostnameConfigurationScm>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetServiceHostnameConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "developer_portals",
                    &self.r#developer_portals,
                ),
                to_pulumi_object_field(
                    "managements",
                    &self.r#managements,
                ),
                to_pulumi_object_field(
                    "portals",
                    &self.r#portals,
                ),
                to_pulumi_object_field(
                    "proxies",
                    &self.r#proxies,
                ),
                to_pulumi_object_field(
                    "scms",
                    &self.r#scms,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetServiceHostnameConfiguration {
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
                    r#developer_portals: {
                        let field_value = match fields_map.get("developer_portals") {
                            Some(value) => value,
                            None => bail!("Missing field 'developer_portals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managements: {
                        let field_value = match fields_map.get("managements") {
                            Some(value) => value,
                            None => bail!("Missing field 'managements' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#portals: {
                        let field_value = match fields_map.get("portals") {
                            Some(value) => value,
                            None => bail!("Missing field 'portals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#proxies: {
                        let field_value = match fields_map.get("proxies") {
                            Some(value) => value,
                            None => bail!("Missing field 'proxies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scms: {
                        let field_value = match fields_map.get("scms") {
                            Some(value) => value,
                            None => bail!("Missing field 'scms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
