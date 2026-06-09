use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    height: u8,
    #[arg(short, long, default_value = "scenes/sphere_pyramid.xml")]
    filename: PathBuf,
    #[arg(long = "width", default_value_t = 1920)]
    img_width: usize,
    #[arg(long = "height", default_value_t = 1080)]
    img_height: usize,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let file = File::create(&args.filename)?;
    let mut w = BufWriter::new(file);

    let distance = args.height as f32;

    let look_from_x = distance * -0.5;
    let look_from_y = distance * 2.0 / 3.0;
    let look_from_z = -distance;

    let look_at_xz = args.height as f32 / 2.0;
    let look_at_height = (args.height as f32 * 2f32.sqrt() / 2.0 + 0.5) / 3.0;

    let light_height = (args.height as f32 * 2f32.sqrt() / 2.0 + 0.5) / 1.5;

    let img_width = args.img_width;
    let img_height = args.img_height;

    write!(
        w,
        r#" <RT3>
    <lookat look_from="{look_from_x} {look_from_y} {look_from_z}" look_at="{look_at_xz} {look_at_height} {look_at_xz}" up="0 1 0" />
    <camera type="perspective" fovy="45" />
    <integrator type="blinn_phong" depth="3" />
    <film type="image" w_res="{img_width}" h_res="{img_height}" filename="sphere_pyramid.png" img_type="png" gamma_corrected="false" />

    <aggregator type="bvh" max_prims_per_node="4"/>

    <world_begin/>
        <!-- The Background -->
        <background type="4_colors" bl="176 214 175" tl="4 99 202" tr="4 99 202" br="176 214 175" />

        <!-- Lights -->
        <light_source type="ambient" I="0.1 0.1 0.1" scale="0.5 0.5 0.5" />
        <light_source type="directional" I="0.9 0.9 0.9" scale="0.6 0.6 0.6" from="1 1.45 -3" to="0 0 1" />
        <light_source type="point" I="0.95 0.9 0.8" scale="0.8 0.8 0.8" from="0 {light_height} -1" />

        <!-- Material Library -->
        <make_named_material type="blinn" name="gold"     ambient="0.4 0.4 0.4" diffuse ="1 0.65 0.0"   specular ="0.8 0.6 0.2"  glossiness ="256" mirror = "0.8 0.8 0.8"/>
        <make_named_material type="blinn" name="grey"     ambient="0.1 0.1 0.1" diffuse ="0.9 0.9 0.9"  specular ="0 0 0"        glossiness ="0"/>
        <make_named_material type="blinn" name="redish"   ambient="0.6 0.6 0.6" diffuse ="0.9 0.2 0.1"  specular ="0.8 0.8 0.8"  glossiness ="64"/>
        <make_named_material type="blinn" name="greenish" ambient="0.6 0.6 0.6" diffuse ="0.2 0.9 0.2"  specular ="0.8 0.8 0.8"  glossiness ="256"/>
        <make_named_material type="blinn" name="blueish"  ambient="0.6 0.6 0.6" diffuse ="0.1 0.2 0.9"  specular ="0.8 0.8 0.8"  glossiness ="16"/>
        <make_named_material type="blinn" name="jade"     ambient="0.6 0.6 0.6" diffuse ="0 0.65 0.29"    specular ="0.8 0.8 0.8" mirror ="0.4 0.4 0.4" glossiness ="128"/>

        <!-- Objects -->

        <named_material name="grey"/>
        <object type="plane" point="0 0 0" normal="0 1 0" />

        <named_material name="jade"/>
"#
    )?;

    for i in 0..args.height {
        for x in 0..(args.height - i) {
            for y in 0..(args.height - i) {
                writeln!(
                    w,
                    "        <object type=\"sphere\" center=\"{} {} {}\" radius=\"0.5\"  />",
                    x as f32 + i as f32 / 2.0,
                    i as f32 * 2f32.sqrt() / 2.0 + 0.5,
                    y as f32 + i as f32 / 2.0
                )?;
            }
        }
        writeln!(w)?;
    }

    write!(
        w,
        r#"
    <world_end/>
</RT3>
"#
    )?;

    Ok(())
}
