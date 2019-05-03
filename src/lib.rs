use failure::{self, Error};

use coffeegrains::{Parse, Import};

pub fn prune_imports(&mut Parse) -> Result<(), Error> {
    /*
     * 1. List all imports
     * 2. List symbols that should be imported
     *    - Will need to check whether they were defined in the same package
     *    - Which will require some kind of project structure
     * 3. Take the symmetric difference
     * 4. Remove non-star imports that remain
     *    - This of course will not remove unneeded star imports
     */
    unimplemented!()
}

pub fn fill_in_imports(&mut Parse) -> Result<(), Error> {
    /*
     * 1. List all imports
     * 2. List symbols that should be imported
     *    - Will need to check whether they were defined in the same package
     *    - Which will require some kind of project structure
     * 3. Take the symmetric difference
     * 4. For symbols that are not imported, search project for if they're imported elsewhere
     * 5. Add these imports to the file
     * 6. If any imports are made redundant by star imports, remove them
     */
    unimplemented!()
}

pub fn format_imports(parse: &Parse) -> Result<String, Error> {
    use std::fmt::Write;

    fn format_import(import: &Import) -> String {
        let static_label = if import.is_static { "static " } else { "" };
        let star_label = if import.star { ".*" } else { "" };
        let path = import.path.0.join(&b'.');
        format!(
            "import {}{}{};",
            static_label,
            unsafe { std::str::from_utf8_unchecked(&path) },
            star_label
        )
    }

    let statics = parse.imports.iter().filter(|import| !import.is_static);
    let main_imports = statics
        .clone()
        .filter(|import| import.path.0[0] != b"java" && import.path.0[0] != b"javax")
        .map(format_import)
        .collect::<Vec<_>>();
    let javax_imports = statics
        .clone()
        .filter(|import| import.path.0[0] == b"javax")
        .map(format_import)
        .collect::<Vec<_>>();
    let java_imports = statics
        .filter(|import| import.path.0[0] == b"java")
        .map(format_import)
        .collect::<Vec<_>>();
    let statics = parse
        .imports
        .iter()
        .filter(|import| import.is_static)
        .map(format_import)
        .collect::<Vec<_>>();

    let mut buffer = String::new();
    let b = &mut buffer;

    if !main_imports.is_empty() {
        for import in main_imports {
            writeln!(b, "{}", &import)?;
        }
        writeln!(b)?;
    }

    if !javax_imports.is_empty() {
        for import in javax_imports {
            writeln!(b, "{}", &import)?;
        }
        writeln!(b)?;
    }

    if !java_imports.is_empty() {
        for import in java_imports {
            writeln!(b, "{}", &import)?;
        }
        writeln!(b)?;
    }

    if !statics.is_empty() {
        for import in statics {
            writeln!(b, "{}", &import)?;
        }
        writeln!(b)?;
    }

    Ok(buffer)
}
