#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RecordData {
    #[builder(into)]
    #[serde(rename = "algorithm")]
    pub r#algorithm: Option<i32>,
    #[builder(into)]
    #[serde(rename = "altitude")]
    pub r#altitude: Option<f64>,
    #[builder(into)]
    #[serde(rename = "certificate")]
    pub r#certificate: Option<String>,
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    #[builder(into)]
    #[serde(rename = "digest")]
    pub r#digest: Option<String>,
    #[builder(into)]
    #[serde(rename = "digestType")]
    pub r#digest_type: Option<i32>,
    #[builder(into)]
    #[serde(rename = "fingerprint")]
    pub r#fingerprint: Option<String>,
    #[builder(into)]
    #[serde(rename = "flags")]
    pub r#flags: Option<String>,
    #[builder(into)]
    #[serde(rename = "keyTag")]
    pub r#key_tag: Option<i32>,
    #[builder(into)]
    #[serde(rename = "latDegrees")]
    pub r#lat_degrees: Option<i32>,
    #[builder(into)]
    #[serde(rename = "latDirection")]
    pub r#lat_direction: Option<String>,
    #[builder(into)]
    #[serde(rename = "latMinutes")]
    pub r#lat_minutes: Option<i32>,
    #[builder(into)]
    #[serde(rename = "latSeconds")]
    pub r#lat_seconds: Option<f64>,
    #[builder(into)]
    #[serde(rename = "longDegrees")]
    pub r#long_degrees: Option<i32>,
    #[builder(into)]
    #[serde(rename = "longDirection")]
    pub r#long_direction: Option<String>,
    #[builder(into)]
    #[serde(rename = "longMinutes")]
    pub r#long_minutes: Option<i32>,
    #[builder(into)]
    #[serde(rename = "longSeconds")]
    pub r#long_seconds: Option<f64>,
    #[builder(into)]
    #[serde(rename = "matchingType")]
    pub r#matching_type: Option<i32>,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Option<i32>,
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    #[builder(into)]
    #[serde(rename = "precisionHorz")]
    pub r#precision_horz: Option<f64>,
    #[builder(into)]
    #[serde(rename = "precisionVert")]
    pub r#precision_vert: Option<f64>,
    #[builder(into)]
    #[serde(rename = "preference")]
    pub r#preference: Option<i32>,
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Option<i32>,
    #[builder(into)]
    #[serde(rename = "proto")]
    pub r#proto: Option<String>,
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<i32>,
    #[builder(into)]
    #[serde(rename = "publicKey")]
    pub r#public_key: Option<String>,
    #[builder(into)]
    #[serde(rename = "regex")]
    pub r#regex: Option<String>,
    #[builder(into)]
    #[serde(rename = "replacement")]
    pub r#replacement: Option<String>,
    #[builder(into)]
    #[serde(rename = "selector")]
    pub r#selector: Option<i32>,
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Option<String>,
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Option<f64>,
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Option<String>,
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Option<String>,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<i32>,
    #[builder(into)]
    #[serde(rename = "usage")]
    pub r#usage: Option<i32>,
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RecordData {
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
                    "algorithm",
                    &self.r#algorithm,
                ),
                to_pulumi_object_field(
                    "altitude",
                    &self.r#altitude,
                ),
                to_pulumi_object_field(
                    "certificate",
                    &self.r#certificate,
                ),
                to_pulumi_object_field(
                    "content",
                    &self.r#content,
                ),
                to_pulumi_object_field(
                    "digest",
                    &self.r#digest,
                ),
                to_pulumi_object_field(
                    "digest_type",
                    &self.r#digest_type,
                ),
                to_pulumi_object_field(
                    "fingerprint",
                    &self.r#fingerprint,
                ),
                to_pulumi_object_field(
                    "flags",
                    &self.r#flags,
                ),
                to_pulumi_object_field(
                    "key_tag",
                    &self.r#key_tag,
                ),
                to_pulumi_object_field(
                    "lat_degrees",
                    &self.r#lat_degrees,
                ),
                to_pulumi_object_field(
                    "lat_direction",
                    &self.r#lat_direction,
                ),
                to_pulumi_object_field(
                    "lat_minutes",
                    &self.r#lat_minutes,
                ),
                to_pulumi_object_field(
                    "lat_seconds",
                    &self.r#lat_seconds,
                ),
                to_pulumi_object_field(
                    "long_degrees",
                    &self.r#long_degrees,
                ),
                to_pulumi_object_field(
                    "long_direction",
                    &self.r#long_direction,
                ),
                to_pulumi_object_field(
                    "long_minutes",
                    &self.r#long_minutes,
                ),
                to_pulumi_object_field(
                    "long_seconds",
                    &self.r#long_seconds,
                ),
                to_pulumi_object_field(
                    "matching_type",
                    &self.r#matching_type,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "order",
                    &self.r#order,
                ),
                to_pulumi_object_field(
                    "port",
                    &self.r#port,
                ),
                to_pulumi_object_field(
                    "precision_horz",
                    &self.r#precision_horz,
                ),
                to_pulumi_object_field(
                    "precision_vert",
                    &self.r#precision_vert,
                ),
                to_pulumi_object_field(
                    "preference",
                    &self.r#preference,
                ),
                to_pulumi_object_field(
                    "priority",
                    &self.r#priority,
                ),
                to_pulumi_object_field(
                    "proto",
                    &self.r#proto,
                ),
                to_pulumi_object_field(
                    "protocol",
                    &self.r#protocol,
                ),
                to_pulumi_object_field(
                    "public_key",
                    &self.r#public_key,
                ),
                to_pulumi_object_field(
                    "regex",
                    &self.r#regex,
                ),
                to_pulumi_object_field(
                    "replacement",
                    &self.r#replacement,
                ),
                to_pulumi_object_field(
                    "selector",
                    &self.r#selector,
                ),
                to_pulumi_object_field(
                    "service",
                    &self.r#service,
                ),
                to_pulumi_object_field(
                    "size",
                    &self.r#size,
                ),
                to_pulumi_object_field(
                    "tag",
                    &self.r#tag,
                ),
                to_pulumi_object_field(
                    "target",
                    &self.r#target,
                ),
                to_pulumi_object_field(
                    "type_",
                    &self.r#type_,
                ),
                to_pulumi_object_field(
                    "usage",
                    &self.r#usage,
                ),
                to_pulumi_object_field(
                    "value",
                    &self.r#value,
                ),
                to_pulumi_object_field(
                    "weight",
                    &self.r#weight,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RecordData {
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
                    r#algorithm: {
                        let field_value = match fields_map.get("algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#altitude: {
                        let field_value = match fields_map.get("altitude") {
                            Some(value) => value,
                            None => bail!("Missing field 'altitude' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#certificate: {
                        let field_value = match fields_map.get("certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#content: {
                        let field_value = match fields_map.get("content") {
                            Some(value) => value,
                            None => bail!("Missing field 'content' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#digest: {
                        let field_value = match fields_map.get("digest") {
                            Some(value) => value,
                            None => bail!("Missing field 'digest' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#digest_type: {
                        let field_value = match fields_map.get("digest_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'digest_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fingerprint: {
                        let field_value = match fields_map.get("fingerprint") {
                            Some(value) => value,
                            None => bail!("Missing field 'fingerprint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#flags: {
                        let field_value = match fields_map.get("flags") {
                            Some(value) => value,
                            None => bail!("Missing field 'flags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_tag: {
                        let field_value = match fields_map.get("key_tag") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_tag' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lat_degrees: {
                        let field_value = match fields_map.get("lat_degrees") {
                            Some(value) => value,
                            None => bail!("Missing field 'lat_degrees' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lat_direction: {
                        let field_value = match fields_map.get("lat_direction") {
                            Some(value) => value,
                            None => bail!("Missing field 'lat_direction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lat_minutes: {
                        let field_value = match fields_map.get("lat_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'lat_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lat_seconds: {
                        let field_value = match fields_map.get("lat_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'lat_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#long_degrees: {
                        let field_value = match fields_map.get("long_degrees") {
                            Some(value) => value,
                            None => bail!("Missing field 'long_degrees' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#long_direction: {
                        let field_value = match fields_map.get("long_direction") {
                            Some(value) => value,
                            None => bail!("Missing field 'long_direction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#long_minutes: {
                        let field_value = match fields_map.get("long_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'long_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#long_seconds: {
                        let field_value = match fields_map.get("long_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'long_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#matching_type: {
                        let field_value = match fields_map.get("matching_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'matching_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#order: {
                        let field_value = match fields_map.get("order") {
                            Some(value) => value,
                            None => bail!("Missing field 'order' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port: {
                        let field_value = match fields_map.get("port") {
                            Some(value) => value,
                            None => bail!("Missing field 'port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#precision_horz: {
                        let field_value = match fields_map.get("precision_horz") {
                            Some(value) => value,
                            None => bail!("Missing field 'precision_horz' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#precision_vert: {
                        let field_value = match fields_map.get("precision_vert") {
                            Some(value) => value,
                            None => bail!("Missing field 'precision_vert' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preference: {
                        let field_value = match fields_map.get("preference") {
                            Some(value) => value,
                            None => bail!("Missing field 'preference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#priority: {
                        let field_value = match fields_map.get("priority") {
                            Some(value) => value,
                            None => bail!("Missing field 'priority' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#proto: {
                        let field_value = match fields_map.get("proto") {
                            Some(value) => value,
                            None => bail!("Missing field 'proto' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocol: {
                        let field_value = match fields_map.get("protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_key: {
                        let field_value = match fields_map.get("public_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#regex: {
                        let field_value = match fields_map.get("regex") {
                            Some(value) => value,
                            None => bail!("Missing field 'regex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replacement: {
                        let field_value = match fields_map.get("replacement") {
                            Some(value) => value,
                            None => bail!("Missing field 'replacement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#selector: {
                        let field_value = match fields_map.get("selector") {
                            Some(value) => value,
                            None => bail!("Missing field 'selector' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service: {
                        let field_value = match fields_map.get("service") {
                            Some(value) => value,
                            None => bail!("Missing field 'service' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#size: {
                        let field_value = match fields_map.get("size") {
                            Some(value) => value,
                            None => bail!("Missing field 'size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag: {
                        let field_value = match fields_map.get("tag") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target: {
                        let field_value = match fields_map.get("target") {
                            Some(value) => value,
                            None => bail!("Missing field 'target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#usage: {
                        let field_value = match fields_map.get("usage") {
                            Some(value) => value,
                            None => bail!("Missing field 'usage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#value: {
                        let field_value = match fields_map.get("value") {
                            Some(value) => value,
                            None => bail!("Missing field 'value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weight: {
                        let field_value = match fields_map.get("weight") {
                            Some(value) => value,
                            None => bail!("Missing field 'weight' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
