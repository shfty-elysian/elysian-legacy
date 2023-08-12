use std::error::Error;

use elysian_ir::module::Module as ElysianModule;
use elysian_naga::NagaBuilder;
use naga::{
    back::glsl::{Options, PipelineOptions, WriterFlags},
    proc::{BoundsCheckPolicies, BoundsCheckPolicy},
    valid::{Capabilities, ModuleInfo, ValidationFlags},
    Module as NagaModule, ShaderStage,
};

pub fn module_to_shadertoy(module: &ElysianModule) -> Result<String, Box<dyn Error>> {
    let (naga_module, module_info) =
        NagaBuilder::new(module).build(ValidationFlags::all(), Capabilities::default())?;
    Ok(naga_to_shadertoy(&naga_module, &module_info)?)
}

pub fn naga_to_shadertoy(
    naga_module: &NagaModule,
    module_info: &ModuleInfo,
) -> Result<String, naga::back::glsl::Error> {
    let mut buf = String::default();

    let options = Options {
        version: naga::back::glsl::Version::Embedded {
            version: 310,
            is_webgl: true,
        },
        writer_flags: WriterFlags::INCLUDE_UNUSED_ITEMS,
        binding_map: Default::default(),
        zero_initialize_workgroup_memory: false,
    };

    let pipeline_options = PipelineOptions {
        shader_stage: ShaderStage::Fragment,
        entry_point: "mainImage".to_string(),
        multiview: None,
    };

    let mut writer = naga::back::glsl::Writer::new(
        &mut buf,
        &naga_module,
        &module_info,
        &options,
        &pipeline_options,
        BoundsCheckPolicies {
            index: BoundsCheckPolicy::Unchecked,
            buffer: BoundsCheckPolicy::Unchecked,
            image_load: BoundsCheckPolicy::Unchecked,
            image_store: BoundsCheckPolicy::Unchecked,
            binding_array: BoundsCheckPolicy::Unchecked,
        },
    )?;

    writer.write()?;

    // Strip Shadertoy-incompatible GLSL, inline duplicated variables
    let mut buf: Vec<_> = buf
        .lines()
        .flat_map(|line| {
            if line.starts_with("precision")
                | line.starts_with("#version")
                | line.starts_with("layout")
            {
                None
            } else if line.contains("fragColor_1") {
                Some(line.replace("fragColor_1", "fragColor"))
            } else {
                Some(line.to_string())
            }
        })
        .collect();

    // Modify main function
    let mut iter = buf
        .iter_mut()
        .skip_while(|line| !line.starts_with("void main"));

    // Replace main signature
    let main = iter.next().unwrap();
    *main = "void mainImage(out vec4 fragColor, in vec2 fragCoord) {".to_string();

    while let Some(line) = iter.next() {
        if line.contains("Context") {
            break;
        } else {
            *line = String::default();
        }
    }

    // Concatenate
    Ok(buf
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line + "\n")
        .collect())
}
