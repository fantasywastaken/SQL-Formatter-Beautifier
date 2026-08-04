use anyhow::{anyhow, Context, Result};
use clap::{Parser, ValueEnum};
use sqlformat::{format, FormatOptions, Indent, QueryParams};
use std::fs;
use std::io::{self, Read, Write};

#[derive(Parser)]
#[command(name = "sqlfmt", version, about = "SQL formatter and beautifier", long_about = None)]
struct Cli {
    #[arg(help = "SQL source file. If omitted, reads from stdin.")]
    file: Option<String>,

    #[arg(long, default_value = "2", help = "Indent width: a number of spaces (e.g. 2, 4) or 'tab'")]
    indent: String,

    #[arg(long, value_enum, default_value_t = CaseMode::Preserve, help = "Keyword case handling")]
    case: CaseMode,

    #[arg(long, default_value_t = 1, help = "Blank lines between statements")]
    lines_between: u8,

    #[arg(long, value_enum, default_value_t = Dialect::Postgres, help = "Target SQL dialect (documented; formatting is dialect-neutral)")]
    dialect: Dialect,

    #[arg(short, long, help = "Write output to this file instead of stdout")]
    output: Option<String>,
}

#[derive(Clone, Copy, ValueEnum)]
enum CaseMode {
    Upper,
    Lower,
    Preserve,
}

#[derive(Clone, Copy, ValueEnum)]
enum Dialect {
    Postgres,
    Mysql,
    Sqlite,
    Tsql,
}

fn parse_indent(spec: &str) -> Result<Indent> {
    let lower = spec.to_lowercase();
    match lower.as_str() {
        "tab" | "tabs" | "t" | "\\t" => Ok(Indent::Tabs),
        other => {
            let n: u8 = other
                .parse()
                .with_context(|| format!("Invalid indent '{}': expected a number or 'tab'", spec))?;
            if n == 0 {
                return Err(anyhow!("Indent width must be at least 1"));
            }
            Ok(Indent::Spaces(n))
        }
    }
}

fn read_source(path: Option<&str>) -> Result<String> {
    match path {
        Some(p) => fs::read_to_string(p).with_context(|| format!("Failed to read {}", p)),
        None => {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .context("Failed to read from stdin")?;
            Ok(buf)
        }
    }
}

fn write_output(text: &str, path: Option<&str>) -> Result<()> {
    match path {
        Some(p) => fs::write(p, text).with_context(|| format!("Failed to write {}", p)),
        None => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            handle.write_all(text.as_bytes())?;
            if !text.ends_with('\n') {
                handle.write_all(b"\n")?;
            }
            Ok(())
        }
    }
}

fn apply_case(text: String, mode: CaseMode) -> String {
    match mode {
        CaseMode::Upper | CaseMode::Preserve => text,
        CaseMode::Lower => text.to_lowercase(),
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let source = read_source(cli.file.as_deref())?;

    let indent = parse_indent(&cli.indent)?;
    let uppercase = matches!(cli.case, CaseMode::Upper);

    let options = FormatOptions {
        indent,
        uppercase,
        lines_between_queries: cli.lines_between.max(1),
    };

    let formatted = format(&source, &QueryParams::None, options);
    let final_text = apply_case(formatted, cli.case);

    let _ = cli.dialect;

    write_output(&final_text, cli.output.as_deref())?;
    Ok(())
}
