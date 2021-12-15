#![allow(dead_code, unused_imports)]

pub type Result<T, E = anyhow::Error> = std::result::Result<T, E>;
pub use anyhow::anyhow;
pub use itertools::Itertools as _;
pub use rayon::prelude::*;
pub use recap::{Recap, Regex};
pub use serde::{Deserialize, Serialize};
pub use std::collections::{
    BTreeMap, BTreeMap as Map, BTreeSet, BTreeSet as Set, HashMap, HashSet,
};
pub use std::str::FromStr;
