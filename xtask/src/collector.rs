use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{self, Item, Visibility};

#[derive(Debug, PartialEq, Eq)]
pub struct ModInfo {
    pub path: String,
    pub items: Vec<String>,
}

fn is_pub(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}

pub fn collect() -> Result<Vec<ModInfo>> {
    let crate_dir = PathBuf::from("scroll_core/src");
    let root = crate_dir.join("lib.rs");
    let src = fs::read_to_string(&root)?;
    let file = syn::parse_file(&src)?;
    let mut out = Vec::new();
    collect_from_items(file.items, "crate", &crate_dir, &mut out)?;
    Ok(out)
}

fn collect_from_items(
    items: Vec<Item>,
    module_path: &str,
    dir: &Path,
    out: &mut Vec<ModInfo>,
) -> Result<()> {
    let mut info = ModInfo {
        path: module_path.to_string(),
        items: Vec::new(),
    };
    for item in items {
        match item {
            Item::Struct(s) if is_pub(&s.vis) => info.items.push(s.ident.to_string()),
            Item::Enum(e) if is_pub(&e.vis) => info.items.push(e.ident.to_string()),
            Item::Mod(m) => {
                let sub_path = format!("{}::{}", module_path, m.ident);
                if let Some((_, items)) = m.content {
                    collect_from_items(items, &sub_path, &dir.join(m.ident.to_string()), out)?;
                } else {
                    let file_rs = dir.join(format!("{}.rs", m.ident));
                    let file_mod = dir.join(m.ident.to_string()).join("mod.rs");
                    let mod_file = if file_rs.exists() {
                        file_rs
                    } else if file_mod.exists() {
                        file_mod
                    } else {
                        continue;
                    };
                    let content = fs::read_to_string(&mod_file)?;
                    let file = syn::parse_file(&content)?;
                    let sub_dir = mod_file.parent().unwrap_or(dir);
                    collect_from_items(file.items, &sub_path, sub_dir, out)?;
                }
            }
            _ => {}
        }
    }
    out.push(info);
    Ok(())
}

pub fn collect_from_source(source: &str, module_path: &str) -> Result<ModInfo> {
    let file = syn::parse_file(source)?;
    let mut info = ModInfo {
        path: module_path.to_string(),
        items: Vec::new(),
    };
    for item in file.items {
        match item {
            Item::Struct(s) if is_pub(&s.vis) => info.items.push(s.ident.to_string()),
            Item::Enum(e) if is_pub(&e.vis) => info.items.push(e.ident.to_string()),
            _ => {}
        }
    }
    Ok(info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_from_source_detects_public_items() {
        let src = "pub struct A; struct B; pub enum C {}";
        let info = collect_from_source(src, "crate::test").unwrap();
        assert_eq!(info.items, vec!["A", "C"]);
    }
}
