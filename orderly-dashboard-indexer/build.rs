use ethers::contract::Abigen;

fn bindgen(contract_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let abi_path = format!("./abis/{}.json", contract_name);
    let bindings = Abigen::new(contract_name, abi_path)?.generate()?;

    bindings.write_to_file(format!("./src/bindings/{}.rs", contract_name))?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    bindgen("operator_manager")?;
    bindgen("user_ledger")?;

    Ok(())
}
