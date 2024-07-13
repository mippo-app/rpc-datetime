use helper::fs::get_files;

// -> Result<(), Box<dyn std::error::Error>>
fn main() {
    {
        let protos = get_files("../../proto/rpc_datetime/", &"proto".into());

        tonic_build::configure()
            .build_client(true)
            .build_server(true)
            .out_dir("./src/pb/")
            .compile(&protos, &["../../proto/", "../../../typ-p/proto/"])
            .unwrap();
    }

    // Ok(())
}
