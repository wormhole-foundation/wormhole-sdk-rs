#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum KnownChainIds {
    Unset = 0,
    Solana = 1,
    Ethereum = 2,
    Terra = 3,
    Bsc = 4,
    Polygon = 5,
    Avalanche = 6,
    Oasis = 7,
    Algorand = 8,
    Aurora = 9,
    Fantom = 10,
    Karura = 11,
    Acala = 12,
    Klaytn = 13,
    Celo = 14,
    Near = 15,
    Moonbeam = 16,
    Neon = 17,
    Terra2 = 18,
    Injective = 19,
    Osmosis = 20,
    Sui = 21,
    Aptos = 22,
    Arbitrum = 23,
    Optimism = 24,
    Gnosis = 25,
    Pythnet = 26,
    Xpla = 28,
    Btc = 29,
    Base = 30,
    Sei = 32,
    Rootstock = 33,
    Wormchain = 3104,
    Sepolia = 10002,
}

impl TryFrom<u16> for KnownChainIds {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        use KnownChainIds::*;
        match value {
            0 => Ok(Unset),
            1 => Ok(Solana),
            2 => Ok(Ethereum),
            3 => Ok(Terra),
            4 => Ok(Bsc),
            5 => Ok(Polygon),
            6 => Ok(Avalanche),
            7 => Ok(Oasis),
            8 => Ok(Algorand),
            9 => Ok(Aurora),
            10 => Ok(Fantom),
            11 => Ok(Karura),
            12 => Ok(Acala),
            13 => Ok(Klaytn),
            14 => Ok(Celo),
            15 => Ok(Near),
            16 => Ok(Moonbeam),
            17 => Ok(Neon),
            18 => Ok(Terra2),
            19 => Ok(Injective),
            20 => Ok(Osmosis),
            21 => Ok(Sui),
            22 => Ok(Aptos),
            23 => Ok(Arbitrum),
            24 => Ok(Optimism),
            25 => Ok(Gnosis),
            26 => Ok(Pythnet),
            28 => Ok(Xpla),
            29 => Ok(Btc),
            30 => Ok(Base),
            32 => Ok(Sei),
            33 => Ok(Rootstock),
            3104 => Ok(Wormchain),
            10002 => Ok(Sepolia),
            _ => Err(()),
        }
    }
}

pub enum ChainId {
    Known(KnownChainIds),
    Unknown(u16),
}

impl ChainId {
    pub fn to_u16(&self) -> u16 {
        match self {
            ChainId::Known(id) => *id as u16,
            ChainId::Unknown(id) => *id,
        }
    }
}

impl From<ChainId> for u16 {
    fn from(id: ChainId) -> Self {
        id.to_u16()
    }
}

impl From<u16> for ChainId {
    fn from(id: u16) -> Self {
        if let Ok(id) = KnownChainIds::try_from(id) {
            ChainId::Known(id)
        } else {
            ChainId::Unknown(id)
        }
    }
}
