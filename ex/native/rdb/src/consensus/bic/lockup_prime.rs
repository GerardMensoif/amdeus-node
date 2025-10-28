use std::panic::panic_any;
use crate::{bcat};
use crate::consensus::{bic::{coin::{balance,mint, to_flat}, epoch::TREASURY_DONATION_ADDRESS, lockup::{create_lock}}, consensus_kv::{kv_get, kv_increment, kv_put, kv_delete}};

pub fn call_lock(env: &mut crate::consensus::consensus_apply::ApplyEnv, args: Vec<Vec<u8>>) {
    if !crate::consensus::bic::coin::exists(env, b"PRIME") {
        kv_increment(env, &bcat(&[b"bic:coin:totalSupply:PRIME"]), 0);

        let mut admin = Vec::new();
        let v0 = &[149, 216, 55, 255, 29, 8, 239, 251, 139, 112, 30, 29, 199, 57, 90, 67, 198, 220, 101, 18, 228, 100, 100, 241, 43, 213, 221, 230, 253, 58, 231, 1, 102, 166, 54, 66, 245, 148, 140, 44, 78, 56, 84, 12, 222, 205, 57, 210];
        admin.push(v0.to_vec());
        let term_admins = crate::consensus::bic::eetf_list_of_binaries(admin).unwrap();
        kv_put(env, &bcat(&[b"bic:coin:permission:PRIME"]), &term_admins);

        kv_put(env, &bcat(&[b"bic:coin:mintable:PRIME"]), b"true");
        kv_put(env, &bcat(&[b"bic:coin:pausable:PRIME"]), b"true");
        kv_put(env, &bcat(&[b"bic:coin:soulbound:PRIME"]), b"true");
    }

    if args.len() != 2 { panic_any("invalid_args") }
    let amount = args[0].as_slice();
    let amount = std::str::from_utf8(&amount).ok().and_then(|s| s.parse::<i128>().ok()).unwrap_or_else(|| panic_any("invalid_amount"));
    let tier = args[1].as_slice();
    let (tier_epochs, multiplier) = match args.get(1).map(|v| v.as_slice()).unwrap_or_else(|| panic_any("invalid_tier")) {
        b"magic"   => (0, 1),
        b"magic2"   => (1, 1),
        b"7d"   => (10, 13),
        b"30d"  => (45, 17),
        b"90d"  => (135, 27),
        b"180d" => (270, 35),
        b"365d" => (547, 54),
        _ => panic_any("invalid_tier"),
    };

    if amount <= to_flat(1) { panic_any("invalid_amount") }
    if amount > balance(env, env.caller_env.account_caller.as_slice(), b"AMA") { panic_any("insufficient_funds") }
    kv_increment(env, &bcat(&[b"bic:coin:balance:", &env.caller_env.account_caller, b":AMA"]), -amount);

    let vault_index = kv_increment(env, &bcat(&[b"bic:lockup_prime:unique_index"]), 1);
    let vault_value = bcat(&[
        &tier,
        b"-", multiplier.to_string().as_bytes(),
        b"-", (env.caller_env.entry_epoch.saturating_add(tier_epochs)).to_string().as_bytes(),
        b"-", amount.to_string().as_bytes()]);
    kv_put(env, &bcat(&[b"bic:lockup_prime:vault:", &env.caller_env.account_caller, b":", vault_index.to_string().as_bytes()]), &vault_value);
}

pub fn call_unlock(env: &mut crate::consensus::consensus_apply::ApplyEnv, args: Vec<Vec<u8>>) {
    if args.len() != 1 { panic_any("invalid_args") }
    let vault_index = args[0].as_slice();

    let vault_key = &bcat(&[b"bic:lockup_prime:vault:", &env.caller_env.account_caller, b":", vault_index]);

    let vault = kv_get(env, vault_key);
    if vault.is_none() { panic_any("invalid_vault") }
    let vault = vault.unwrap();

    let vault_parts: Vec<Vec<u8>> = vault.split(|&b| b == b'-').map(|seg| seg.to_vec()).collect();
    let tier = vault_parts[0].as_slice();
    let multiplier = vault_parts[1].as_slice();
    let multiplier = std::str::from_utf8(&multiplier).ok().and_then(|s| s.parse::<u64>().ok()).unwrap_or_else(|| panic_any("invalid_multiplier"));
    let unlock_epoch = vault_parts[2].as_slice();
    let unlock_epoch = std::str::from_utf8(&unlock_epoch).ok().and_then(|s| s.parse::<u64>().ok()).unwrap_or_else(|| panic_any("invalid_unlock_epoch"));
    let unlock_amount = vault_parts[3].as_slice();
    let unlock_amount = std::str::from_utf8(&unlock_amount).ok().and_then(|s| s.parse::<u64>().ok()).unwrap_or_else(|| panic_any("invalid_unlock_amount"));

    if env.caller_env.entry_epoch < unlock_epoch {
        let penalty = unlock_amount / 4;
        let disbursement = unlock_amount - penalty;

        kv_increment(env, &bcat(&[b"bic:coin:balance:", TREASURY_DONATION_ADDRESS, b":AMA"]), penalty as i128);
        //Lockup for 5 epochs
        create_lock(env, env.caller_env.account_caller.to_vec().as_slice(), b"AMA", disbursement as i128, env.caller_env.entry_epoch.saturating_add(5));
    } else {
        let prime_points = unlock_amount * multiplier;
        mint(env, b"PRIME", prime_points as i128, env.caller_env.account_caller.to_vec().as_slice());
        kv_increment(env, &bcat(&[b"bic:coin:balance:", &env.caller_env.account_caller, b":AMA"]), unlock_amount as i128);
    }

    kv_delete(env, vault_key);
}

pub fn call_daily_checkin(env: &mut crate::consensus::consensus_apply::ApplyEnv, args: Vec<Vec<u8>>) {
    if args.len() != 1 { panic_any("invalid_args") }
    let vault_index = args[0].as_slice();

    let vault_key = &bcat(&[b"bic:lockup_prime:vault:", &env.caller_env.account_caller, b":", vault_index]);
    let vault = kv_get(env, vault_key);
    if vault.is_none() { panic_any("invalid_vault") }
    let vault = vault.unwrap();
    let vault_parts: Vec<Vec<u8>> = vault.split(|&b| b == b'-').map(|seg| seg.to_vec()).collect();
    let unlock_amount = vault_parts[3].as_slice();
    let unlock_amount = std::str::from_utf8(&unlock_amount).ok().and_then(|s| s.parse::<u64>().ok()).unwrap_or_else(|| panic_any("invalid_unlock_amount"));

    let next_checkin_epoch: u64 = kv_get(env, &bcat(&[b"bic:lockup_prime:next_checkin_epoch:", &env.caller_env.account_caller]))
        .map(|bytes| { std::str::from_utf8(&bytes).ok().and_then(|s| s.parse::<u64>().ok()).unwrap_or_else(|| panic_any("invalid_next_checkin_epoch")) })
        .unwrap_or(env.caller_env.entry_epoch);
    let delta = (env.caller_env.entry_epoch as i64) - (next_checkin_epoch as i64);
    if delta == 0 || delta == 1 {
        kv_put(env, &bcat(&[b"bic:lockup_prime:next_checkin_epoch:", &env.caller_env.account_caller]), env.caller_env.entry_epoch.saturating_add(2).to_string().as_bytes());

        let daily_bonus = unlock_amount / 100;
        mint(env, b"PRIME", daily_bonus as i128, env.caller_env.account_caller.to_vec().as_slice());

        let streak = kv_increment(env, &bcat(&[b"bic:lockup_prime:daily_streak:", &env.caller_env.account_caller]), 1);
        if streak >= 30 {
            kv_put(env, &bcat(&[b"bic:lockup_prime:daily_streak:", &env.caller_env.account_caller]), b"0");
            let streak_bonus = daily_bonus * 30;
            mint(env, b"PRIME", streak_bonus as i128, env.caller_env.account_caller.to_vec().as_slice());
        }
    } else if delta > 2 {
        kv_put(env, &bcat(&[b"bic:lockup_prime:next_checkin_epoch:", &env.caller_env.account_caller]), env.caller_env.entry_epoch.saturating_add(2).to_string().as_bytes());
        kv_put(env, &bcat(&[b"bic:lockup_prime:daily_streak:", &env.caller_env.account_caller]), b"0");
    } else {
        //already checked in for the day, 2 epoch window
    }
}
