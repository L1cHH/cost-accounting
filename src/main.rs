pub mod pages;
mod user;
mod styles;
pub mod custom_widgets;
mod income;
mod expense;
mod error;

use std::env;
use std::env::current_dir;
use dotenv;
use anyhow::{Result};
use iced::{Application, Command, Element, Renderer, Settings, Theme, Length, Padding, Color, Alignment};
use iced::Alignment::Center;
use iced::widget::{container, text, column, text_input, Text, button, row, Svg, Component, Space};
use sqlx::{mysql, MySql, Pool, Row, Error};
use once_cell::sync::OnceCell;
use crate::custom_widgets::exit_button_widget::ExitButton;
use crate::custom_widgets::hyperlink_widget::Hyperlink;
use crate::custom_widgets::modal_window::Modal;
use crate::error::QueryError;
use crate::expense::Expense;
use crate::income::Income;
use crate::pages::{Page};
use crate::pages::registration_login_page::{Login, LoginError, RegistrationError, is_password_relevant};
use crate::user::User;
use crate::pages::notes_page::{Notes, NotesCategory};
use crate::pages::notes_page::InputError::IncorrectFormat;
use crate::styles::notes_styling::{CategoryContainer, CorrectTextInputStyle, ErrorTextInputStyle, NotesContainer, TestContainer};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
   dotenv::from_filename("file.env").expect("cant find env file");


   FinanceApp::run(Settings::default())?;


   Ok(())
}


static POOL: OnceCell<Pool<MySql>> = OnceCell::new();

pub struct FinanceApp {
   page: Page,
   user: Option<User>,
   data_base_error: Option<DataBaseError>,
   query_error: Option<QueryError>,
}

#[derive(Debug, Clone)]
pub enum Message {
   ConnectToDB(Result<Pool<MySql>, DataBaseError>),

   //Input Messages
   LoginChanged(String),
   PasswordChanged(String),
   RepeatPasswordChanged(String),

   //Registration Page
   TryToCreateUser(Result<bool, QueryError>),
   UserCreated(Result<(), QueryError>),
   SignUp,
   ToLoginPage,

   //Login Page
   ToRegistrationPage,
   LogIn,
   LoggedIn(Result<User, QueryError>),

   //NotesPage
   ChangeCategory(NotesCategory),
   TryToExit,
   ExitAccepted,
   ExitNotAccepted,
   ProfileNameChanged(String),
   ProfileSurnameChanged(String),
   SaveChangedName,
   SaveChangedSurname,
   LoadIncomes(Result<Vec<Income>, QueryError>),
   LoadExpenses(Result<Vec<Expense>, QueryError>)
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
            Login::new_registration_page()
         ),
         user: None,
         data_base_error: None,
         query_error: None,
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
                  _ = login.check_password();
                  Command::none()
               }

               Message::RepeatPasswordChanged(repeated_password_field) => {
                  login.change_repeated_password(repeated_password_field);
                  _ = login.check_password();
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
                  if login.check_password() {
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
                        self.query_error = Some(error);
                        println!("Error");
                        Command::none()
                     }
                  }

               }

               Message::UserCreated(Ok(())) => {
                  login.registered();
                  self.page = Page::NotesPage(Notes::new());
                  Command::none()
               }

               Message::ToLoginPage => {
                  self.page = Page::LoginPage(Login::new_login_page());
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

               Message::ToRegistrationPage => {
                  self.page = Page::RegistrationPage(Login::new_registration_page());
                  Command::none()
               },

               Message::LogIn => {
                  Command::perform(log_in(POOL.get().unwrap(), login.get_login().to_string(), login.get_password().to_string()), Message::LoggedIn)
               },

               Message::LoggedIn(Ok(user)) => {
                  self.user = Some(user);
                  login.set_login_error(None);
                  self.page = Page::NotesPage(Notes::new());

                  //Need to go on another page and try to load saved incomes and expenses
                  Command::batch(vec![
                     Command::perform(load_incomes(POOL.get().unwrap(), self.user.as_ref().unwrap().get_id()), Message::LoadIncomes),
                     Command::perform(load_expenses(POOL.get().unwrap(), self.user.as_ref().unwrap().get_id()), Message::LoadExpenses)
                  ])
               },

               Message::LoggedIn(Err(err)) => {
                  if err == QueryError::NoResultFound {
                     login.set_login_error(Some(LoginError::WrongPasswordOrLogin));
                     eprintln!("Cant log in. Wrong login or password");
                  }
                  eprintln!("Cant log in. There are some errors (not include wrong password-login )");
                  Command::none()
               }

               _ => {Command::none()}
            }
         }
         Page::NotesPage(notes_page) => {
            match message {
               Message::ChangeCategory(new_category) => {

                  if notes_page.current_category == new_category {
                     Command::none()
                  } else {
                     notes_page.current_category = new_category;
                     Command::none()
                  }

               },

               Message::TryToExit => {
                  notes_page.show_modal = true;
                  Command::none()
               }

               Message::ExitAccepted => {
                  notes_page.show_modal = false;
                  self.user = None;
                  self.page = Page::LoginPage(Login::new_login_page());
                  Command::none()
               }

               Message::ExitNotAccepted => {
                  notes_page.show_modal = false;
                  Command::none()
               }

               Message::ProfileNameChanged(name) => {
                  notes_page.profile_name_input = name;

                  match correct_format(notes_page.profile_name_input.as_ref()) {
                     true => notes_page.name_input_error = None,
                     false => notes_page.name_input_error = Some(IncorrectFormat)
                  }

                  Command::none()
               }

               Message::ProfileSurnameChanged(surname) => {
                  notes_page.profile_surname_input = surname;

                  match correct_format(notes_page.profile_surname_input.as_ref()) {
                     true => notes_page.surname_input_error = None,
                     false => notes_page.surname_input_error = Some(IncorrectFormat)
                  }

                  Command::none()
               }

               Message::SaveChangedName => {
                  match notes_page.name_input_error {
                     Some(_) => {},
                     None => {
                        if notes_page.profile_name_input.len() > 0 {
                           self.user.as_mut().unwrap().first_name = Some(notes_page.profile_name_input.clone())
                        }
                     }
                  }

                  Command::none()
               }

               Message::SaveChangedSurname => {

                  match notes_page.surname_input_error {
                     Some(_) => {},
                     None => {
                        if notes_page.profile_surname_input.len() > 0 {
                           self.user.as_mut().unwrap().last_name = Some(notes_page.profile_surname_input.clone())
                        }
                     }
                  }
                  Command::none()
               }

               Message::LoadIncomes(Ok(incomes)) => {
                  self.user.as_mut().unwrap().add_incomes_to_user(incomes);
                  Command::none()
               }

               Message::LoadExpenses(Ok(expenses)) => {
                  self.user.as_mut().unwrap().add_expenses_to_user(expenses);
                  Command::none()
               }

               _ => Command::none()

            }
         }
      }
   }

   fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
      match &self.page {
         Page::RegistrationPage(login) => {
            let registration_text:Text<'_, Self::Theme, Renderer> = text("Регистрация").size(50);

            let login_input = text_input("Your e-mail...", &login.get_login()).width(500).padding(10).size(20).on_input(Message::LoginChanged);
            let password_input = text_input("Create password...", &login.get_password()).width(500).padding(10).size(20).on_input(Message::PasswordChanged);
            let repeat_password_input = text_input("Repeat password", &login.get_repeated_password()).width(500).padding(10).size(20).on_input(Message::RepeatPasswordChanged);
            let registration_btn = button("Зарегистрироваться").padding(10).on_press(Message::SignUp);
            let to_login_page_btn = button("Авторизация").padding(10).on_press(Message::ToLoginPage);


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
                        text("Ошибка: Пользователь с таким логином уже существует").size(20)
                     }
                  }
               }
               None => {
                  match login.is_registered() {
                     true => text("Регистрация прошла успешно. Гуд лак хэв фан"),
                     false => match is_password_relevant(login.get_password()) {
                        false => text("Придумайте пароль..."),
                        true => text("Пароль соответствует требованиям...Нажмите на кнопку")
                     }
                  }
               }

            };

            let content = container(column![
               registration_text,
               login_input,
               password_input,
               repeat_password_input,
               error_text,
               row![registration_btn, to_login_page_btn].align_items(Center).spacing(15)

            ].align_items(Center).spacing(20)).width(600).height(800).center_x().center_y();

            container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
         }
         Page::LoginPage(login) => {
            let login_text = text("Вход").size(50);

            let login_input = text_input("Your e-mail...", &login.get_login()).width(500).padding(10).size(20).on_input(Message::LoginChanged);
            let password_input = text_input("Create password...", &login.get_password()).width(500).padding(10).size(20).on_input(Message::PasswordChanged);

            let login_btn = button("Войти").padding(10).on_press(Message::LogIn);
            let to_registration_btn = button("Регистрация").padding(10).on_press(Message::ToRegistrationPage);
            let err_text = match login.get_login_error() {
               Some(err) =>
                  match err {
                     LoginError::WrongPasswordOrLogin => text("Неверный логин или пароль. Если нет учетной записи - зарегистрируйтесь").size(20),
                  }
               None => {
                  match &self.user {
                     Some(_) => text("Вы успешно зарегистрировались!").size(20),
                     None => text("Введите логин и пароль...").size(20)
                  }
               }
            };
            let content = container(column![
               login_text,
               login_input,
               password_input,
               err_text,
               row![login_btn, to_registration_btn].align_items(Center).spacing(15)
            ].align_items(Center).spacing(20)).width(600).height(800).center_x().center_y();

            container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
         }
         Page::NotesPage(notes_page) => {

            let mut icons_dir = current_dir().unwrap();

            //PROFILE SVG
            let mut user_svg_path = icons_dir.clone();
            user_svg_path.push("src\\icons\\user.svg");
            let user_svg = Svg::from_path(user_svg_path).height(50);

            //EXIT BUTTON VIEW IN ACTIVE MODE
            let mut active_exit_path = icons_dir.clone();
            active_exit_path.push("src\\icons\\active_exit.svg");

            //EXIT BUTTON VIEW IN HOVERED MODE
            let mut hovered_exit_path = icons_dir.clone();
            hovered_exit_path.push("src\\icons\\hovered_exit.svg");

            //EXIT BUTTON VIEW IN CLICKED MODE
            let mut clicked_exit_path = icons_dir.clone();
            clicked_exit_path.push("src\\icons\\clicked_exit.svg");

            //ACCEPT BUTTON VIEW IN ACTIVE MODE
            let mut active_accept_path = icons_dir.clone();
            active_accept_path.push("src\\icons\\active_accept.svg");

            //ACCEPT BUTTON VIEW IN HOVERED MODE
            let mut hovered_accept_path = icons_dir.clone();
            hovered_accept_path.push("src\\icons\\hovered_accept.svg");

            //ACCEPT BUTTON VIEW IN CLICKED MODE
            let mut clicked_accept_path = icons_dir.clone();
            clicked_accept_path.push("src\\icons\\clicked_accept.svg");

            let notes_section = match notes_page.current_category{

               NotesCategory::MyProfile => {
                  let user = self.user.as_ref().unwrap();

                  let person_name = match user.first_name.as_ref() {
                     Some(name) => text(format!("{name}")).size(20),
                     None => text("Неизвестно").size(20)
                  };

                  let person_last_name = match user.last_name.as_ref() {
                     Some(last_name) => text(format!("{last_name}")).size(20),
                     None => text("Неизвестно").size(20)
                  };

                  let person_login = &user.nickname;

                  let welcome_section = row![
                     Space::with_width(Length::FillPortion(2)),
                     text("Ваш профиль").size(20).width(Length::FillPortion(1)),
                     Space::with_width(Length::FillPortion(2))
                  ];


                  let name_section = row![
                     Space::with_width(Length::FillPortion(1)),
                     person_last_name.clone().width(Length::FillPortion(2)),
                     person_name.clone().width(Length::FillPortion(2)),
                     Space::with_width(Length::FillPortion(12)),
                  ];

                  let login_section = row![
                     Space::with_width(Length::FillPortion(1)),
                     text(person_login).width(Length::FillPortion(3)),
                     Space::with_width(Length::FillPortion(6)),
                  ];

                  let edit_section = row![
                     Space::with_width(Length::FillPortion(2)),
                     text("Редактировать").size(20).width(Length::FillPortion(1)),
                     Space::with_width(Length::FillPortion(2))
                  ];

                  let edit_name = row![
                     Space::with_width(Length::FillPortion(1)),
                     text("Ваше Имя:").width(Length::FillPortion(1)),
                     person_name.width(Length::FillPortion(1)),
                     Space::with_width(Length::FillPortion(1)),
                     text("Новое Имя:").width(Length::FillPortion(1)),
                     text_input("Имя", &notes_page.profile_name_input)
                        .style(iced::theme::TextInput::Custom(
                        match notes_page.name_input_error.as_ref() {
                           Some(_) => {
                              Box::new(ErrorTextInputStyle)
                           },
                           None => {
                              Box::new(CorrectTextInputStyle)
                           }
                        })).on_input(Message::ProfileNameChanged),
                     container(ExitButton::new(active_accept_path.clone(), hovered_accept_path.clone(), clicked_accept_path.clone(), |()| Message::SaveChangedName)).width(Length::FillPortion(1)),
                     Space::with_width(Length::FillPortion(1)),
                  ].align_items(Center);

                  let edit_surname = row![
                     Space::with_width(Length::FillPortion(1)),
                     text("Ваша Фамилия:").width(Length::FillPortion(1)),
                     person_last_name.width(Length::FillPortion(1)),
                     Space::with_width(Length::FillPortion(1)),
                     text("Новая Фамилия:").width(Length::FillPortion(1)),
                     text_input("Фамилия", &notes_page.profile_surname_input).style(iced::theme::TextInput::Custom(
                        match notes_page.surname_input_error.as_ref() {
                           Some(_) => {
                              Box::new(ErrorTextInputStyle)
                           },
                           None => {
                              Box::new(CorrectTextInputStyle)
                           }
                        })).on_input(Message::ProfileSurnameChanged),
                     container(ExitButton::new(active_accept_path, hovered_accept_path, clicked_accept_path, |()| Message::SaveChangedSurname)).width(Length::FillPortion(1)),
                     Space::with_width(Length::FillPortion(1)),
                  ].align_items(Center);



                  container(column![
                     welcome_section.padding(Padding::from([0, 0, 40, 0])),
                     name_section.padding(Padding::from([0, 0, 35, 0])),
                     login_section.padding(Padding::from([0, 0, 90, 0])),
                     edit_section.padding(Padding::from([0, 0, 30, 0])),
                     edit_name.padding(Padding::from([0, 0, 10, 0])),
                     edit_surname,
                  ])
                      .width(Length::FillPortion(4))
                      .height(Length::Fill)
                      .padding(Padding::from([50, 0]))
                      .style(iced::theme::Container::Custom(Box::new(NotesContainer)))
               }

               NotesCategory::IncomesState => {

                  let incomes_text = text("Here will be incomes..").size(20);


                  container(incomes_text)
                     .center_x()
                     .width(Length::FillPortion(4))
                     .height(Length::Fill)
                     .style(iced::theme::Container::Custom(Box::new(NotesContainer)))
               },

               NotesCategory::ExpensesState => {

                  let expenses_text = text("Here will be expenses..").size(20);

                  container(expenses_text)
                      .center_x()
                      .width(Length::FillPortion(4))
                      .height(Length::Fill)
                      .style(iced::theme::Container::Custom(Box::new(NotesContainer)))
               }

            };

            let my_profile_category = Hyperlink::new("Профиль".to_string(), NotesCategory::MyProfile, |category| Message::ChangeCategory(category));
            let incomes_category = Hyperlink::new("Доходы".to_string(), NotesCategory::IncomesState, |category| Message::ChangeCategory(category));
            let expenses_category = Hyperlink::new("Расходы".to_string(), NotesCategory::ExpensesState, |category| Message::ChangeCategory(category));

            let exit_btn = ExitButton::new(active_exit_path, hovered_exit_path, clicked_exit_path, |()| Message::TryToExit);

            //User SVG
            let user_image = row![
               Space::with_width(Length::FillPortion(1)),
               container(user_svg).width(Length::FillPortion(1)),
               Space::with_width(Length::FillPortion(1))
            ].padding(Padding::from([0, 0, 20, 0]));

            //User Profile
            let user_info = row![
               Space::with_width(Length::FillPortion(1)),
               container(my_profile_category).width(Length::FillPortion(2)).padding(Padding::from([0, 20, 0, 0])),
               container(exit_btn).width(Length::FillPortion(2)),
               Space::with_width(Length::FillPortion(1))
            ].align_items(Center).spacing(10).padding(Padding::from([0, 0, 40, 0]));

            //TEXT "РАЗДЕЛ"
            let choice_text = row![
               Space::with_width(Length::FillPortion(1)),
               container(text("Разделы").style(iced::theme::Text::Color(Color::from_rgba8(96, 83, 90, 0.8))).size(23)).width(Length::FillPortion(1)),
               Space::with_width(Length::FillPortion(1))
            ];

            //Категории
            let categories_menu = row![
               container(column![incomes_category, expenses_category].align_items(Alignment::Start)).width(Length::FillPortion(1)),
               Space::with_width(Length::FillPortion(2))
            ].padding(Padding::from([10, 0, 0, 10]));



            let category_section = container(
               column![
                  user_image,
                  user_info,
                  choice_text,
                  categories_menu
               ].width(Length::Fill)
            )
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .padding(Padding::from([20, 0]))
                .style(iced::theme::Container::Custom(Box::new(CategoryContainer)));

            let final_container = container(row![notes_section, category_section].align_items(Center).spacing(15)).padding(20);
            let modal_window = container(
               column![
                  text("Вы точно хотите выйти?").size(20),
                  row![
                     button("Вернуться").padding(5).on_press(Message::ExitNotAccepted),
                     button("Выйти").padding(5).on_press(Message::ExitAccepted)
                  ].align_items(Alignment::Center).spacing(15)
               ].align_items(Center).spacing(15)
            ).padding(30).style(iced::theme::Container::Custom(Box::new(CategoryContainer)));

            match notes_page.show_modal {
               false => {
                  final_container.into()
               }

               true => {
                  Modal::new(final_container, modal_window).on_blur(Message::ExitNotAccepted).into()
               }
            }


         }
      }
   }
}

fn correct_format(input: &str) -> bool {
   input.chars().all(|c| (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') && (input.len() > 0))
}


async fn connect_to_db() -> Result<Pool<MySql>, DataBaseError> {
   let pool = mysql::MySqlPool::connect(&env::var("DATABASE_URL").expect("Need to set env. variable 'DATABASE_URL'..."))
       .await
       .map_err(|_| DataBaseError::DataBaseConnectionErr)?;
   Ok(pool)
}
async fn is_user_exists(pool: &Pool<MySql>, login: String) -> Result<bool, QueryError> {
   let user = sqlx::query(
      r#"
      SELECT Count(user_id) as count from Пользователь
      Where NICKNAME = ( ? ) limit 1
      "#
   ).bind(login).fetch_one(pool)
       .await
       .map_err(|err| {
          QueryError::match_sqlx_error(err)
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
async fn add_user(pool: &Pool<MySql>, login: String, password: String) -> Result<(), QueryError> {
   let user_id = sqlx::query(&format!(
      r#"
      INSERT INTO ПОЛЬЗОВАТЕЛЬ (NICKNAME, PASSWORD)
      VALUES ('{login}', '{password}')
      "#
   )).execute(pool)
       .await
       .map_err(|err| {
          QueryError::match_sqlx_error(err)
       })?
       .last_insert_id();

   println!("{}", format!("User was added. ID: {user_id}"));

   Ok(())
}
async fn log_in(pool: &Pool<MySql>, login: String, password: String) -> Result<User, QueryError> {
   let user = sqlx::query(
      r#"
      SELECT user_id, last_name, first_name, nickname from Пользователь
      where nickname = ( ? ) and password = ( ? ) limit 1
      "#
   ).bind(login).bind(password).fetch_one(pool)
       .await
       .map_err(|err| {
          QueryError::match_sqlx_error(err)
       })?;

   let user_id: i32 = user.get("user_id");
   let last_name: Option<String> = user.get("last_name");
   let first_name: Option<String> = user.get("first_name");
   let nickname: String = user.get("nickname");

   Ok(User::new(user_id, last_name, first_name, nickname))
}
async fn load_incomes(pool: &Pool<MySql>, users_id: i32) -> Result<Vec<Income>, QueryError> {
   let incomes = sqlx::query(
      r#"
      SELECT * FROM ДОХОД
      WHERE INCOMES_CREATOR = ( ? )
      "#
   )
       .bind(users_id)
       .fetch_all(pool)
       .await
       .map_err(|err| {
          QueryError::match_sqlx_error(err)
       })?;

   Ok(Income::collect_from_query_vec(incomes))
}
async fn load_expenses(pool: &Pool<MySql>, users_ud: i32) -> Result<Vec<Expense>, QueryError> {
   let expenses = sqlx::query(
      r#"
      SELECT * FROM РАСХОД
      WHERE EXPENSES_CREATOR = ( ? )
      "#
   )
       .bind(users_ud)
       .fetch_all(pool)
       .await
       .map_err(|err| {
          QueryError::match_sqlx_error(err)
       })?;

   Ok(Expense::collect_from_query_vec(expenses))
}


