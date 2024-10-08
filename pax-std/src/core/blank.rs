#[allow(unused)]
use crate::*;
use pax_engine::pax;
/// A blank component, roughly an alias for <Group />, used in cases where
/// a dummy or placeholder is needed (e.g. within designer)
#[pax]
#[engine_import_path("pax_engine")]
#[inlined(<Group/>)]
pub struct BlankComponent {}
