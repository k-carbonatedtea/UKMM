use crate::{
    prelude::*,
    util::{self, DeleteMap},
    Result, UKError,
};
use roead::aamp::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttClientList {
    pub att_pos: ParameterObject,
    pub att_clients: DeleteMap<String, String>,
}

impl TryFrom<&ParameterIO> for AttClientList {
    type Error = UKError;

    fn try_from(pio: &ParameterIO) -> Result<Self> {
        Ok(Self {
            att_pos: pio
                .object("AttPos")
                .ok_or(UKError::MissingAampKey(
                    "Attention client list missing AttPos",
                ))?
                .clone(),
            att_clients: pio
                .list("AttClients")
                .ok_or(UKError::MissingAampKey(
                    "Attention client list missing attention lists",
                ))?
                .objects
                .0
                .values()
                .map(|obj| -> Result<(String, String)> {
                    Ok((
                        obj.param("Name")
                            .ok_or(UKError::MissingAampKey(
                                "Attention client list client missing name",
                            ))?
                            .as_string64()?
                            .to_owned(),
                        obj.param("FileName")
                            .ok_or(UKError::MissingAampKey(
                                "Attention client list client missing filename",
                            ))?
                            .as_string64()?
                            .to_owned(),
                    ))
                })
                .collect::<Result<_>>()?,
        })
    }
}

impl From<AttClientList> for ParameterIO {
    fn from(val: AttClientList) -> Self {
        Self::new().with_object("AttPos", val.att_pos).with_list(
            "AttClients",
            ParameterList::new().with_objects(val.att_clients.into_iter().enumerate().map(
                |(i, (name, filename))| {
                    (
                        format!("AttClient_{}", i),
                        ParameterObject::new()
                            .with_param("Name", Parameter::String64(name))
                            .with_param("FileName", Parameter::String64(filename)),
                    )
                },
            )),
        )
    }
}

impl Mergeable<ParameterIO> for AttClientList {
    fn diff(&self, other: &Self) -> Self {
        Self {
            att_pos: util::diff_pobj(&self.att_pos, &other.att_pos),
            att_clients: self.att_clients.diff(&other.att_clients),
        }
    }

    fn merge(&self, diff: &Self) -> Self {
        Self {
            att_pos: util::merge_pobj(&self.att_pos, &diff.att_pos),
            att_clients: self.att_clients.merge(&diff.att_clients),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn serde() {
        let actor = crate::tests::test_base_actorpack("Enemy_Guardian_A");
        let pio = roead::aamp::ParameterIO::from_binary(
            actor
                .get_file_data("Actor/AttClientList/Guardian_A.batcllist")
                .unwrap(),
        )
        .unwrap();
        let atcllist = super::AttClientList::try_from(&pio).unwrap();
        let data = atcllist.clone().into_pio().to_binary();
        let pio2 = roead::aamp::ParameterIO::from_binary(&data).unwrap();
        let atcllist2 = super::AttClientList::try_from(&pio2).unwrap();
        assert_eq!(atcllist, atcllist2);
    }

    #[test]
    fn diff() {
        let actor = crate::tests::test_base_actorpack("Enemy_Guardian_A");
        let pio = roead::aamp::ParameterIO::from_binary(
            actor
                .get_file_data("Actor/AttClientList/Guardian_A.batcllist")
                .unwrap(),
        )
        .unwrap();
        let atcllist = super::AttClientList::try_from(&pio).unwrap();
        let actor2 = crate::tests::test_mod_actorpack("Enemy_Guardian_A");
        let pio2 = roead::aamp::ParameterIO::from_binary(
            actor2
                .get_file_data("Actor/AttClientList/Guardian_A.batcllist")
                .unwrap(),
        )
        .unwrap();
        let atcllist2 = super::AttClientList::try_from(&pio2).unwrap();
        let diff = atcllist.diff(&atcllist2);
        println!("{}", serde_json::to_string_pretty(&diff).unwrap());
    }

    #[test]
    fn merge() {
        let actor = crate::tests::test_base_actorpack("Enemy_Guardian_A");
        let pio = roead::aamp::ParameterIO::from_binary(
            actor
                .get_file_data("Actor/AttClientList/Guardian_A.batcllist")
                .unwrap(),
        )
        .unwrap();
        let actor2 = crate::tests::test_mod_actorpack("Enemy_Guardian_A");
        let atcllist = super::AttClientList::try_from(&pio).unwrap();
        let pio2 = roead::aamp::ParameterIO::from_binary(
            actor2
                .get_file_data("Actor/AttClientList/Guardian_A.batcllist")
                .unwrap(),
        )
        .unwrap();
        let atcllist2 = super::AttClientList::try_from(&pio2).unwrap();
        let diff = atcllist.diff(&atcllist2);
        let merged = atcllist.merge(&diff);
        assert_eq!(atcllist2, merged);
    }
}