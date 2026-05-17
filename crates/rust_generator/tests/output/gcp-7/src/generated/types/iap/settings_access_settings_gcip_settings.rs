#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SettingsAccessSettingsGcipSettings {
    /// Login page URI associated with the GCIP tenants. Typically, all resources within
    /// the same project share the same login page, though it could be overridden at the
    /// sub resource level.
    #[builder(into)]
    #[serde(rename = "loginPageUri")]
    pub r#login_page_uri: Option<String>,
    /// GCIP tenant ids that are linked to the IAP resource. tenantIds could be a string
    /// beginning with a number character to indicate authenticating with GCIP tenant flow,
    /// or in the format of _ to indicate authenticating with GCIP agent flow. If agent flow
    /// is used, tenantIds should only contain one single element, while for tenant flow,
    /// tenantIds can contain multiple elements.
    #[builder(into)]
    #[serde(rename = "tenantIds")]
    pub r#tenant_ids: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SettingsAccessSettingsGcipSettings {
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
                "login_page_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#login_page_uri,
                )
                .await,
            );
            map.insert(
                "tenant_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tenant_ids,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SettingsAccessSettingsGcipSettings {
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
                    r#login_page_uri: {
                        let field_value = match fields_map.get("login_page_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'login_page_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tenant_ids: {
                        let field_value = match fields_map.get("tenant_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'tenant_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
