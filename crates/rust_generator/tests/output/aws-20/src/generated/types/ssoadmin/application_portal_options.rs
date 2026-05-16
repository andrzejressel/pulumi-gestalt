#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationPortalOptions {
    /// Sign-in options for the access portal. See `sign_in_options` below.
    #[builder(into)]
    #[serde(rename = "signInOptions")]
    pub r#sign_in_options: Option<Box<super::super::types::ssoadmin::ApplicationPortalOptionsSignInOptions>>,
    /// Indicates whether this application is visible in the access portal. Valid values are `ENABLED` and `DISABLED`.
    #[builder(into)]
    #[serde(rename = "visibility")]
    pub r#visibility: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationPortalOptions {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("sign_in_options".to_string(), self.r#sign_in_options.to_pulumi_value().await);
            map.insert("visibility".to_string(), self.r#visibility.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApplicationPortalOptions {
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
                    r#sign_in_options: {
                        let field_value = match fields_map.get("sign_in_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'sign_in_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::ssoadmin::ApplicationPortalOptionsSignInOptions>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#visibility: {
                        let field_value = match fields_map.get("visibility") {
                            Some(value) => value,
                            None => bail!("Missing field 'visibility' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
