// src/network/server.rs
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use crate::game::player::Player;
use crate::game::quiz::{Quiz, Question};

pub struct GameServer {
    players: Arc<Mutex<HashMap<String, Player>>>,
    quiz: Quiz,
}

impl GameServer {
    pub fn new() -> Self {
        let quiz = Quiz::new(
            1,
            "Sample Quiz".to_string(),
            vec![
                Question {
                    id: 1,
                    text: "What is the capital of France?".to_string(),
                    options: vec![
                        "London".to_string(),
                        "Berlin".to_string(),
                        "Paris".to_string(),
                        "Madrid".to_string(),
                    ],
                    correct_answer: 2,
                },
                // Add more questions here
            ],
        );

        Self {
            players: Arc::new(Mutex::new(HashMap::new())),
            quiz,
        }
    }

    pub async fn run(&self, address: &str) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(address).await?;
        println!("Server listening on {}", address);

        let quiz = Arc::new(self.quiz.clone());

        loop {
            let (socket, addr) = listener.accept().await?;
            let players = Arc::clone(&self.players);
            let quiz = Arc::clone(&quiz);
            
            tokio::spawn(async move {
                if let Err(e) = GameServer::handle_connection(socket, addr, players, quiz).await {
                    eprintln!("Error handling connection: {}", e);
                }
            });
        }
    }

    async fn handle_connection(
        mut socket: TcpStream,
        addr: std::net::SocketAddr,
        players: Arc<Mutex<HashMap<String, Player>>>,
        quiz: Arc<Quiz>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = [0; 1024];

        // Read the player's name
        let n = socket.read(&mut buffer).await?;
        let name = String::from_utf8_lossy(&buffer[..n]).trim().to_string();

        let player_id = format!("{}", addr);
        let mut player = Player::new(player_id.clone(), name.clone(), addr);

        {
            let mut players = players.lock().await;
            players.insert(player_id.clone(), player.clone());
        }

        println!("New player connected: {} ({})", name, addr);

        // Send welcome message
        socket.write_all(format!("Welcome to RustyQuiz, {}!\n", name).as_bytes()).await?;

        // Main game loop for this player
        for (index, question) in quiz.questions.iter().enumerate() {
            // Send question
            let question_text = format!(
                "Question {}: {}\nOptions:\n{}",
                index + 1,
                question.text,
                question.options.iter().enumerate().map(|(i, opt)| format!("{}. {}", i + 1, opt)).collect::<Vec<_>>().join("\n")
            );
            socket.write_all(question_text.as_bytes()).await?;

            // Read answer
            let n = socket.read(&mut buffer).await?;
            let answer = String::from_utf8_lossy(&buffer[..n]).trim().parse::<usize>().unwrap_or(0) - 1;

            // Check answer and send result
            if quiz.check_answer(index, answer) {
                socket.write_all(b"Correct!\n").await?;
                player.add_score(1);
            } else {
                socket.write_all(b"Incorrect.\n").await?;
            }
        }

        // Send final score
        socket.write_all(format!("Quiz completed! Your final score is: {}\n", player.score).as_bytes()).await?;

        // Remove player from the game
        {
            let mut players = players.lock().await;
            players.remove(&player_id);
        }

        println!("Player disconnected: {} ({})", name, addr);

        Ok(())
    }
}