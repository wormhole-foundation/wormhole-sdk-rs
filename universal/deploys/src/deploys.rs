use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::CoreDeployment;

pub static MAINNET: &[&Lazy<CoreDeployment>] = &[&ETHEREUM];
pub static TESTNET: &[&Lazy<CoreDeployment>] = &[&GOERLI, &SEPOLIA];

pub static MAINNET_BY_ID: Lazy<HashMap<u16, &'static CoreDeployment>> = Lazy::new(|| {
    let mut mainnet: HashMap<u16, &'static CoreDeployment> = Default::default();

    for net in MAINNET {
        mainnet.insert(net.chain_id.to_u16(), net);
    }

    mainnet
});

pub static MAINNET_BY_NAME: Lazy<HashMap<String, &'static CoreDeployment>> = Lazy::new(|| {
    let mut mainnet: HashMap<String, &'static CoreDeployment> = Default::default();
    for net in MAINNET {
        mainnet.insert(net.name.to_owned(), *net);
        mainnet.insert(net.name.to_ascii_lowercase(), &*ETHEREUM);
    }

    // more here
    mainnet
});

pub static TESTNET_BY_ID: Lazy<HashMap<u16, &'static CoreDeployment>> = Lazy::new(|| {
    let mut testnet: HashMap<u16, &'static CoreDeployment> = Default::default();

    for net in TESTNET {
        testnet.insert(net.chain_id.to_u16(), net);
    }

    testnet
});

pub static TESTNET_BY_NAME: Lazy<HashMap<String, &'static CoreDeployment>> = Lazy::new(|| {
    let mut mainnet: HashMap<String, &'static CoreDeployment> = Default::default();
    for net in MAINNET {
        mainnet.insert(net.name.to_owned(), *net);
        mainnet.insert(net.name.to_ascii_lowercase(), &*ETHEREUM);
    }

    // more here
    mainnet
});

// TODO: deploy info

macro_rules! evm_net {
    ($name:ident {
        chain_id: $chain_id:literal,
        name: $name_prop:literal,
        aliases: [$($alias:literal),*],
        core_address: $core_address:literal,
        token_bridge_address: $token_bridge_address:literal,  nft_bridge_address: $nft_bridge_address:literal,
     }) => {
        pub static $name: Lazy<CoreDeployment> = Lazy::new(|| CoreDeployment {
            chain_id: $crate::ChainId::try_from($chain_id).unwrap(),
            name: $name_prop,
            core_address: hex::decode($core_address).unwrap(),
            token_bridge_address: Some(hex::decode($token_bridge_address).unwrap()),
            nft_bridge_address: Some(hex::decode($nft_bridge_address).unwrap()),
            vm: $crate::Vm::Evm,
        });
    };
}

evm_net!(ETHEREUM {
    chain_id: 2,
    name: "Ethereum",
    aliases: [],
    core_address: "0x98f3c9e6E3fAce36bAAd05FE09d375Ef1464288B",
    token_bridge_address: "0x3ee18B2214AFF97000D974cf647E7C347E8fa585",
    nft_bridge_address: "0x6FFd7EdE62328b3Af38FCD61461Bbfc52F5651fE",
});

evm_net!(GOERLI {
    chain_id: 2,
    name: "Goerli",
    aliases: ["Ethereum"],
    core_address: "0x706abc4E45D419950511e474C7B9Ed348A4a716c",
    token_bridge_address: "0xF890982f9310df57d00f659cf4fd87e65adEd8d7",
    nft_bridge_address: "0xD8E4C2DbDd2e2bd8F1336EA691dBFF6952B1a6eB",
});

evm_net!(SEPOLIA {
    chain_id: 2,
    name: "Sepolia",
    aliases: [],
    core_address: "0x4a8bc80Ed5a4067f1CCf107057b8270E0cC11A78",
    token_bridge_address: "0xDB5492265f6038831E89f495670FF909aDe94bd9",
    nft_bridge_address: "0x6a0B52ac198e4870e5F3797d5B403838a5bbFD99",
});
