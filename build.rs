use ext_php_rs_build::{ApiVersion, PHPInfo, emit_check_cfg, emit_php_cfg_flags, find_php};

fn main() -> anyhow::Result<()> {
    let php = find_php()?;
    let info = PHPInfo::get(&php)?;
    let version: ApiVersion = info.zend_version()?.try_into()?;

    emit_check_cfg();
    emit_php_cfg_flags(version);

    Ok(())
}
