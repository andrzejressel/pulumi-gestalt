#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ManagedZoneDnssecConfig {
    /// Specifies parameters that will be used for generating initial DnsKeys
    /// for this ManagedZone. If you provide a spec for keySigning or zoneSigning,
    /// you must also provide one for the other.
    /// default_key_specs can only be updated when the state is `off`.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "defaultKeySpecs")]
    pub r#default_key_specs: Option<Vec<super::super::types::dns::ManagedZoneDnssecConfigDefaultKeySpec>>,
    /// Identifies what kind of resource this is
    #[builder(into)]
    #[serde(rename = "kind")]
    pub r#kind: Option<String>,
    /// Specifies the mechanism used to provide authenticated denial-of-existence responses.
    /// non_existence can only be updated when the state is `off`.
    /// Possible values are: `nsec`, `nsec3`.
    #[builder(into)]
    #[serde(rename = "nonExistence")]
    pub r#non_existence: Option<String>,
    /// Specifies whether DNSSEC is enabled, and what mode it is in
    /// Possible values are: `off`, `on`, `transfer`.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ManagedZoneDnssecConfig {
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
                    "default_key_specs",
                    &self.r#default_key_specs,
                ),
                to_pulumi_object_field(
                    "kind",
                    &self.r#kind,
                ),
                to_pulumi_object_field(
                    "non_existence",
                    &self.r#non_existence,
                ),
                to_pulumi_object_field(
                    "state",
                    &self.r#state,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ManagedZoneDnssecConfig {
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
                    r#default_key_specs: {
                        let field_value = match fields_map.get("default_key_specs") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_key_specs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kind: {
                        let field_value = match fields_map.get("kind") {
                            Some(value) => value,
                            None => bail!("Missing field 'kind' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#non_existence: {
                        let field_value = match fields_map.get("non_existence") {
                            Some(value) => value,
                            None => bail!("Missing field 'non_existence' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
