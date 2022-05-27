use crate::{
    map::EntryPos,
    prelude::Mergeable,
    util::{DeleteMap, DeleteVec, SortedDeleteMap},
    Result, UKError,
};
use roead::byml::Byml;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Static {
    pub general: BTreeMap<String, DeleteVec<Byml>>,
    pub start_pos: DeleteMap<String, DeleteMap<String, EntryPos>>,
}

impl TryFrom<&Byml> for Static {
    type Error = UKError;

    fn try_from(byml: &Byml) -> Result<Self> {
        Ok(Self {
            start_pos: byml
                .as_hash()?
                .get("StartPos")
                .ok_or(UKError::MissingBymlKey("CDungeon static missing StartPos"))?
                .as_array()?
                .iter()
                .try_fold(
                    DeleteMap::new(),
                    |mut entry_map,
                     entry|
                     -> Result<DeleteMap<String, DeleteMap<String, EntryPos>>> {
                        let entry = entry.as_hash()?;
                        let map = entry
                            .get("Map")
                            .ok_or(UKError::MissingBymlKey(
                                "CDungeon static entry missing Map name",
                            ))?
                            .as_string()?
                            .to_owned();
                        let pos_name = match entry.get("PosName") {
                            Some(pos_name) => pos_name.as_string()?.to_owned(),
                            _ => return Ok(entry_map),
                        };
                        let rotate = entry
                            .get("Rotate")
                            .ok_or(UKError::MissingBymlKey(
                                "CDungeon static entry missing Rotate",
                            ))?
                            .clone();
                        let translate = entry
                            .get("Translate")
                            .ok_or(UKError::MissingBymlKey(
                                "CDungeon static entry missing Translate",
                            ))?
                            .clone();
                        let player_state = entry
                            .get("PlayerState")
                            .map(|state| -> Result<String> { Ok(state.as_string()?.to_owned()) })
                            .transpose()?;
                        if let Some(map_entries) = entry_map.get_mut(&map) {
                            map_entries.insert(
                                pos_name,
                                EntryPos {
                                    rotate,
                                    translate,
                                    player_state,
                                },
                            );
                        } else {
                            entry_map.insert(
                                map,
                                [(
                                    pos_name,
                                    EntryPos {
                                        rotate,
                                        translate,
                                        player_state,
                                    },
                                )]
                                .into_iter()
                                .collect(),
                            );
                        };
                        Ok(entry_map)
                    },
                )?,
            general: byml
                .as_hash()?
                .iter()
                .map(|(key, array)| -> Result<(String, DeleteVec<Byml>)> {
                    Ok((key.to_owned(), array.as_array()?.iter().cloned().collect()))
                })
                .collect::<Result<_>>()?,
        })
    }
}

impl From<Static> for Byml {
    fn from(val: Static) -> Self {
        [(
            "StartPos".to_owned(),
            val.start_pos
                .into_iter()
                .flat_map(|(map, entries): (String, DeleteMap<String, EntryPos>)| {
                    entries
                        .into_iter()
                        .map(|(pos_name, pos)| {
                            [
                                ("Map", Byml::String(map.clone())),
                                ("PosName", Byml::String(pos_name)),
                                ("Rotate", pos.rotate),
                                ("Translate", pos.translate),
                            ]
                            .into_iter()
                            .collect()
                        })
                        .collect::<Vec<Byml>>()
                })
                .collect(),
        )]
        .into_iter()
        .chain(
            val.general
                .into_iter()
                .map(|(key, array)| (key, array.into_iter().collect())),
        )
        .collect()
    }
}

impl Mergeable<Byml> for Static {
    fn diff(&self, other: &Self) -> Self {
        Self {
            general: other
                .general
                .iter()
                .filter_map(|(key, diff_entries)| {
                    if let Some(self_entries) = self.general.get(key) {
                        if self_entries == diff_entries {
                            None
                        } else {
                            Some((key.clone(), self_entries.diff(diff_entries)))
                        }
                    } else {
                        Some((key.clone(), diff_entries.clone()))
                    }
                })
                .collect(),
            start_pos: self.start_pos.deep_diff(&other.start_pos),
        }
    }

    fn merge(&self, diff: &Self) -> Self {
        Self {
            general: self
                .general
                .iter()
                .map(|(key, self_entries)| {
                    if let Some(diff_entries) = diff.general.get(key) {
                        (key.clone(), self_entries.merge(diff_entries))
                    } else {
                        (key.clone(), self_entries.clone())
                    }
                })
                .collect(),
            start_pos: self.start_pos.deep_merge(&diff.start_pos),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use roead::byml::Byml;

    fn load_mstatic() -> Byml {
        Byml::from_binary(
            &roead::yaz0::decompress(&std::fs::read("test/Map/MainField/Static.smubin").unwrap())
                .unwrap(),
        )
        .unwrap()
    }

    fn load_mod_mstatic() -> Byml {
        Byml::from_binary(
            &roead::yaz0::decompress(
                &std::fs::read("test/Map/MainField/Static.mod.smubin").unwrap(),
            )
            .unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn serde() {
        let byml = load_mstatic();
        let mstatic = super::Static::try_from(&byml).unwrap();
        let data = Byml::from(mstatic.clone()).to_binary(roead::Endian::Big);
        let byml2 = Byml::from_binary(&data).unwrap();
        let mstatic2 = super::Static::try_from(&byml2).unwrap();
        assert_eq!(mstatic, mstatic2);
    }

    #[test]
    fn diff() {
        let byml = load_mstatic();
        let mstatic = super::Static::try_from(&byml).unwrap();
        let byml2 = load_mod_mstatic();
        let mstatic2 = super::Static::try_from(&byml2).unwrap();
        let _diff = mstatic.diff(&mstatic2);
    }

    #[test]
    fn merge() {
        let byml = load_mstatic();
        let mstatic = super::Static::try_from(&byml).unwrap();
        let byml2 = load_mod_mstatic();
        let mstatic2 = super::Static::try_from(&byml2).unwrap();
        let diff = mstatic.diff(&mstatic2);
        let merged = mstatic.merge(&diff);
        assert_eq!(merged, mstatic2);
    }
}
