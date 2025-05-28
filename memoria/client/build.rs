fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .enum_attribute("value", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute(
            "LobbyReply",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "KeyAssignment",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "BoardState",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "CloseCards",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "BoardValue",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute("Card", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute(
            "ConnectionUpdate",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "PlayerMove",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "NextPlayer",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "GameStateReply",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .compile_protos(&["memoria.proto"], &["../proto"])?;
    Ok(())
}
