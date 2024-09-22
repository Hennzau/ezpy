fn main() -> eyre::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async {
        let mut uv = uv::UV::new().unwrap();
        println!("{:?}", uv.installed);
        let result = uv.install_bin().await;
        println!("{:?}", result);
        println!("{:?}", uv.installed);
    });

    Ok(())
}
