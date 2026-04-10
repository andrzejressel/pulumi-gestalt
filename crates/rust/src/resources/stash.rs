use crate::{
    Context, CustomResourceOptions, InputOrOutput, Output, PulumiAny, RegisterResourceRequest,
    ResourceRequestObjectField,
};
use bon::Builder;

/// Input arguments used to create a [`Stash`] resource.
#[derive(Builder)]
#[builder(finish_fn = build_struct)]
pub struct StashArgs {
    /// The value to store in stash state.
    #[builder(into)]
    pub input: InputOrOutput<PulumiAny>,
}

/// Output object returned from stash creation.
pub struct StashResult {
    /// Pulumi ID is the provider-assigned unique ID for this managed resource.
    /// It is set during deployments and may be missing (unknown) during planning phases.
    pub id: Output<String>,
    /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
    pub urn: Output<String>,
    /// The most recent value passed to the stash resource.
    pub input: Output<PulumiAny>,
    /// The value saved in state for the stash.
    pub output: Output<PulumiAny>,
}

/// Registers a new stash resource with the given unique name and arguments.
pub fn create(context: &Context, name: &str, args: StashArgs) -> StashResult {
    __create(context, name, args, None)
}

/// Same as [`create`], but with additional generic options that control registration behavior.
pub fn create_with_options(
    context: &Context,
    name: &str,
    args: StashArgs,
    options: CustomResourceOptions,
) -> StashResult {
    __create(context, name, args, Some(options))
}

fn __create(
    context: &Context,
    name: &str,
    args: StashArgs,
    options: Option<CustomResourceOptions>,
) -> StashResult {
    let input_binding = args.input.get_output(context);
    let request = RegisterResourceRequest {
        type_: "pulumi:index:Stash".into(),
        name: name.to_string(),
        version: String::new(),
        object: &[ResourceRequestObjectField {
            name: "input".into(),
            value: &input_binding.drop_type(),
        }],
        options,
    };
    let composite = context.register_resource(request);

    StashResult {
        id: composite.get_id(),
        urn: composite.get_urn(),
        input: composite.get_field("input"),
        output: composite.get_field("output"),
    }
}
