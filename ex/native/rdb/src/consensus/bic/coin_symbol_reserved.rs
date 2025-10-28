#[inline]
fn ascii_upper(input: &[u8]) -> Vec<u8> {
    input.iter().map(|b| b.to_ascii_uppercase()).collect()
}

#[inline]
fn is_reserved_symbol(upper: &[u8]) -> bool {
    match upper {
        b"AMA" |
        b"BTC" | b"ETH" | b"USDT" | b"XRP" | b"BNB" | b"SOL" | b"USDC" | b"DOGE" | b"ADA" | b"TRX" | b"STETH" |
        b"WBTC" | b"SUI" | b"LINK" | b"AVAX" | b"XLM" | b"LEO" | b"USDS" | b"SHIB" | b"TON" | b"HBAR" | b"WSTETH" |
        b"BCH" | b"HYPE" | b"LTC" | b"DOT" | b"WETH" | b"BSC-USD" | b"BGB" | b"XMR" | b"USDE" | b"PI" | b"WBT" |
        b"CBBTC" | b"WEETH" | b"PEPE" | b"DAI" | b"APT" | b"SUSDS" | b"OKB" | b"TAO" | b"UNI" | b"NEAR" | b"BUIDL" |
        b"ONDO" | b"AAVE" | b"GT" | b"ETC" | b"ICP" | b"KAS" | b"MNT" | b"CRO" | b"TKX" | b"RENDER" | b"TRUMP" |
        b"VET" | b"USD1" | b"SUSDE" | b"POL" | b"LBTC" | b"ATOM" | b"FTN" | b"FET" | b"ALGO" | b"FIL" | b"S" |
        b"ENA" | b"JLP" | b"ARB" | b"TIA" | b"FDUSD" | b"SOLVBTC" | b"KCS" | b"BONK" | b"WLD" | b"MKR" | b"NEXO" |
        b"QNT" | b"JUP" | b"FLR" | b"STX" | b"BNSOL" | b"XDC" | b"OP" | b"EOS" | b"VIRTUAL" | b"FARTCOIN" | b"SEI" |
        b"RSETH" | b"USDT0" | b"IMX" | b"IP" | b"INJ" | b"PYUSD" | b"CRV" | b"GRT" | b"WBNB" | b"RETH" | b"DEXE" |
        b"XAUT" | b"JASMY" | b"RAY" | b"PAXG" | b"IOTA" | b"MSOL" | b"FLOKI" | b"CLBTC" | b"JUPSOL" | b"BSV" | b"LDO" |
        b"XSOLVBTC" | b"BTT" | b"THETA" | b"METH" | b"GALA" | b"SAND" | b"HNT" | b"CORE" | b"KAIA" | b"WAL" | b"LAYER" |
        b"USD0" | b"PENGU" | b"ENS" | b"CAKE" | b"USDX" | b"FLOW" | b"USDY" | b"EZETH" | b"XTZ" | b"ZEC" | b"WIF" |
        b"BRETT" | b"XCN" | b"MANA" | b"PENDLE" | b"USDC.E" | b"JTO" | b"AERO" | b"PYTH" | b"TEL" | b"UBTC" | b"RSR" |
        b"TUSD" | b"OSETH" | b"BTC.B" | b"SPX" | b"AR" | b"BDX" | b"AIOZ" | b"RUNE" | b"DYDX" | b"OUSG" | b"PUMPBTC" |
        b"KAVA" | b"EGLD" | b"TBTC" | b"DEEP" | b"XEC" | b"MOVE" | b"NFT" | b"NEO" | b"GRASS" | b"USYC" | b"STRK" |
        b"USDB" | b"OM" | b"APE" | b"SUPEROETH" | b"CMETH" | b"AXS" | b"BEAM" | b"CHZ" | b"MATIC" | b"CFX" | b"BERA" |
        b"W" | b"OHM" | b"POPCAT" | b"COMP" | b"EETH" | b"AKT" | b"JST" | b"MWC" | b"MORPHO" | b"PLUME" | b"RON" |
        b"SAROS" | b"SUN" | b"AXL" | b"CGETH.HASHKEY" | b"USDD" | b"AMP" | b"TWT" | b"TURBO" | b"BUSD" | b"LUNC" |
        b"FRAX" | b"RLUSD" | b"SUPER" | b"CTC" | b"CHEEMS" | b"KET" | b"WHYPE" | b"BERASTONE" | b"GNO" | b"VENOM" |
        b"WAVAX" | b"MINA" | b"AI16Z" | b"EBTC" | b"ZRO" | b"1INCH" | b"DASH" | b"USR" | b"DOG" | b"QGOLD" | b"ETHX" |
        b"HONEY" | b"MX" | b"SFP" | b"SAFE" | b"GLM" | b"SYRUPUSDC" | b"TFUEL" | b"ATH" | b"SNEK" | b"MEW" | b"CVX" |
        b"CWBTC" | b"CBETH" | b"KSM" | b"IBERA" | b"USDG" | b"SWETH" | b"GHO" | b"ZIL" | b"ABTC" | b"BTSE" | b"EIGEN" |
        b"NOT" | b"BLUR" | b"SNX" | b"EURC" | b"LSETH" | b"MOCA" | b"QTUM" | b"USDF" | b"VRSC" | b"FRXETH" | b"VTHO" |
        b"CKB" | b"ZRX" | b"MOG" | b"ARKM" | b"BAT" | b"KDA" | b"SAVAX" | b"BBSOL" | b"ZETA" | b"ASTR" | b"BABYDOGE" |
        b"GAS" | b"DCR" | b"DSOL" | b"USDA" | b"TRIP" | b"BABY" | b"TRAC" | b"STG" | b"CELO" | b"BORG" | b"ROSE" |
        b"ZK" | b"LPT" | b"KAITO" | b"STHYPE" | b"ANKR" | b"SYRUP" | b"CSPR" | b"CHEX" | b"AGENTFUN" | b"SC" | b"YFI" |
        b"ONE" | b"PRIME" | b"EUTBL" | b"GAMA" | b"UXLINK" | b"ELF" | b"DEUSD" | b"XYO" | b"DRIFT" | b"T" | b"GIGA" |
        b"ZANO" | b"HOT" | b"AIC" | b"RVN" | b"IOTX" | b"LVLUSD" | b"PNUT" | b"TETH" | b"HMSTR" | b"$RCGE" | b"POLYX" |
        b"CET" | b"XEM" | b"ETHW" | b"TOSHI" | b"SFRXETH" | b"XCH" | b"VANA" | b"SOS" | b"ORCA" | b"KOGE" |
        b"DGB" | b"FLUID" | b"QUBIC" | b"GOMINING" | b"CRVUSD" | b"MPLX" | b"WEMIX" | b"ORDI" | b"OSMO" | b"GMT" |
        b"KAU" | b"USDO" | b"EUL" | b"DLC" | b"TRIBE" | b"PUNDIX" | b"ALCH" | b"AIXBT" | b"EURS" | b"ZBCN" | b"SQD" |
        b"ME" | b"RLB" | b"CUSDO" | b"WBETH" | b"SWFTC" | b"MAG7.SSI" | b"WOO" | b"KULA" | b"DAG" | b"ILV" | b"ENJ" |
        b"GMX" | b"MELANIA" | b"BIGTIME" | b"ZEN" | b"COTI" | b"CONSCIOUS" | b"WMTX" | b"KUB" | b"ONT" | b"LCX" |
        b"GOHOME" | b"AMAPT" | b"CETUS" | b"STEAKUSDC" | b"ZKJ" | b"ACH" | b"STRAX" | b"NPC" | b"ETHFI" | b"KAG" |
        b"MEOW" | b"FAI" | b"SXP" | b"ASBNB" | b"SKL" | b"PWR" | b"STPT" | b"STS" | b"USDZ" | b"BAND" | b"XNO" |
        b"SUSHI" | b"SDEUSD" | b"PAAL" | b"LRC" | b"OPT" | b"B3" | b"ZIG" | b"COW" | b"LUNA" | b"HIVE" | b"ETH+" |
        b"OZO" | b"MYTH" | b"IO" | b"APFC" | b"NXM" | b"NKYC" | b"BDCA" | b"ARDR" | b"VVV" | b"GAL" |
        b"WAVES" | b"MASK" | b"DAKU" | b"BICO" | b"FLUX" | b"BIO" | b"VCNT" | b"VVS" | b"BORA" | b"INIT" | b"G" |
        b"REUSD" | b"ICX" | b"PROM" | b"ANIME" | b"CPOOL" | b"WFRAGSOL" | b"ABT" | b"AGI" | b"FBTC" | b"NTGL" |
        b"RED" | b"SIGN" | b"ZEUS" | b"QUSDT" | b"STBTC" | b"USDL" | b"USUAL" | b"XPR" | b"BOME" | b"PEAQ" | b"UMA" |
        b"KMNO" | b"XVS" | b"SGB" | b"SBTC" | b"SONIC" | b"METIS" | b"WELL" | b"OSAK" | b"ORBS" | b"XMW" | b"SUPRA" |
        b"W3S" | b"REQ" | b"FRXUSD" | b"OKT" | b"LSK" | b"AUDIO" | b"NEIRO" | b"ECOIN" | b"SSOL" | b"GCB" | b"ALT" |
        b"VELO" | b"AGIX" | b"ACT" | b"AEVO" | b"COREUM" | b"BLAST" | b"MVL" | b"POWR" | b"CGPT" | b"IQ" | b"ACX" |
        b"MANTA" | b"SPELL" | b"BMX" | b"IOST" | b"VEE" | b"SNT" | b"PUNDIAI" | b"ALEO" | b"ZRC" | b"XRD" | b"HEART" |
        b"AERGO" | b"CARV" | b"RPL" | b"UXP" | b"DYM" | b"SUSDA" | b"MEME" | b"ID" | b"TEMPLE" | b"API3" | b"CVC" |
        b"YGG" | b"AGLD" | b"SATS" | b"ONG" | b"USTBL" | b"WAXP" | b"ZENT" | b"DKA" | b"OETH" | b"GFI" | b"RLC" |
        b"SOLO" | b"ARK" | b"OMNI" | b"BONE" | b"ACRED" | b"CETH" | b"XVG" | b"H2O" | b"PHA" | b"REUSDC" | b"MLK" |
        b"NMD" | b"CSUSDL" | b"LON" | b"GLMR" | b"TNQ" | b"SOLVBTC.JUP" | b"ANDY" | b"FIDA" | b"TRB" | b"POND" |
        b"ROAM" | b"DOGS" | b"RSWETH" | b"RUSD" | b"DENT" | b"UNP" | b"AUCTION" | b"MED" | b"GEOD" | b"CFG" |
        b"STEEM" | b"PCI" | b"CTF" | b"JOE" | b"GOAT" | b"CHR" | b"PROMPT" | b"ELON" | b"USDP" | b"NOS" | b"PEOPLE" |
        b"COOKIE" | b"MIU" | b"AVA" | b"HOUSE" | b"AVAIL" | b"TAIKO" | b"NIL" | b"SAGA" | b"SLND" | b"NMR" | b"BAL" |
        b"0X0" | b"MTL" | b"IAG" | b"WCT" | b"PTGC" | b"SOLVBTC.CORE" | b"SLP" | b"DEGEN" | b"SCRT" | b"USTC" |
        b"EZSOL" | b"NILA" | b"SAAS" | b"PURR" | b"BUCK" | b"ANON" | b"BANANA" | b"LQTY" | b"AITECH" | b"SFRAX" |
        b"GAME" | b"DOLA" | b"GRIFFAIN" | b"DESO" | b"ACS" | b"DIA" | b"XAI" | b"BGSC" | b"BITCOIN" | b"B2M" |
        b"ERG" | b"OCEAN" | b"RLP" | b"TST" | b"OAS" | b"APU" | b"SHFL" | b"KNC" | b"SIREN" | b"ORAI" | b"PUFF" |
        b"AIAT" | b"FMC" | b"MOVR" | b"IGT" | b"OMI" | b"USUALX" | b"VANRY" | b"PONKE" | b"UDS" | b"ARC" | b"WOLF" |
        b"ISLM" | b"CTK" | b"MWETH" | b"SCR" | b"DEVVE" | b"MIM" | b"SOLV" | b"CX" | b"PZETH" | b"C98" | b"MAGIC" |
        b"SYN" | b"PIN" | b"CTSI" | b"TRUAPT" | b"BTU" | b"BERAETH" | b"QKC" | b"CELR" | b"WPOL" | b"GUSD" | b"QI" |
        b"HDX" | b"SLVLUSD" | b"ALI" | b"BNT" | b"AURORA" | b"WIN" | b"DF" | b"MNSRY" | b"AVUSD" | b"AO" | b"FUEL" |
        b"HPO" | b"REI" | b"HUNT" | b"CBK" | b"PEPECOIN" | b"ELG" | b"CCD" | b"FEUSD" | b"SKI" | b"ETHDYDX" |
        b"RARE" | b"CORGIAI" | b"WILD" | b"NMT" | b"HSK" | b"FUN" | b"JNFTC" | b"AXGT" | b"BB" | b"AGETH" | b"USD+" |
        b"USDM" | b"FIUSD" | b"ANT" | b"MEMEFI" | b"PARTI" | b"TRU" | b"HT" | b"TNSR" | b"SHELL" | b"EURCV" |
        b"SAVUSD" | b"SUNDOG" | b"CUDOS" | b"METFI" | b"WFCA" | b"TAI" | b"DBR" | b"CHILLGUY" | b"BANANAS31" |
        b"APEX" | b"SN" | b"MERL" | b"TOKEN" | b"CYBER" | b"KTA" | b"ZEREBRO" | b"AVL" | b"FWOG" | b"KEEP" |
        b"USD3" | b"OLAS" | b"METAL" | b"ZEDXION" | b"LADYS" | b"MBL" | b"WZRD" | b"LUSD" | b"RAIL" | b"RIF" |
        b"MOBILE" | b"SSV" | b"GNS" | b"SDEX" | b"STONKS" | b"REKT" | b"RSS3" | b"SILO" | b"CAT" | b"BAN" | b"BFC" |
        b"PRO" | b"ICE" | b"INF" | b"KUJI" | b"AUX" | b"STRIKE" | b"FORT" | b"AUKI" | b"QANX" | b"SFUND" | b"STORJ" |
        b"ANVL" | b"WSTUSR" | b"STMX" | b"OXT" | b"META" | b"EWT" | b"COQ" | b"EDGE" | b"AZERO" | b"MCDULL" |
        b"GORK" | b"EVER" | b"M87" | b"NYM" | b"OGN" | b"ALICE" | b"DEGO" | b"WXRP" | b"EKUBO" | b"MOODENG" |
        b"MBX" | b"SPA" | b"SKYAI" | b"STO" | b"RAD" | b"SYS" | b"NCT" | b"VINE" | b"DUSK" | b"BMT" | b"AMPL" |
        b"ROOT" | b"A8" | b"PRCL" | b"TT" | b"GODS" | b"LISUSD" | b"SCBTC" | b"CXO" | b"OBT" | b"GRND" | b"L3" |
        b"SOSO" | b"ACA" | b"UQC" | b"BINK" | b"NTRN" | b"HFUN" | b"ALPH" | b"XT" | b"GPS" | b"BC" | b"MNDE" |
        b"THAPT" | b"NEURAL" | b"TLOS" | b"HIFI" | b"MILK" | b"GEAR" | b"KYSOL" | b"DAO" | b"GNUS" | b"JELLYJELLY" |
        b"CUSD" | b"ALEX" | b"BOBA" | b"SWEAT" | b"CUSDC" | b"HBD" | b"SB" | b"SERAPH" | b"IXS" | b"QRL" | b"RACA" |
        b"EGGS" | b"FIS" | b"REG" | b"BIM" | b"MAV" | b"EDU" | b"RSC" | b"ASM" | b"DEP" | b"SLERF" | b"MIN" |
        b"REZ" | b"J" | b"ALPACA" | b"ZORA" | b"AINTI" | b"SURE" | b"TRUMATIC" | b"BAKE" | b"ACE" | b"HFT" | b"ERN" |
        b"TIBBIR" | b"XTER" | b"GPU" | b"XTUSD" | b"DRGN" | b"FORTH" | b"PLT" | b"SCRVUSD" | b"NFP" | b"EL" |
        b"VADER" | b"BROCCOLI" | b"AQT" | b"MATICX" | b"ORA" | b"DEFI" | b"DOGINME" | b"SUSD" | b"FCT" | b"ATA" |
        b"GRS" | b"BXN" | b"ANYONE" | b"SAUCE" | b"AIDOGE" | b"HOOK" | b"TORN" | b"NS" | b"TKP" | b"XPLA" | b"LAT" |
        b"MIGGLES" | b"DXI" | b"CLV" | b"PNG" | b"DG" | b"GUN" | b"MOC" | b"FROK" | b"HEGIC" | b"SPEC" | b"EFI" |
        b"TLM" | b"WIBE" | b"VRO" | b"LOKA" | b"SRX" | b"USDT.E" | b"A47" | b"TREE" | b"CORN" | b"HMT" | b"MUBARAK" |
        b"REX" | b"EPIC" | b"XION" | b"CLANKER" | b"MRB" | b"QUAI" | b"TROLL" | b"SD" | b"MAPO" | b"ASUSDF" | b"NEON" |
        b"ZCX" | b"LISTA" | b"GFAL" | b"BOLD" | b"ALPHA" | b"SEND" | b"SWARMS" | b"BMEX" | b"RDNT" | b"LOFI" |
        b"EURT" | b"PHB" | b"UNIETH" | b"CXT" | b"BURN" | b"KEYCAT" | b"HYPER" | b"BLUE" | b"CULT" | b"TRYB" |
        b"D" | b"HAEDAL" | b"OMG" | b"LHYPE" | b"SFRXUSD" | b"WNXM" | b"WHITE" | b"OGY" | b"LBT" | b"PEAS" | b"POKT" |
        b"CRTS" | b"B2" | b"MIMATIC" | b"MBOX" | b"SSUI" | b"$FARTBOY" | b"ETN" | b"OFT" | b"PIXEL" | b"NKN" |
        b"STRD" | b"GXC" | b"KERNEL" | b"SERV" | b"ELA" | b"SPECTRE" | b"MYRIA" | b"VICE" | b"EURE" | b"IBGT" |
        b"BEL" | b"VOW" | b"GOTS" | b"SX" | b"AHT" | b"CAM" | b"NAKA" | b"VITA" | b"CAW" | b"WEN" | b"BOUNTY" |
        b"ORDER" | b"OL" | b"HASHAI" | b"QUIL" => true,
        _ => false,
    }
}

#[inline]
pub fn is_free(symbol: &[u8], _caller: &[u8]) -> bool {
    let up = ascii_upper(symbol);
    if up.starts_with(b"AMA") { return false; }
    if up.starts_with(b"W") { return false; }
    !is_reserved_symbol(&up)
}
