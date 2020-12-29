extern crate parity_wasm;

use parity_wasm::elements::Section;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: wasm-info <*.wasm> [-n]");
        return;
    }

    let mut module = parity_wasm::deserialize_file(&args[1]).expect("Failed to load module");

    println!("Module sections: {}", module.sections().len());

    for section in module.sections() {
        match *section {
            Section::Import(ref import_section) => {
                println!("  Imports: {}", import_section.entries().len());
                import_section
                    .entries()
                    .iter()
                    .map(|e| println!("    {}.{}", e.module(), e.field()))
                    .count();
            }
            Section::Export(ref exports_section) => {
                println!("  Exports: {}", exports_section.entries().len());
                exports_section
                    .entries()
                    .iter()
                    .map(|e| println!("    {}", e.field()))
                    .count();
            }
            Section::Function(ref function_section) => {
                println!("  Functions: {}", function_section.entries().len());
            }
            Section::Type(ref type_section) => {
                println!("  Types: {}", type_section.types().len());
            }
            Section::Global(ref globals_section) => {
                println!("  Globals: {}", globals_section.entries().len());
            }
            Section::Table(ref table_section) => {
                println!("  Tables: {}", table_section.entries().len());
            }
            Section::Memory(ref memory_section) => {
                println!("  Memories: {}", memory_section.entries().len());
            }
            Section::Data(ref data_section) if data_section.entries().len() > 0 => {
                let data = &data_section.entries()[0];
                println!("  Data size: {}", data.value().len());
            }
            Section::Start(ref start_section) => {
                println!("  Start: {}", start_section);
            }
            _ => {}
        }
    }

    if args.len() > 2 && args[2] == "-n" {
        module = match module.parse_names() {
            Ok(module) => module,
            Err((_, module)) => module,
        };

        let res = module
            .names_section()
            .unwrap()
            .functions()
            .unwrap()
            .names()
            .iter()
            .map(|(idx, func)| format!("{:#}[{}]", rustc_demangle::demangle(func), idx))
            .collect::<Vec<_>>();

        if res.is_empty() {
            println!("No name section in {}", &args[1]);
        }
        println!("{:#?}", res);
    }
}
