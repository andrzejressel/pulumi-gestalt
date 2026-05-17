#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FhirServiceOciArtifact {
    /// A digest of an image within Azure container registry used for export operations of the service instance to narrow the artifacts down.
    #[builder(into)]
    #[serde(rename = "digest")]
    pub r#digest: Option<String>,
    /// An image within Azure container registry used for export operations of the service instance.
    #[builder(into)]
    #[serde(rename = "imageName")]
    pub r#image_name: Option<String>,
    /// An Azure container registry used for export operations of the service instance.
    #[builder(into)]
    #[serde(rename = "loginServer")]
    pub r#login_server: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FhirServiceOciArtifact {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "digest".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#digest,
                )
                .await,
            );
            map.insert(
                "image_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image_name,
                )
                .await,
            );
            map.insert(
                "login_server".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#login_server,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FhirServiceOciArtifact {
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
                    r#digest: {
                        let field_value = match fields_map.get("digest") {
                            Some(value) => value,
                            None => bail!("Missing field 'digest' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_name: {
                        let field_value = match fields_map.get("image_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#login_server: {
                        let field_value = match fields_map.get("login_server") {
                            Some(value) => value,
                            None => bail!("Missing field 'login_server' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
