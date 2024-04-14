
pub mod board;
pub mod chess_move;
pub mod game;
pub mod pieces;
pub mod square;
tonic::include_proto!("chess_server"); // The string specified here must match the proto package name
pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("chess_server_descriptor");
