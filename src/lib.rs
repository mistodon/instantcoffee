use std::{
    borrow::Cow,
};

use failure::{self, Error};
use coffeegrains::{Ident, Import, Parse, CowParse, Scoped};

type cbstr<'a> = std::borrow::Cow<'a, [u8]>;

pub fn fix_imports<'a>(parse: &mut CowParse, project: &'a [Parse<'a>]) -> Result<(), Error> {
    let project = project.iter().map(Parse::clone).map(Parse::into_cow).collect::<Vec<_>>();

    const PRELUDE: &[cbstr] = &[
        Cow::Borrowed(b"void"),
        Cow::Borrowed(b"int"),
        Cow::Borrowed(b"long"),
        Cow::Borrowed(b"float"),
        Cow::Borrowed(b"double"),
        Cow::Borrowed(b"String"),
    ];

    let all_imports = list_all_imports(&project);
    let symbols_in_package = list_symbols_in_package(&project, &parse.package);
    let external_symbols = find_external_symbols(parse);

    let non_imported_symbols = external_symbols
        .iter()
        .filter(|&&symbol| {
            let base = &symbol.0[0];
            let base_imported = parse
                    .imports
                    .iter()
                    .any(|import| Some(&symbol.0[0]) == import.path.0.last())
                    || symbols_in_package.contains(&&symbol.0[0])
                    || PRELUDE.contains(&symbol.0[0].0);
            let full_path_imported = parse.imports.iter().any(|import| &import.path == symbol);

            !(base_imported || full_path_imported)
        })
        .collect::<Vec<_>>();

    // TODO: Only return missing imports
    Ok(())
}

fn list_all_imports<'a>(project: &'a [CowParse<'a>]) -> Vec<&'a Import<cbstr<'a>>> {
    let mut imports = vec![];

    for parse in project {
        for import in &parse.imports {
            imports.push(import);
        }
    }

    imports
}

fn list_symbols_in_package<'a>(
    project: &'a [CowParse<'a>],
    package: &'a Scoped<cbstr<'a>>,
) -> Vec<&'a Ident<cbstr<'a>>> {
    let mut symbols = vec![];
    for parse in project.iter().filter(|parse| &parse.package == package) {
        for class in &parse.classes {
            symbols.push(&class.name);
        }
    }
    symbols
}

fn find_external_symbols<'a>(parse: &'a CowParse<'a>) -> Vec<&'a Scoped<cbstr<'a>>> {
    let mut types = vec![];

    for class in &parse.classes {
        for annotation in &class.annotations {
            types.push(annotation);
        }
        for field in &class.fields {
            for annotation in &field.annotations {
                types.push(annotation);
            }
            types.push(&field.field_type);
        }
        for method in &class.methods {
            for annotation in &method.annotations {
                types.push(annotation);
            }
            types.push(&method.return_type);
            for throws in &method.throws {
                types.push(throws);
            }

            for arg in &method.args {
                for annotation in &arg.annotations {
                    types.push(annotation);
                }
                types.push(&arg.arg_type);
            }
        }
    }

    let mut symbols = vec![];
    while let Some(ty) = types.pop() {
        symbols.push(&ty.type_name);
        for param in &ty.type_params {
            types.push(param);
        }
    }

    symbols
}

pub fn format_imports<'a>(parse: &CowParse<'a>, _additional: &[Import<&'a [u8]>]) -> Result<String, Error> {
//     use std::fmt::Write;
// 
//     fn format_import(import: &Import<&[u8]>) -> String {
//         let static_label = if import.is_static { "static " } else { "" };
//         let star_label = if import.star { ".*" } else { "" };
//         format!(
//             "import {}{}{};",
//             static_label,
//             import.path,
//             star_label
//         )
//     }
// 
//     let imports = parse.imports.iter();
//     let non_statics = imports.clone().filter(|import| !import.is_static);
//     let mut main_imports = non_statics
//         .clone()
//         .filter(|import| import.path.0[0] != b"java" && import.path.0[0] != b"javax")
//         .map(format_import)
//         .collect::<Vec<_>>();
//     let mut javax_imports = non_statics
//         .clone()
//         .filter(|import| import.path.0[0] == b"javax")
//         .map(format_import)
//         .collect::<Vec<_>>();
//     let mut java_imports = non_statics
//         .filter(|import| import.path.0[0] == b"java")
//         .map(format_import)
//         .collect::<Vec<_>>();
//     let mut statics = parse.imports.iter()
//         .filter(|import| import.is_static)
//         .map(format_import)
//         .collect::<Vec<_>>();
// 
//     main_imports.sort();
//     main_imports.dedup();
//     javax_imports.sort();
//     javax_imports.dedup();
//     java_imports.sort();
//     java_imports.dedup();
//     statics.sort();
//     statics.dedup();
// 
//     let mut buffer = String::new();
//     let b = &mut buffer;
// 
//     if !main_imports.is_empty() {
//         for import in main_imports {
//             writeln!(b, "{}", &import)?;
//         }
//         writeln!(b)?;
//     }
// 
//     if !javax_imports.is_empty() {
//         for import in javax_imports {
//             writeln!(b, "{}", &import)?;
//         }
//         writeln!(b)?;
//     }
// 
//     if !java_imports.is_empty() {
//         for import in java_imports {
//             writeln!(b, "{}", &import)?;
//         }
//         writeln!(b)?;
//     }
// 
//     if !statics.is_empty() {
//         for import in statics {
//             writeln!(b, "{}", &import)?;
//         }
//         writeln!(b)?;
//     }
// 
//     Ok(buffer)
    unimplemented!()
}
