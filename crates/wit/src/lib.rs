#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
#[cfg(feature = "client")]
pub mod client_bindings {
    wit_bindgen::generate!({
        world: "pulumi-gestalt",
        generate_all
    });
}

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
#[cfg(feature = "runner")]
pub mod bindings_runner {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct SingleThreadedContext {
        pub engine: Rc<RefCell<pulumi_gestalt_core::Engine>>,
        pub project_name: String,
    }

    impl SingleThreadedContext {
        pub fn new(engine: pulumi_gestalt_core::Engine, project_name: String) -> Self {
            Self {
                engine: Rc::new(RefCell::new(engine)),
                project_name,
            }
        }
    }
    unsafe impl Send for SingleThreadedContext {}

    pub struct SingleThreadedOutput {
        pub output_id: pulumi_gestalt_core::OutputId,
        pub engine: Rc<RefCell<pulumi_gestalt_core::Engine>>,
    }
    unsafe impl Send for SingleThreadedOutput {}
    impl SingleThreadedOutput {
        pub fn new(
            output_id: pulumi_gestalt_core::OutputId,
            engine: Rc<RefCell<pulumi_gestalt_core::Engine>>,
        ) -> Self {
            Self { output_id, engine }
        }
    }

    pub struct SingleThreadedCompositeOutput {
        pub output_id: pulumi_gestalt_core::OutputId,
        pub engine: Rc<RefCell<pulumi_gestalt_core::Engine>>,
    }
    unsafe impl Send for SingleThreadedCompositeOutput {}
    impl SingleThreadedCompositeOutput {
        pub fn new(
            output_id: pulumi_gestalt_core::OutputId,
            engine: Rc<RefCell<pulumi_gestalt_core::Engine>>,
        ) -> Self {
            Self { output_id, engine }
        }
    }

    wasmtime::component::bindgen!({
        world: "pulumi-gestalt",
        imports: {
            default: async | trappable
            // "component": trappable
        },
        with: {
            "component:pulumi-gestalt/context/context":SingleThreadedContext,
            "component:pulumi-gestalt/output-interface/output": SingleThreadedOutput,
            "component:pulumi-gestalt/output-interface/composite-output": SingleThreadedCompositeOutput,
        }
    });
}
