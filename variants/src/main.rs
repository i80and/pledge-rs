mod promise;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::iter::empty;
use std::iter::once;
use std::ops::RangeInclusive;

use reqwest::StatusCode;
use scraper::{ElementRef, Html, Selector};

use self::promise::Promise;

const IOCTL: &'_ str = r#"dt > a[href$="/ioctl.2"]"#;
const OTHER: &'_ str = r#"dt > var"#;
const VAR: &'_ str = r#"var"#;

fn address(version: usize) -> String {
    format!(
        "https://man.openbsd.org/OpenBSD-{}.{}/pledge.2",
        version / 10,
        version % 10
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut promises: BTreeMap<String, Promise> = Default::default();
    let client = reqwest::Client::new();

    for version in 59.. {
        let address = address(version);

        eprintln!("{}", address);

        let mut response = client.get(&address).send()?;

        match response.status() {
            // stop: OpenBSD version not found
            StatusCode::BAD_REQUEST => break,

            // stop: pledge(2) page not found
            StatusCode::NOT_FOUND => break,

            x if !x.is_success() => panic!("{}", x),

            _ => {}
        }

        let document = Html::parse_document(&response.text()?);

        let ioctl = Selector::parse(IOCTL).expect("selector is static");
        let other = Selector::parse(OTHER).expect("selector is static");
        let var = Selector::parse(VAR).expect("selector is static");

        let keywords = document
            .select(&ioctl)
            .find_map(|x| {
                x.parent()
                    .expect("parent is a <dt>")
                    .next_siblings()
                    .find_map(ElementRef::wrap)
            })
            .iter()
            .flat_map(|x| x.select(&var))
            .chain(document.select(&other))
            .collect::<Vec<_>>();

        for keyword in keywords {
            let keyword = keyword.text().collect::<String>();
            let variant = camelcase(&keyword);
            promises
                .entry(variant)
                .or_insert_with(|| Promise::new(keyword))
                .versions
                .insert(version);
        }
    }

    println!("// This file was generated from the pledge(2) manual pages");
    println!("// using the helper documented at </variants/README.md>.");
    println!();
    println!("pub enum Promise {{");

    for (variant, promise) in &promises {
        let versions = ranges(&promise.versions)
            .into_iter()
            .map(|x| {
                let (p, q) = x.into_inner();

                if p == q {
                    format!("{}.{}", p / 10, p % 10)
                } else {
                    format!("{}.{}–{}.{}", p / 10, p % 10, q / 10, q % 10)
                }
            })
            .collect::<Vec<_>>()
            .join(", ");

        println!("    /// OpenBSD {}", versions);
        println!("    {},", variant);
        println!();
    }

    println!("    // FIXME rust-lang/rust#44109");
    println!("    #[doc(hidden)]");
    println!("    _NonExhaustive,");
    println!("}}");
    println!();
    println!("impl Promise {{");
    println!("    pub fn to_promise_string(&self) -> &'static str {{");
    println!("        match *self {{");

    for (variant, promise) in &promises {
        println!("            Promise::{} => {:?},", variant, promise.keyword);
    }

    let error = "Promise::to_promise_string is incomplete (bug)";
    println!("            _ => panic!({:?}),", error);
    println!("        }}");
    println!("    }}");
    println!("}}");

    Ok(())
}

fn camelcase(keyword: &str) -> String {
    keyword
        .split('_')
        .flat_map(|x| -> Box<dyn Iterator<Item = char>> {
            let mut chars = x.chars();

            if let Some(first) = chars.next() {
                Box::new(once(first.to_ascii_uppercase()).chain(chars))
            } else {
                Box::new(empty())
            }
        })
        .collect()
}

/// # Panics
///
/// Panics if versions is empty.
fn ranges(versions: &BTreeSet<usize>) -> Vec<RangeInclusive<usize>> {
    let mut versions = versions.iter().copied();
    let first = versions.next().expect("contract");

    versions.fold(vec![first..=first], |mut a, x| {
        let (p, q) = a.pop().expect("never empty").into_inner();

        if x == q + 1 {
            a.push(p..=x);
        } else {
            a.push(p..=q);
            a.push(x..=x);
        }

        return a;
    })
}

#[cfg(test)]
mod test {
    #[test]
    fn camelcase() {
        assert_eq!(super::camelcase("prot_exec"), "ProtExec");
    }

    #[test]
    fn ranges() {
        assert_eq!(super::ranges(&set(&[1])), [1..=1]);
        assert_eq!(super::ranges(&set(&[1, 2])), [1..=2]);
        assert_eq!(super::ranges(&set(&[1, 2, 3])), [1..=3]);
        assert_eq!(super::ranges(&set(&[1, 3])), [1..=1, 3..=3]);

        fn set(x: &[usize]) -> std::collections::BTreeSet<usize> {
            x.iter().copied().collect()
        }
    }
}
