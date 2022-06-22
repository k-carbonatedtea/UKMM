use anyhow::{Context, Result};
use join_str::jstr;
use roead::aamp::ParameterIO;
use roead::byml::Byml;
use roead::sarc::SarcWriter;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
use uk_content::prelude::*;
use uk_content::{
    actor::{
        info::ActorInfo,
        params::{
            aiprog::AIProgram, aischedule::AISchedule, animinfo::AnimationInfo, aslist::ASList,
            atcl::AttClient, atcllist::AttClientList, aware::Awareness, bonectrl::BoneControl,
            chemical::Chemical, damage::DamageParam, drop::DropTable, general::GeneralParamList,
            life::LifeCondition, link::ActorLink, lod::Lod, modellist::ModelList, physics::Physics,
            r#as::AS, recipe::Recipe, rgbw::RagdollBlendWeight, rgconfig::RagdollConfig,
            rgconfiglist::RagdollConfigList, shop::ShopData, umii::UMii,
        },
        residents::ResidentActors,
        Actor,
    },
    chemical::chmres::ChemicalRes,
    cooking::data::CookData,
    data::{gamedata::GameDataPack, savedata::SaveDataPack, shop::ShopGameDataInfo},
    demo::Demo,
    eco::{areadata::AreaData, level::LevelSensor, status::StatusEffectList},
    event::{info::EventInfo, residents::ResidentEvents},
    map::{lazy::LazyTraverseList, mainfield::location::Location, static_::Static, unit::MapUnit},
    quest::product::QuestProduct,
    sound::barslist::BarslistInfo,
    tips::Tips,
    util::SortedDeleteMap,
    worldmgr::info::WorldInfo,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MergeableResource {
    Actor(Box<Actor>),
    ActorInfo(Box<ActorInfo>),
    ActorLink(Box<ActorLink>),
    AIProgram(Box<AIProgram>),
    AISchedule(Box<AISchedule>),
    AnimationInfo(Box<AnimationInfo>),
    AreaData(Box<AreaData>),
    AS(Box<AS>),
    ASList(Box<ASList>),
    AttClient(Box<AttClient>),
    AttClientList(Box<AttClientList>),
    Awareness(Box<Awareness>),
    BarslistInfo(Box<BarslistInfo>),
    BoneControl(Box<BoneControl>),
    Chemical(Box<Chemical>),
    ChemicalRes(Box<ChemicalRes>),
    CookData(Box<CookData>),
    DamageParam(Box<DamageParam>),
    Demo(Box<Demo>),
    DropTable(Box<DropTable>),
    EventInfo(Box<EventInfo>),
    GameDataPack(Box<GameDataPack>),
    GeneralParamList(Box<GeneralParamList>),
    LazyTraverseList(Box<LazyTraverseList>),
    LevelSensor(Box<LevelSensor>),
    LifeCondition(Box<LifeCondition>),
    Location(Box<Location>),
    Lod(Box<Lod>),
    MapUnit(Box<MapUnit>),
    ModelList(Box<ModelList>),
    Physics(Box<Physics>),
    QuestProduct(Box<QuestProduct>),
    RagdollBlendWeight(Box<RagdollBlendWeight>),
    RagdollConfig(Box<RagdollConfig>),
    RagdollConfigList(Box<RagdollConfigList>),
    Recipe(Box<Recipe>),
    ResidentActors(Box<ResidentActors>),
    ResidentEvents(Box<ResidentEvents>),
    SaveDataPack(Box<SaveDataPack>),
    ShopData(Box<ShopData>),
    ShopGameDataInfo(Box<ShopGameDataInfo>),
    Static(Box<Static>),
    StatusEffectList(Box<StatusEffectList>),
    Tips(Box<Tips>),
    UMii(Box<UMii>),
    WorldInfo(Box<WorldInfo>),
    GenericAamp(Box<ParameterIO>),
    GenericByml(Box<Byml>),
}

impl Mergeable for MergeableResource {
    fn diff(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Actor(a), Self::Actor(b)) => Self::Actor(Box::new(a.diff(b))),
            (Self::ActorInfo(a), Self::ActorInfo(b)) => Self::ActorInfo(Box::new(a.diff(b))),
            (Self::ActorLink(a), Self::ActorLink(b)) => Self::ActorLink(Box::new(a.diff(b))),
            (Self::AIProgram(a), Self::AIProgram(b)) => Self::AIProgram(Box::new(a.diff(b))),
            (Self::AISchedule(a), Self::AISchedule(b)) => Self::AISchedule(Box::new(a.diff(b))),
            (Self::AnimationInfo(a), Self::AnimationInfo(b)) => {
                Self::AnimationInfo(Box::new(a.diff(b)))
            }
            (Self::AreaData(a), Self::AreaData(b)) => Self::AreaData(Box::new(a.diff(b))),
            (Self::AS(a), Self::AS(b)) => Self::AS(Box::new(a.diff(b))),
            (Self::ASList(a), Self::ASList(b)) => Self::ASList(Box::new(a.diff(b))),
            (Self::AttClient(a), Self::AttClient(b)) => Self::AttClient(Box::new(a.diff(b))),
            (Self::AttClientList(a), Self::AttClientList(b)) => {
                Self::AttClientList(Box::new(a.diff(b)))
            }
            (Self::Awareness(a), Self::Awareness(b)) => Self::Awareness(Box::new(a.diff(b))),
            (Self::BarslistInfo(a), Self::BarslistInfo(b)) => {
                Self::BarslistInfo(Box::new(a.diff(b)))
            }
            (Self::BoneControl(a), Self::BoneControl(b)) => Self::BoneControl(Box::new(a.diff(b))),
            (Self::Chemical(a), Self::Chemical(b)) => Self::Chemical(Box::new(a.diff(b))),
            (Self::ChemicalRes(a), Self::ChemicalRes(b)) => Self::ChemicalRes(Box::new(a.diff(b))),
            (Self::CookData(a), Self::CookData(b)) => Self::CookData(Box::new(a.diff(b))),
            (Self::DamageParam(a), Self::DamageParam(b)) => Self::DamageParam(Box::new(a.diff(b))),
            (Self::Demo(a), Self::Demo(b)) => Self::Demo(Box::new(a.diff(b))),
            (Self::DropTable(a), Self::DropTable(b)) => Self::DropTable(Box::new(a.diff(b))),
            (Self::EventInfo(a), Self::EventInfo(b)) => Self::EventInfo(Box::new(a.diff(b))),
            (Self::GameDataPack(a), Self::GameDataPack(b)) => {
                Self::GameDataPack(Box::new(a.diff(b)))
            }
            (Self::GeneralParamList(a), Self::GeneralParamList(b)) => {
                Self::GeneralParamList(Box::new(a.diff(b)))
            }
            (Self::LazyTraverseList(a), Self::LazyTraverseList(b)) => {
                Self::LazyTraverseList(Box::new(a.diff(b)))
            }
            (Self::LevelSensor(a), Self::LevelSensor(b)) => Self::LevelSensor(Box::new(a.diff(b))),
            (Self::LifeCondition(a), Self::LifeCondition(b)) => {
                Self::LifeCondition(Box::new(a.diff(b)))
            }
            (Self::Location(a), Self::Location(b)) => Self::Location(Box::new(a.diff(b))),
            (Self::Lod(a), Self::Lod(b)) => Self::Lod(Box::new(a.diff(b))),
            (Self::MapUnit(a), Self::MapUnit(b)) => Self::MapUnit(Box::new(a.diff(b))),
            (Self::ModelList(a), Self::ModelList(b)) => Self::ModelList(Box::new(a.diff(b))),
            (Self::Physics(a), Self::Physics(b)) => Self::Physics(Box::new(a.diff(b))),
            (Self::QuestProduct(a), Self::QuestProduct(b)) => {
                Self::QuestProduct(Box::new(a.diff(b)))
            }
            (Self::RagdollBlendWeight(a), Self::RagdollBlendWeight(b)) => {
                Self::RagdollBlendWeight(Box::new(a.diff(b)))
            }
            (Self::RagdollConfig(a), Self::RagdollConfig(b)) => {
                Self::RagdollConfig(Box::new(a.diff(b)))
            }
            (Self::RagdollConfigList(a), Self::RagdollConfigList(b)) => {
                Self::RagdollConfigList(Box::new(a.diff(b)))
            }
            (Self::Recipe(a), Self::Recipe(b)) => Self::Recipe(Box::new(a.diff(b))),
            (Self::ResidentActors(a), Self::ResidentActors(b)) => {
                Self::ResidentActors(Box::new(a.diff(b)))
            }
            (Self::ResidentEvents(a), Self::ResidentEvents(b)) => {
                Self::ResidentEvents(Box::new(a.diff(b)))
            }
            (Self::SaveDataPack(a), Self::SaveDataPack(b)) => {
                Self::SaveDataPack(Box::new(a.diff(b)))
            }
            (Self::ShopData(a), Self::ShopData(b)) => Self::ShopData(Box::new(a.diff(b))),
            (Self::ShopGameDataInfo(a), Self::ShopGameDataInfo(b)) => {
                Self::ShopGameDataInfo(Box::new(a.diff(b)))
            }
            (Self::Static(a), Self::Static(b)) => Self::Static(Box::new(a.diff(b))),
            (Self::StatusEffectList(a), Self::StatusEffectList(b)) => {
                Self::StatusEffectList(Box::new(a.diff(b)))
            }
            (Self::Tips(a), Self::Tips(b)) => Self::Tips(Box::new(a.diff(b))),
            (Self::UMii(a), Self::UMii(b)) => Self::UMii(Box::new(a.diff(b))),
            (Self::WorldInfo(a), Self::WorldInfo(b)) => Self::WorldInfo(Box::new(a.diff(b))),
            _ => panic!(
                "Tried to diff incompatible resources: {:?} and {:?}",
                &self, &other
            ),
        }
    }

    fn merge(&self, diff: &Self) -> Self {
        match (self, diff) {
            (Self::Actor(a), Self::Actor(b)) => Self::Actor(Box::new(a.merge(b))),
            (Self::ActorInfo(a), Self::ActorInfo(b)) => Self::ActorInfo(Box::new(a.merge(b))),
            (Self::ActorLink(a), Self::ActorLink(b)) => Self::ActorLink(Box::new(a.merge(b))),
            (Self::AIProgram(a), Self::AIProgram(b)) => Self::AIProgram(Box::new(a.merge(b))),
            (Self::AISchedule(a), Self::AISchedule(b)) => Self::AISchedule(Box::new(a.merge(b))),
            (Self::AnimationInfo(a), Self::AnimationInfo(b)) => {
                Self::AnimationInfo(Box::new(a.merge(b)))
            }
            (Self::AreaData(a), Self::AreaData(b)) => Self::AreaData(Box::new(a.merge(b))),
            (Self::AS(a), Self::AS(b)) => Self::AS(Box::new(a.merge(b))),
            (Self::ASList(a), Self::ASList(b)) => Self::ASList(Box::new(a.merge(b))),
            (Self::AttClient(a), Self::AttClient(b)) => Self::AttClient(Box::new(a.merge(b))),
            (Self::AttClientList(a), Self::AttClientList(b)) => {
                Self::AttClientList(Box::new(a.merge(b)))
            }
            (Self::Awareness(a), Self::Awareness(b)) => Self::Awareness(Box::new(a.merge(b))),
            (Self::BarslistInfo(a), Self::BarslistInfo(b)) => {
                Self::BarslistInfo(Box::new(a.merge(b)))
            }
            (Self::BoneControl(a), Self::BoneControl(b)) => Self::BoneControl(Box::new(a.merge(b))),
            (Self::Chemical(a), Self::Chemical(b)) => Self::Chemical(Box::new(a.merge(b))),
            (Self::ChemicalRes(a), Self::ChemicalRes(b)) => Self::ChemicalRes(Box::new(a.merge(b))),
            (Self::CookData(a), Self::CookData(b)) => Self::CookData(Box::new(a.merge(b))),
            (Self::DamageParam(a), Self::DamageParam(b)) => Self::DamageParam(Box::new(a.merge(b))),
            (Self::Demo(a), Self::Demo(b)) => Self::Demo(Box::new(a.merge(b))),
            (Self::DropTable(a), Self::DropTable(b)) => Self::DropTable(Box::new(a.merge(b))),
            (Self::EventInfo(a), Self::EventInfo(b)) => Self::EventInfo(Box::new(a.merge(b))),
            (Self::GameDataPack(a), Self::GameDataPack(b)) => {
                Self::GameDataPack(Box::new(a.merge(b)))
            }
            (Self::GeneralParamList(a), Self::GeneralParamList(b)) => {
                Self::GeneralParamList(Box::new(a.merge(b)))
            }
            (Self::LazyTraverseList(a), Self::LazyTraverseList(b)) => {
                Self::LazyTraverseList(Box::new(a.merge(b)))
            }
            (Self::LevelSensor(a), Self::LevelSensor(b)) => Self::LevelSensor(Box::new(a.merge(b))),
            (Self::LifeCondition(a), Self::LifeCondition(b)) => {
                Self::LifeCondition(Box::new(a.merge(b)))
            }
            (Self::Location(a), Self::Location(b)) => Self::Location(Box::new(a.merge(b))),
            (Self::Lod(a), Self::Lod(b)) => Self::Lod(Box::new(a.merge(b))),
            (Self::MapUnit(a), Self::MapUnit(b)) => Self::MapUnit(Box::new(a.merge(b))),
            (Self::ModelList(a), Self::ModelList(b)) => Self::ModelList(Box::new(a.merge(b))),
            (Self::Physics(a), Self::Physics(b)) => Self::Physics(Box::new(a.merge(b))),
            (Self::QuestProduct(a), Self::QuestProduct(b)) => {
                Self::QuestProduct(Box::new(a.merge(b)))
            }
            (Self::RagdollBlendWeight(a), Self::RagdollBlendWeight(b)) => {
                Self::RagdollBlendWeight(Box::new(a.merge(b)))
            }
            (Self::RagdollConfig(a), Self::RagdollConfig(b)) => {
                Self::RagdollConfig(Box::new(a.merge(b)))
            }
            (Self::RagdollConfigList(a), Self::RagdollConfigList(b)) => {
                Self::RagdollConfigList(Box::new(a.merge(b)))
            }
            (Self::Recipe(a), Self::Recipe(b)) => Self::Recipe(Box::new(a.merge(b))),
            (Self::ResidentActors(a), Self::ResidentActors(b)) => {
                Self::ResidentActors(Box::new(a.merge(b)))
            }
            (Self::ResidentEvents(a), Self::ResidentEvents(b)) => {
                Self::ResidentEvents(Box::new(a.merge(b)))
            }
            (Self::SaveDataPack(a), Self::SaveDataPack(b)) => {
                Self::SaveDataPack(Box::new(a.merge(b)))
            }
            (Self::ShopData(a), Self::ShopData(b)) => Self::ShopData(Box::new(a.merge(b))),
            (Self::ShopGameDataInfo(a), Self::ShopGameDataInfo(b)) => {
                Self::ShopGameDataInfo(Box::new(a.merge(b)))
            }
            (Self::Static(a), Self::Static(b)) => Self::Static(Box::new(a.merge(b))),
            (Self::StatusEffectList(a), Self::StatusEffectList(b)) => {
                Self::StatusEffectList(Box::new(a.merge(b)))
            }
            (Self::Tips(a), Self::Tips(b)) => Self::Tips(Box::new(a.merge(b))),
            (Self::UMii(a), Self::UMii(b)) => Self::UMii(Box::new(a.merge(b))),
            (Self::WorldInfo(a), Self::WorldInfo(b)) => Self::WorldInfo(Box::new(a.merge(b))),
            _ => panic!(
                "Tried to merge incompatible resources: {:?} and {:?}",
                &self, &diff
            ),
        }
    }
}

impl MergeableResource {
    pub fn into_binary(self, endian: Endian) -> Vec<u8> {
        match self {
            Self::Actor(v) => v.into_binary(endian),
            Self::ActorInfo(v) => v.into_binary(endian),
            Self::ActorLink(v) => v.into_binary(endian),
            Self::AIProgram(v) => v.into_binary(endian),
            Self::AISchedule(v) => v.into_binary(endian),
            Self::AnimationInfo(v) => v.into_binary(endian),
            Self::AreaData(v) => v.into_binary(endian),
            Self::AS(v) => v.into_binary(endian),
            Self::ASList(v) => v.into_binary(endian),
            Self::AttClient(v) => v.into_binary(endian),
            Self::AttClientList(v) => v.into_binary(endian),
            Self::Awareness(v) => v.into_binary(endian),
            Self::BarslistInfo(v) => v.into_binary(endian),
            Self::BoneControl(v) => v.into_binary(endian),
            Self::Chemical(v) => v.into_binary(endian),
            Self::ChemicalRes(v) => v.into_binary(endian),
            Self::CookData(v) => v.into_binary(endian),
            Self::DamageParam(v) => v.into_binary(endian),
            Self::Demo(v) => v.into_binary(endian),
            Self::DropTable(v) => v.into_binary(endian),
            Self::EventInfo(v) => v.into_binary(endian),
            Self::GameDataPack(v) => v.into_binary(endian),
            Self::GeneralParamList(v) => v.into_binary(endian),
            Self::LazyTraverseList(v) => v.into_binary(endian),
            Self::LevelSensor(v) => v.into_binary(endian),
            Self::LifeCondition(v) => v.into_binary(endian),
            Self::Location(v) => v.into_binary(endian),
            Self::Lod(v) => v.into_binary(endian),
            Self::MapUnit(v) => v.into_binary(endian),
            Self::ModelList(v) => v.into_binary(endian),
            Self::Physics(v) => v.into_binary(endian),
            Self::QuestProduct(v) => v.into_binary(endian),
            Self::RagdollBlendWeight(v) => v.into_binary(endian),
            Self::RagdollConfig(v) => v.into_binary(endian),
            Self::RagdollConfigList(v) => v.into_binary(endian),
            Self::Recipe(v) => v.into_binary(endian),
            Self::ResidentActors(v) => v.into_binary(endian),
            Self::ResidentEvents(v) => v.into_binary(endian),
            Self::SaveDataPack(v) => v.into_binary(endian),
            Self::ShopData(v) => v.into_binary(endian),
            Self::ShopGameDataInfo(v) => v.into_binary(endian),
            Self::Static(v) => v.into_binary(endian),
            Self::StatusEffectList(v) => v.into_binary(endian),
            Self::Tips(v) => v.into_binary(endian),
            Self::UMii(v) => v.into_binary(endian),
            Self::WorldInfo(v) => v.into_binary(endian),
            Self::GenericAamp(v) => v.to_binary(),
            Self::GenericByml(v) => v.to_binary(endian.into()),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct SarcMap(pub SortedDeleteMap<String, String>);

impl Mergeable for SarcMap {
    fn diff(&self, other: &Self) -> Self {
        Self(self.0.diff(&other.0))
    }

    fn merge(&self, diff: &Self) -> Self {
        Self(self.0.merge(&diff.0))
    }
}

impl SarcMap {
    pub fn to_binary(
        &self,
        endian: uk_content::prelude::Endian,
        resources: &BTreeMap<String, ResourceData>,
    ) -> Result<Vec<u8>> {
        let mut sarc = SarcWriter::new(endian.into());
        sarc.files = self
            .0
            .iter()
            .map(|(path, canon)| -> Result<(String, Vec<u8>)> {
                let resource = resources
                    .get(canon)
                    .with_context(|| jstr!("Missing resource for SARC: {&canon}"))?;
                let data = resource.to_binary(endian, resources)?;
                Ok((path.clone(), data))
            })
            .collect::<Result<_>>()?;
        Ok(sarc.to_binary())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryResource {
    Agnostic(Vec<u8>),
    Platform {
        wiiu: Option<Vec<u8>>,
        nx: Option<Vec<u8>>,
    },
}

impl Mergeable for BinaryResource {
    fn diff(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Agnostic(_), Self::Agnostic(b)) => Self::Agnostic(b.clone()),
            (Self::Platform { wiiu: u1, nx: nx1 }, Self::Platform { wiiu: u2, nx: nx2 }) => {
                Self::Platform {
                    wiiu: (u1 != u2).then(|| u2.as_ref().cloned()).flatten(),
                    nx: (nx1 != nx2).then(|| nx2.as_ref().cloned()).flatten(),
                }
            }
            _ => panic!("Attempted to diff incompatible binary resource types"),
        }
    }

    fn merge(&self, diff: &Self) -> Self {
        match (self, diff) {
            (Self::Agnostic(_), Self::Agnostic(b)) => Self::Agnostic(b.clone()),
            (Self::Platform { wiiu: u1, nx: nx1 }, Self::Platform { wiiu: u2, nx: nx2 }) => {
                Self::Platform {
                    wiiu: u2.as_ref().or(u1.as_ref()).cloned(),
                    nx: nx2.as_ref().or(nx1.as_ref()).cloned(),
                }
            }
            _ => panic!("Attempted to merge incompatible binary resource types"),
        }
    }
}

impl BinaryResource {
    pub fn to_binary(&self, endian: Endian) -> Result<Vec<u8>> {
        match self {
            BinaryResource::Agnostic(data) => Ok(data.clone()),
            BinaryResource::Platform { wiiu, nx } => match endian {
                Endian::Big => wiiu.as_ref().cloned(),
                Endian::Little => nx.as_ref().cloned(),
            }
            .context("Resource missing binary data for target platform"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResourceData {
    Binary(BinaryResource),
    Mergeable(crate::resource::MergeableResource),
    Sarc(SarcMap),
}

impl ResourceData {
    pub fn from_binary(name: impl AsRef<Path>, data: Vec<u8>) -> Result<Self> {
        let name = name.as_ref();
        let data = roead::yaz0::decompress_if(data)?;
        if Actor::path_matches(name) {
            Ok(Self::Mergeable(crate::resource::MergeableResource::Actor(
                Box::new(Actor::from_binary(&data)?),
            )))
        } else if ActorInfo::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::ActorInfo(Box::new(ActorInfo::from_binary(
                    &data,
                )?)),
            ))
        } else if ActorLink::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::ActorLink(Box::new(ActorLink::from_binary(
                    &data,
                )?)),
            ))
        } else if AIProgram::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::AIProgram(Box::new(AIProgram::from_binary(
                    &data,
                )?)),
            ))
        } else if AISchedule::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::AISchedule(Box::new(AISchedule::from_binary(
                    &data,
                )?)),
            ))
        } else if AnimationInfo::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::AnimationInfo(Box::new(
                    AnimationInfo::from_binary(&data)?,
                )),
            ))
        } else if AreaData::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::AreaData(Box::new(AreaData::from_binary(
                    &data,
                )?)),
            ))
        } else if AS::path_matches(name) {
            Ok(Self::Mergeable(crate::resource::MergeableResource::AS(
                Box::new(AS::from_binary(&data)?),
            )))
        } else if ASList::path_matches(name) {
            Ok(Self::Mergeable(crate::resource::MergeableResource::ASList(
                Box::new(ASList::from_binary(&data)?),
            )))
        } else if AttClient::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::AttClient(Box::new(AttClient::from_binary(
                    &data,
                )?)),
            ))
        } else if AttClientList::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::AttClientList(Box::new(
                    AttClientList::from_binary(&data)?,
                )),
            ))
        } else if Awareness::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::Awareness(Box::new(Awareness::from_binary(
                    &data,
                )?)),
            ))
        } else if BarslistInfo::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::BarslistInfo(Box::new(
                    BarslistInfo::from_binary(&data)?,
                )),
            ))
        } else if BoneControl::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::BoneControl(Box::new(
                    BoneControl::from_binary(&data)?,
                )),
            ))
        } else if Chemical::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::Chemical(Box::new(Chemical::from_binary(
                    &data,
                )?)),
            ))
        } else if ChemicalRes::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::ChemicalRes(Box::new(
                    ChemicalRes::from_binary(&data)?,
                )),
            ))
        } else if CookData::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::CookData(Box::new(CookData::from_binary(
                    &data,
                )?)),
            ))
        } else if DamageParam::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::DamageParam(Box::new(
                    DamageParam::from_binary(&data)?,
                )),
            ))
        } else if Demo::path_matches(name) {
            Ok(Self::Mergeable(crate::resource::MergeableResource::Demo(
                Box::new(Demo::from_binary(&data)?),
            )))
        } else if DropTable::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::DropTable(Box::new(DropTable::from_binary(
                    &data,
                )?)),
            ))
        } else if EventInfo::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::EventInfo(Box::new(EventInfo::from_binary(
                    &data,
                )?)),
            ))
        } else if GameDataPack::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::GameDataPack(Box::new(
                    GameDataPack::from_binary(&data)?,
                )),
            ))
        } else if GeneralParamList::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::GeneralParamList(Box::new(
                    GeneralParamList::from_binary(&data)?,
                )),
            ))
        } else if LazyTraverseList::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::LazyTraverseList(Box::new(
                    LazyTraverseList::from_binary(&data)?,
                )),
            ))
        } else if LevelSensor::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::LevelSensor(Box::new(
                    LevelSensor::from_binary(&data)?,
                )),
            ))
        } else if LifeCondition::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::LifeCondition(Box::new(
                    LifeCondition::from_binary(&data)?,
                )),
            ))
        } else if Location::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::Location(Box::new(Location::from_binary(
                    &data,
                )?)),
            ))
        } else if Lod::path_matches(name) {
            Ok(Self::Mergeable(crate::resource::MergeableResource::Lod(
                Box::new(Lod::from_binary(&data)?),
            )))
        } else if MapUnit::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::MapUnit(Box::new(MapUnit::from_binary(&data)?)),
            ))
        } else if ModelList::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::ModelList(Box::new(ModelList::from_binary(
                    &data,
                )?)),
            ))
        } else if Physics::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::Physics(Box::new(Physics::from_binary(&data)?)),
            ))
        } else if QuestProduct::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::QuestProduct(Box::new(
                    QuestProduct::from_binary(&data)?,
                )),
            ))
        } else if RagdollBlendWeight::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::RagdollBlendWeight(Box::new(
                    RagdollBlendWeight::from_binary(&data)?,
                )),
            ))
        } else if RagdollConfig::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::RagdollConfig(Box::new(
                    RagdollConfig::from_binary(&data)?,
                )),
            ))
        } else if RagdollConfigList::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::RagdollConfigList(Box::new(
                    RagdollConfigList::from_binary(&data)?,
                )),
            ))
        } else if Recipe::path_matches(name) {
            Ok(Self::Mergeable(crate::resource::MergeableResource::Recipe(
                Box::new(Recipe::from_binary(&data)?),
            )))
        } else if ResidentActors::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::ResidentActors(Box::new(
                    ResidentActors::from_binary(&data)?,
                )),
            ))
        } else if ResidentEvents::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::ResidentEvents(Box::new(
                    ResidentEvents::from_binary(&data)?,
                )),
            ))
        } else if SaveDataPack::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::SaveDataPack(Box::new(
                    SaveDataPack::from_binary(&data)?,
                )),
            ))
        } else if ShopData::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::ShopData(Box::new(ShopData::from_binary(
                    &data,
                )?)),
            ))
        } else if ShopGameDataInfo::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::ShopGameDataInfo(Box::new(
                    ShopGameDataInfo::from_binary(&data)?,
                )),
            ))
        } else if Static::path_matches(name) {
            Ok(Self::Mergeable(crate::resource::MergeableResource::Static(
                Box::new(Static::from_binary(&data)?),
            )))
        } else if StatusEffectList::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::StatusEffectList(Box::new(
                    StatusEffectList::from_binary(&data)?,
                )),
            ))
        } else if Tips::path_matches(name) {
            Ok(Self::Mergeable(crate::resource::MergeableResource::Tips(
                Box::new(Tips::from_binary(&data)?),
            )))
        } else if UMii::path_matches(name) {
            Ok(Self::Mergeable(crate::resource::MergeableResource::UMii(
                Box::new(UMii::from_binary(&data)?),
            )))
        } else if WorldInfo::path_matches(name) {
            Ok(Self::Mergeable(
                crate::resource::MergeableResource::WorldInfo(Box::new(WorldInfo::from_binary(
                    &data,
                )?)),
            ))
        } else if data.len() > 4 && &data[0..4] == b"AAMP" {
            Ok(Self::Binary(BinaryResource::Agnostic(data.into())))
        } else if data.len() > 2 && (&data[0..2] == b"BY" || &data[0..2] == b"YB") {
            Ok(Self::Binary(BinaryResource::Platform {
                wiiu: (&data[0..2] == b"BY").then(|| data.clone().into()),
                nx: (&data[0..2] == b"YB").then(|| data.into()),
            }))
        } else {
            todo!()
        }
    }

    pub fn to_binary(
        &self,
        endian: Endian,
        resources: &BTreeMap<String, ResourceData>,
    ) -> Result<Vec<u8>> {
        Ok(match self {
            ResourceData::Binary(data) => data.to_binary(endian)?,
            ResourceData::Mergeable(resource) => resource.clone().into_binary(endian),
            ResourceData::Sarc(sarc) => sarc.to_binary(endian, resources)?,
        })
    }
}
