use std::sync::LazyLock;

use crate::windows::procedure::Procedure;

static ADDRESS: LazyLock<Procedure> = LazyLock::new(|| Procedure::new("Kernel32", "LoadLibraryW").unwrap());

pub struct Loader;
