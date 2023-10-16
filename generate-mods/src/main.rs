use std::{fs::File, process::Command};

use generate_mods::*;

const AUTOMATED_DISCLAIMER: &str = "//! This file was generated automatically. Do not modify.";
const URL: &str = "https://raw.githubusercontent.com/ppy/osu-web/master/database/mods.json";
const OUT_FILE: &str = "./generated_mods.rs";

fn main() -> GenResult {
    let bytes = minreq::get(URL).send()?.into_bytes();
    let mut rulesets: Vec<RulesetMods> = serde_json::from_slice(&bytes)?;
    RulesetMods::process(&mut rulesets);

    let mut writer = Writer::new(File::create(OUT_FILE)?);
    let mut itoa_buf = itoa::Buffer::new();

    println!("Specifying preamble...");
    specify_preamble(&mut writer, URL, AUTOMATED_DISCLAIMER)?;
    println!("Defining gamemod structs...");
    define_gamemod_structs(&rulesets, &mut writer, &mut itoa_buf)?;
    println!("Defining GameModKind...");
    define_gamemod_kind(&rulesets, &mut writer)?;
    println!("Defining GameModIntermode...");
    define_gamemod_intermode(&rulesets, &mut writer, &mut itoa_buf)?;
    println!("Defining GameModOrder...");
    define_gamemod_order(&rulesets, &mut writer, &mut itoa_buf)?;
    println!("Defining GameMod...");
    define_gamemod_enum(&rulesets, &mut writer)?;
    println!("Defining GameMod methods...");
    define_gamemod_fns(&rulesets, &mut writer)?;
    println!("Implement base traits for GameMod...");
    impl_gamemod_traits(&mut writer)?;
    println!("Implement deserialize logic...");
    impl_serde(&rulesets, &mut writer)?;
    println!("Implementing macro...");
    impl_macro(&rulesets, &mut writer)?;

    writer.flush()?;

    println!("Running formatter...");
    let output = Command::new("rustfmt").arg(OUT_FILE).output()?;

    if output.status.success() {
        println!("Done formatting");
    } else {
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
