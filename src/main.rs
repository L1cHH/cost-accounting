pub mod pages;

use std::env;
use dotenv;
use anyhow::{Result};
use iced::{Application, Command, Element, Renderer, Settings, Theme, Length};
use iced::Alignment::Center;
use iced::widget::{container, text, column, text_input, Text, button};
use sqlx::{mysql, MySql, Pool, Row};
use once_cell::sync::OnceCell;
use crate::pages::{Page};
use crate::pages::registration_login_page::{Login, LoginError, RegistrationError};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
   dotenv::from_filename("file.env").expect("cant find th file");

   FinanceApp::run(Settings::default())?;


   Ok(())
}


static POOL: OnceCell<Pool<MySql>> = OnceCell::new();

struct FinanceApp {
   page: Page,
   data_base_error: Option<DataBaseError>
}

#[derive(Debug, Clone)]
enum Message {
   ConnectToDB(Result<Pool<MySql>, DataBaseError>),
   TryToCreateUser(Result<bool, DataBaseError>),
   UserCreated(Result<(), DataBaseError>),
   VerifyUser,

   LoginChanged(String),
   PasswordChanged(String),
   RepeatPasswordChanged(String),
   SignUp
}

#[derive(Clone, Debug)]
enum DataBaseError {
   DataBaseConnectionErr,
   GetUserErr,
   CreateUserErr,
}

impl Application for FinanceApp {
   type Executor = iced::executor::Default;
   type Flags = ();
   type Message = Message;
   type Theme = Theme;
   
   fn title(&self) -> String {
      "FinanceApp".to_string()
   }

   fn theme(&self) -> Self::Theme {
      Theme::Dracula
   }

   fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
      (Self {
         page: Page::RegistrationPage(
            Login::new()
         ),
         data_base_error: None
      },
      Command::perform(connect_to_db(), Message::ConnectToDB)
      )
   }
   fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
      match &mut self.page {
         Page::RegistrationPage(login) => {
            match message {
               Message::LoginChanged(login_field) => {
                  login.change_login(login_field);
                  Command::none()
               },

               Message::PasswordChanged(password_field) => {
                  login.change_password(password_field);
                  _ = login.verify_password();
                  Command::none()
               }

               Message::RepeatPasswordChanged(repeated_password_field) => {
                  login.change_repeated_password(repeated_password_field);
                  _ = login.verify_password();
                  Command::none()
               }

               Message::ConnectToDB(Ok(pool)) => {
                  POOL.set(pool).unwrap();
                  Command::none()
               },

               Message::ConnectToDB(Err(connection_err)) => {
                  self.data_base_error = Some(connection_err);
                  Command::none()
               }

               Message::SignUp => {
                  if login.verify_password() {
                     //Нужно проверить на существование данного логина и пароля
                     let str = login.get_login().to_string();
                     Command::perform(is_user_exists(POOL.get().unwrap(), str), Message::TryToCreateUser)
                  } else {
                     Command::none()
                  }
               }

               Message::TryToCreateUser(is_user_already_exists) => {

                  match is_user_already_exists {
                     Ok(is_exists) => match is_exists {
                        true => {
                           login.set_registration_error(Some(RegistrationError::UserAlreadyExists));
                           Command::none()
                        },
                        false => {
                           Command::perform(add_user(POOL.get().unwrap(), login.get_login().to_string(), login.get_password().to_string()), Message::UserCreated)
                        }
                     }
                     Err(error) => {
                        self.data_base_error = Some(error);
                        println!("Error");
                        Command::none()
                     }
                  }

               }

               Message::UserCreated(Err(err_with_adding)) => {
                  self.data_base_error = Some(err_with_adding);
                  Command::none()
               }

               _ => {Command::none()}
            }
         }

         Page::LoginPage(login) => {
            match message {
               Message::LoginChanged(login_field) => {
                  login.change_login(login_field);
                  Command::none()
               }

               Message::PasswordChanged(password) => {
                  login.change_password(password);
                  Command::none()
               },

               _ => {Command::none()}
            }
         }
      }
   }

   fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
      let content = match &self.page {
         Page::RegistrationPage(login) => {
            let registration_text:Text<'_, Self::Theme, Renderer> = text("Регистрация").size(50);

            let login_input = text_input("Your e-mail...", &login.get_login()).width(500).padding(10).size(20).on_input(Message::LoginChanged);
            let password_input = text_input("Create password...", &login.get_password()).width(500).padding(10).size(20).on_input(Message::PasswordChanged);
            let repeat_password_input = text_input("Repeat password", &login.get_repeated_password()).width(500).padding(10).size(20).on_input(Message::RepeatPasswordChanged);
            let registration_btn = button("Зарегистрироваться").padding(10).on_press(Message::SignUp);


            let error_text = match login.get_registration_error() {
               Some(err) => {
                  match err {
                     RegistrationError::IrrelevantPassword => {
                        text("Ошибка: Неправильный формат пароля, пароль должен быть минимум 8 символов в длину, а также должен содержать только символы латинского алфавита или числа[0-9]!").size(20)
                     },
                     RegistrationError::DifferentPasswords => {
                        text("Ошибка: Пароли не совпадают, проверьте еще раз").size(20)
                     },

                     RegistrationError::UserAlreadyExists => {
                        text("Ошибка: Пользователь с таким логином уже существует")
                     }
                  }
               }
               None => {
                  text("").size(20)
               }

            };

            container(column![
               registration_text,
               login_input,
               password_input,
               repeat_password_input,
               error_text,
               registration_btn,
            ].align_items(Center).spacing(20)).width(600).height(800).center_x().center_y()
         },
         Page::LoginPage(login) => {
            let login_text = text("Вход").size(50);

            let login_input = text_input("Your e-mail...", &login.get_login()).width(500).padding(10).size(20).on_input(Message::LoginChanged);
            let password_input = text_input("Create password...", &login.get_password()).width(500).padding(10).size(20).on_input(Message::PasswordChanged);

            container(column![
               login_text,
               login_input,
               password_input
            ].align_items(Center).spacing(20)).width(600).height(800).center_x().center_y()
         }
      };

      container(content)
          .width(Length::Fill)
          .height(Length::Fill)
          .center_x()
          .center_y()
          .into()
   }
}


async fn connect_to_db() -> Result<Pool<MySql>, DataBaseError> {
   let pool = mysql::MySqlPool::connect(&env::var("DATABASE_URL").expect("Need to set env. variable 'DATABASE_URL'..."))
       .await
       .map_err(|_| DataBaseError::DataBaseConnectionErr)?;
   Ok(pool)
}

async fn is_user_exists(pool: &Pool<MySql>, login: String) -> Result<bool, DataBaseError> {
   let user = sqlx::query(
      r#"
      SELECT Count(user_id) as count from Пользователь
      Where NICKNAME = ( ? ) limit 1
      "#
   ).bind(login).fetch_one(pool)
       .await
       .map_err(|_| {
          DataBaseError::GetUserErr
       })?;

   let users_count: i32 = user.get("count");

   match users_count {
      0 => {
         Ok(false)
      },
      _ => {
         Ok(true)
      }
   }

}

async fn add_user(pool: &Pool<MySql>, login: String, password: String) -> Result<(), DataBaseError> {
   let user = sqlx::query(&format!(
      r#"
      INSERT INTO ПОЛЬЗОВАТЕЛЬ (NICKNAME, PASSWORD)
      VALUES ('{login}', '{password}')
      "#
   )).execute(pool)
       .await
       .map_err(|_| DataBaseError::CreateUserErr)?
       .last_insert_id();

   println!("{}", format!("User was added. ID: {user}"));

   Ok(())
}

