mod parse;

use failure::{self, Error};

use parse::Parser;

type Ident<'a> = &'a [u8];

#[derive(Debug, PartialEq)]
pub struct Path<'a>(Vec<Ident<'a>>);

#[derive(Debug, PartialEq)]
pub struct SymbolSoup<'a> {
    pub idents: Vec<Ident<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Parse<'a> {
    pub source: &'a str,
    pub package: Path<'a>,
    pub imports: Vec<Import<'a>>,
    pub import_span: [usize; 2],
    pub classes: Vec<Class<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Import<'a> {
    pub path: Path<'a>,
    pub is_static: bool,
    pub star: bool,
}

#[derive(Debug, PartialEq)]
pub struct Class<'a> {
    pub annotations: Vec<Type<'a>>,
    pub name: Ident<'a>,
    pub fields: Vec<Field<'a>>,
    pub methods: Vec<Method<'a>>,
    pub type_params: Vec<Ident<'a>>,
    pub subtypes: Vec<Type<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Type<'a> {
    pub type_name: Path<'a>,
    pub type_params: Vec<Type<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Field<'a> {
    pub annotations: Vec<Type<'a>>,
    pub name: Ident<'a>,
    pub field_type: Type<'a>,
    pub value: SymbolSoup<'a>,
}

#[derive(Debug, PartialEq)]
pub struct Method<'a> {
    pub annotations: Vec<Type<'a>>,
    pub name: Ident<'a>,
    pub return_type: Type<'a>,
    pub type_parameters: Vec<Ident<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Arg<'a> {
    pub annotations: Vec<Type<'a>>,
    pub name: Ident<'a>,
    pub arg_type: Type<'a>,
}

fn is_ident_char(ch: u8) -> bool {
    ch == b'_'
        || (b'a' <= ch && ch <= b'z')
        || (b'A' <= ch && ch <= b'Z')
        || (b'0' <= ch && ch <= b'9')
}

pub fn parse_file(source: &str) -> Result<Parse, Error> {
    let parser = &mut Parser::new(source);
    parser.skip_whitespace();
    parser.expect_keyword(b"package")?;

    let package = parse_path(parser);
    parser.expect(b";")?;

    let mut imports = vec![];
    let import_start_point = parser.cursor();

    while parser.skip_keyword(b"import") {
        let is_static = parser.skip_keyword(b"static");
        let path = parse_path(parser);
        let star = parser.skip_only(b"*");

        imports.push(Import {
            path,
            is_static,
            star,
        });

        parser.expect(b";")?;
    }

    let import_end_point = parser.cursor();

    Ok(Parse {
        source,
        package,
        imports,
        import_span: [import_start_point, import_end_point],
        classes: vec![],
    })
}

fn parse_path<'a>(parser: &mut Parser<'a>) -> Path<'a> {
    let mut path = vec![];
    while let Some(ident) = parser.skip_matching(is_ident_char) {
        path.push(ident);
        if !parser.skip(b".") {
            break;
        }
    }
    Path(path)
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
    for import in main_imports {
        writeln!(b, "{}", &import)?;
    }
    writeln!(b)?;
    for import in javax_imports {
        writeln!(b, "{}", &import)?;
    }
    writeln!(b)?;
    for import in java_imports {
        writeln!(b, "{}", &import)?;
    }
    writeln!(b)?;
    for import in statics {
        writeln!(b, "{}", &import)?;
    }
    writeln!(b)?;

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_package_success() {
        let tests: &[(&str, &[&[u8]])] = &[
            ("package a;", &[b"a"]),
            ("package a.b.c;", &[b"a", b"b", b"c"]),
            (
                "package com.falseidolfactory.thing;",
                &[b"com", b"falseidolfactory", b"thing"],
            ),
        ];

        for (src, package) in tests {
            assert_eq!(parse_file(src).unwrap().package, Path(package.to_vec()));
        }
    }

    #[test]
    fn parse_imports_success() {
        let source = "
            package com.com.com;

            import a.b.c;
            import a;

            import java.util.List;
            import java.concurrent.*;

            import static java.util.Arrays.asList;
        ";
        let result = parse_file(source).unwrap();
        let expected = &[
            Import {
                path: Path(vec![b"a", b"b", b"c"]),
                is_static: false,
                star: false,
            },
            Import {
                path: Path(vec![b"a"]),
                is_static: false,
                star: false,
            },
            Import {
                path: Path(vec![b"java", b"util", b"List"]),
                is_static: false,
                star: false,
            },
            Import {
                path: Path(vec![b"java", b"concurrent"]),
                is_static: false,
                star: true,
            },
            Import {
                path: Path(vec![b"java", b"util", b"Arrays", b"asList"]),
                is_static: true,
                star: false,
            },
        ];

        assert_eq!(result.imports, expected);
    }
}
