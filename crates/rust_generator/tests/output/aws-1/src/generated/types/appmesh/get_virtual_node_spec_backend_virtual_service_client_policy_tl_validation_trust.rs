#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrust {
    #[builder(into)]
    #[serde(rename = "acms")]
    pub r#acms: Vec<super::super::types::appmesh::GetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustAcm>,
    #[builder(into)]
    #[serde(rename = "files")]
    pub r#files: Vec<super::super::types::appmesh::GetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustFile>,
    #[builder(into)]
    #[serde(rename = "sds")]
    pub r#sds: Vec<super::super::types::appmesh::GetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrustSd>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrust {
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
                "acms".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#acms,
                )
                .await,
            );
            map.insert(
                "files".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#files,
                )
                .await,
            );
            map.insert(
                "sds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sds,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVirtualNodeSpecBackendVirtualServiceClientPolicyTlValidationTrust {
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
                    r#acms: {
                        let field_value = match fields_map.get("acms") {
                            Some(value) => value,
                            None => bail!("Missing field 'acms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#files: {
                        let field_value = match fields_map.get("files") {
                            Some(value) => value,
                            None => bail!("Missing field 'files' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sds: {
                        let field_value = match fields_map.get("sds") {
                            Some(value) => value,
                            None => bail!("Missing field 'sds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
