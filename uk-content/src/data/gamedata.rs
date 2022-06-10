use crate::{prelude::*, util::SortedDeleteMap, Result, UKError};
use roead::{
    byml::Byml,
    sarc::{Sarc, SarcWriter},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct GameData {
    pub data_type: String,
    pub flags: SortedDeleteMap<u32, Byml>,
}

impl TryFrom<&Byml> for GameData {
    type Error = UKError;

    fn try_from(byml: &Byml) -> Result<Self> {
        let hash = byml.as_hash()?;
        Ok(Self {
            data_type: hash
                .keys()
                .next()
                .ok_or(UKError::MissingBymlKey("bgdata file missing data type key"))?
                .to_owned(),
            flags: hash
                .values()
                .next()
                .ok_or(UKError::MissingBymlKey("bgdata file missing data"))?
                .as_array()?
                .iter()
                .map(|item| -> Result<(u32, Byml)> {
                    Ok((
                        item.as_hash()?
                            .get("HashValue")
                            .ok_or(UKError::MissingBymlKey(
                                "bgdata file entry missing HashValue",
                            ))?
                            .as_int()? as u32,
                        item.clone(),
                    ))
                })
                .collect::<Result<_>>()?,
        })
    }
}

impl From<GameData> for Byml {
    fn from(val: GameData) -> Self {
        [(val.data_type, val.flags.values().cloned().collect())]
            .into_iter()
            .collect()
    }
}

impl GameData {
    fn divide(self) -> Vec<GameData> {
        let total = (self.flags.len() as f32 / 4096.).ceil() as usize;
        let mut iter = self.flags.into_iter();
        let mut out = Vec::with_capacity(total);
        for _ in 0..total {
            out.push(GameData {
                data_type: self.data_type.clone(),
                flags: iter.by_ref().take(4096).collect(),
            });
        }
        out
    }
}

impl Mergeable<Byml> for GameData {
    fn diff(&self, other: &Self) -> Self {
        assert_eq!(
            self.data_type, other.data_type,
            "Attempted to diff different gamedata types: {} and {}",
            self.data_type, other.data_type
        );
        Self {
            data_type: self.data_type.clone(),
            flags: self.flags.diff(&other.flags),
        }
    }

    fn merge(&self, diff: &Self) -> Self {
        assert_eq!(
            self.data_type, diff.data_type,
            "Attempted to merge different gamedata types: {} and {}",
            self.data_type, diff.data_type
        );
        Self {
            data_type: self.data_type.clone(),
            flags: self.flags.merge(&diff.flags),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct GameDataPack {
    pub bool_array_data: GameData,
    pub bool_data: GameData,
    pub f32_array_data: GameData,
    pub f32_data: GameData,
    pub revival_bool_data: GameData,
    pub revival_s32_data: GameData,
    pub s32_array_data: GameData,
    pub s32_data: GameData,
    pub string32_data: GameData,
    pub string64_array_data: GameData,
    pub string64_data: GameData,
    pub string256_array_data: GameData,
    pub string256_data: GameData,
    pub vector2f_array_data: GameData,
    pub vector2f_data: GameData,
    pub vector3f_array_data: GameData,
    pub vector3f_data: GameData,
    pub vector4f_data: GameData,
}

macro_rules! extract_sarc_gamedata {
    ($sarc:expr, $($type:ident),*) => {
        Self {
            $(
                $type: {
                    let _key = stringify!($type);
                    $sarc.files().filter_map(|file| -> Option<Result<GameData>> {
                        if file.name().map(|n| n.trim_start_matches('/').starts_with(_key)).unwrap_or(false) {
                            Some(Byml::from_binary(file.data())
                                .map_err(UKError::from)
                                .and_then(|byml: Byml| -> Result<GameData> { (&byml).try_into() }))
                        } else {
                            None
                        }
                    })
                    .collect::<Result<Vec<GameData>>>()?
                    .into_iter()
                    .fold(GameData {
                        data_type: if _key == "string32_data" {
                            "string_data"
                        } else {
                            _key.trim_start_matches("revival_")
                        }.to_owned(),
                        flags: SortedDeleteMap::new(),
                    }, |acc, val| {
                        acc.merge(&val)
                    })
                },
            )*
        }
    };
}

macro_rules! extract_sarcwriter_gamedata {
    ($sarc:expr, $($type:ident),*) => {
        Self {
            $(
                $type: {
                    let _key = stringify!($type);
                    $sarc.files.iter().filter_map(|(file, data)| -> Option<Result<GameData>> {
                        if file.trim_start_matches('/').starts_with(_key) {
                            Some(Byml::from_binary(&data)
                                .map_err(UKError::from)
                                .and_then(|byml: Byml| -> Result<GameData> { (&byml).try_into() }))
                        } else {
                            None
                        }
                    })
                    .collect::<Result<Vec<GameData>>>()?
                    .into_iter()
                    .fold(GameData {
                        data_type: if _key == "string32_data" {
                            "string_data"
                        } else {
                            _key.trim_start_matches("revival_")
                        }.to_owned(),
                        flags: SortedDeleteMap::new(),
                    }, |acc, val| {
                        acc.merge(&val)
                    })
                },
            )*
        }
    };
}

macro_rules! build_gamedata_pack {
    ($data:expr, $sarc:expr, $($type:ident),*) => {
        let _endian = $sarc.endian;
        $(
            let _type = $data.$type.data_type.clone();
            $sarc.add_files($data.$type.divide().into_iter().enumerate().map(|(i, data)| {
                (
                    format!("/{}_{}.bgdata", stringify!($type), i),
                    Byml::from(data).to_binary(_endian),
                )
            }));
        )*
    };
}

impl GameDataPack {
    pub fn from_sarc_writer(sarc: &SarcWriter) -> Result<Self> {
        Ok(extract_sarcwriter_gamedata!(
            sarc,
            bool_array_data,
            bool_data,
            f32_array_data,
            f32_data,
            revival_bool_data,
            revival_s32_data,
            s32_array_data,
            s32_data,
            string32_data,
            string64_array_data,
            string64_data,
            string256_array_data,
            string256_data,
            vector2f_array_data,
            vector2f_data,
            vector3f_array_data,
            vector3f_data,
            vector4f_data
        ))
    }

    pub fn from_sarc(sarc: &Sarc<'_>) -> Result<Self> {
        Ok(extract_sarc_gamedata!(
            sarc,
            bool_array_data,
            bool_data,
            f32_array_data,
            f32_data,
            revival_bool_data,
            revival_s32_data,
            s32_array_data,
            s32_data,
            string32_data,
            string64_array_data,
            string64_data,
            string256_array_data,
            string256_data,
            vector2f_array_data,
            vector2f_data,
            vector3f_array_data,
            vector3f_data,
            vector4f_data
        ))
    }

    pub fn into_sarc_writer(self, endian: Endian) -> SarcWriter {
        let mut sarc = SarcWriter::new(endian.into());
        build_gamedata_pack!(
            self,
            sarc,
            bool_array_data,
            bool_data,
            f32_array_data,
            f32_data,
            revival_bool_data,
            revival_s32_data,
            s32_array_data,
            s32_data,
            string32_data,
            string64_array_data,
            string64_data,
            string256_array_data,
            string256_data,
            vector2f_array_data,
            vector2f_data,
            vector3f_array_data,
            vector3f_data,
            vector4f_data
        );
        sarc
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use roead::{byml::Byml, sarc::Sarc};

    fn load_gamedata_sarc() -> Sarc<'static> {
        Sarc::read(std::fs::read("test/GameData/gamedata.ssarc").unwrap()).unwrap()
    }

    fn load_gamedata() -> Byml {
        let gs = load_gamedata_sarc();
        Byml::from_binary(&gs.get_file_data("/revival_s32_data_0.bgdata").unwrap()).unwrap()
    }

    fn load_mod_gamedata() -> Byml {
        let gs = load_gamedata_sarc();
        Byml::from_binary(&gs.get_file_data("/revival_s32_data_0.mod.bgdata").unwrap()).unwrap()
    }

    #[test]
    fn serde() {
        let byml = load_gamedata();
        let gamedata = super::GameData::try_from(&byml).unwrap();
        let data = Byml::from(gamedata.clone()).to_binary(roead::Endian::Big);
        let byml2 = Byml::from_binary(&data).unwrap();
        let gamedata2 = super::GameData::try_from(&byml2).unwrap();
        assert_eq!(gamedata, gamedata2);
    }

    #[test]
    fn diff() {
        let byml = load_gamedata();
        let gamedata = super::GameData::try_from(&byml).unwrap();
        let byml2 = load_mod_gamedata();
        let gamedata2 = super::GameData::try_from(&byml2).unwrap();
        let diff = gamedata.diff(&gamedata2);
        dbg!(diff);
    }

    #[test]
    fn merge() {
        let byml = load_gamedata();
        let gamedata = super::GameData::try_from(&byml).unwrap();
        let byml2 = load_mod_gamedata();
        let gamedata2 = super::GameData::try_from(&byml2).unwrap();
        let diff = gamedata.diff(&gamedata2);
        let merged = gamedata.merge(&diff);
        assert_eq!(merged, gamedata2);
    }

    #[test]
    fn pack() {
        let gs = load_gamedata_sarc();
        let gamedata = super::GameDataPack::from_sarc(&gs).unwrap();
        let gs2 = gamedata
            .clone()
            .into_sarc_writer(crate::prelude::Endian::Big);
        let gamedata2 = super::GameDataPack::from_sarc_writer(&gs2).unwrap();
        assert_eq!(gamedata, gamedata2);
    }
}
