mod pages;

use std::env;
use dotenv;
use anyhow::Result;
use iced::{Application, Command, Element, Renderer, Settings, Theme};
use sqlx::mysql;

#[tokio::main]
async fn main() -> Result<()> {
   dotenv::dotenv()?;
   let pool = mysql::MySqlPool::connect(&env::var("DATABASE_URL")?).await?;



}

struct FinanceApp {

}

enum Message {
   
}

impl Application for FinanceApp {
   type Executor = ();
   type Flags = ();
   type Message = Message;
   type Theme = Theme::Dracula;
   
   fn title(&self) -> String {
      "FinanceApp".to_string()
   }

   fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
      todo!()
   }

   fn run(settings: Settings<Self::Flags>) -> iced::Result where Self: 'static {
      todo!()
   }

   fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
      todo!()
   }

   fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
      todo!()
   }
}
