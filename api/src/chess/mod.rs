pub mod board;
pub mod chess_move;
pub mod game;
pub mod pieces;
pub mod service;
pub mod square;
tonic::include_proto!("chessgame"); // The string specified here must match the proto package name
