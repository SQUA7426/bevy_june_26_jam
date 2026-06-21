use bevy::prelude::*;

const MAIN_TEXT_COLOR: Color = Color::srgb(1., 1., 1.);

#[derive(Clone, Copy, Default, Eq, Debug, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Component, Resource, Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

#[derive(Component)]
pub struct Setting<T>(pub T);

#[derive(Component, Resource, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Volume(pub u32);
macro_rules! create_components {
    ($($name:ident),*) => ($(#[derive(Component)] struct $name;)*)
}

fn create_screen_node() -> Node {
    Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: percent(100),
        height: percent(100),
        ..default()
    }
}

#[derive(Debug)]
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DisplayQuality::Medium)
            .insert_resource(Volume(7))
            .init_state::<GameState>()
            .add_systems(Startup, setup)
            .add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin));
    }
}

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2d);
}

mod splash {
    use bevy::prelude::*;

    use super::GameState;
    use crate::components::game_menu::create_screen_node;

    pub fn splash_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), splash_setup)
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)));
    }

    create_components!(OnSplashScreen);

    #[derive(Resource, Deref, DerefMut)]
    struct SplashTimer(Timer);

    fn splash_setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
        let icon = asset_server.load("img/icon.png");
        cmds.spawn((
            DespawnOnExit(GameState::Splash),
            create_screen_node(),
            OnSplashScreen,
            children![(
                ImageNode::new(icon),
                Node {
                    width: px(200),
                    ..default()
                },
            )],
        ));
        cmds.insert_resource(SplashTimer(Timer::from_seconds(1., TimerMode::Once)));
    }

    fn countdown(
        mut game_state: ResMut<NextState<GameState>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>,
    ) {
        if timer.tick(time.delta()).is_finished() {
            game_state.set(GameState::Menu);
        }
    }
}

mod game {
    use bevy::{
        color::palettes::basic::{BLUE, LIME},
        prelude::*,
    };

    use super::{DisplayQuality, GameState, MAIN_TEXT_COLOR, Volume};
    use crate::components::game_menu::create_screen_node;

    pub fn game_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(Update, game.run_if(in_state(GameState::Game)));
    }

    create_components!(OnGameScreen);

    #[derive(Resource, Deref, DerefMut)]
    struct GameTimer(Timer);

    fn game_setup(
        mut commands: Commands,
        display_quality: Res<DisplayQuality>,
        volume: Res<Volume>,
    ) {
        fn game_screen(text: String, color: Color) -> (TextSpan, TextFont, TextColor) {
            (
                TextSpan(text),
                TextFont {
                    font_size: 50.,
                    ..default()
                },
                TextColor(color),
            )
        }
        commands.spawn((
            DespawnOnExit(GameState::Game),
            create_screen_node(),
            OnGameScreen,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::BLACK),
                children![
                    (
                        Text::new("Will be back to the menu shortly..."),
                        TextFont {
                            font_size: 67.,
                            ..default()
                        },
                        TextColor(MAIN_TEXT_COLOR),
                        Node {
                            margin: UiRect::all(px(50)),
                            ..default()
                        },
                    ),
                    (
                        Text::default(),
                        Node {
                            margin: UiRect::all(px(50)),
                            ..default()
                        },
                        children![
                            game_screen(format!("quality: {:?}", *display_quality), BLUE.into()),
                            game_screen("-".to_string(), MAIN_TEXT_COLOR),
                            game_screen(format!("volume: {:?}", *volume), LIME.into()),
                        ]
                    ),
                ]
            )],
        ));
        commands.insert_resource(GameTimer(Timer::from_seconds(5., TimerMode::Once)));
    }

    fn game(
        time: Res<Time>,
        mut game_state: ResMut<NextState<GameState>>,
        mut timer: ResMut<GameTimer>,
    ) {
        if timer.tick(time.delta()).is_finished() {
            game_state.set(GameState::Menu);
        }
    }
}

mod menu {
    use bevy::{
        app::AppExit,
        color::palettes::css::BLUE_VIOLET,
        ecs::spawn::{SpawnIter, SpawnWith},
        prelude::*,
    };

    use super::MAIN_TEXT_COLOR;
    use crate::components::game_menu::{
        DisplayQuality, GameState, Setting, Volume, create_screen_node,
    };

    pub fn menu_plugin(app: &mut App) {
        app.init_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), menu_setup)
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
            .add_systems(
                OnEnter(MenuState::SettingsDisplay),
                display_settings_menu_setup,
            )
            .add_systems(
                Update,
                (setting_button::<DisplayQuality>.run_if(in_state(MenuState::SettingsDisplay)),),
            )
            .add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
            .add_systems(
                Update,
                setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)),
            )
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(GameState::Menu)),
            );
    }

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, States)]
    enum MenuState {
        Main,
        Settings,
        SettingsDisplay,
        SettingsSound,
        #[default]
        Disabled,
    }

    create_components!(
        OnMainMenuScreen,
        OnSettingsMenuScreen,
        OnDisplaySettingsMenuScreen,
        OnSoundSettingsMenuScreen,
        SelectedOption
    );

    const NORMAL_BTN: Color = Color::srgb(0.15, 0.15, 0.45);
    const HOVERED_BTN: Color = Color::srgb(0.25, 0.25, 0.55);
    const HOVERED_PRESSED_BTN: Color = Color::srgb(0.25, 0.65, 0.55);
    const PRESSED_BTN: Color = Color::srgb(0.35, 0.75, 0.65);

    #[derive(Component)]
    enum MenuButtonAction {
        Play,
        Settings,
        SettingsDisplay,
        SettingsSound,
        BackToMainMenu,
        BackToSettings,
        Quit,
    }

    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut bg_color, selected) in &mut interaction_query {
            *bg_color = match (*interaction, selected) {
                (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BTN.into(),
                (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BTN.into(),
                (Interaction::Hovered, None) => HOVERED_BTN.into(),
                (Interaction::None, None) => NORMAL_BTN.into(),
            }
        }
    }

    fn setting_button<T: Resource + Component + PartialEq + Copy>(
        interaction_query: Query<
            (&Interaction, &Setting<T>, Entity),
            (Changed<Interaction>, With<Button>),
        >,
        selected_query: Single<(Entity, &mut BackgroundColor), With<SelectedOption>>,
        mut cmds: Commands,
        mut setting: ResMut<T>,
    ) {
        let (prev_btn, mut prev_btn_color) = selected_query.into_inner();

        for (interaction, btn_setting, entity) in &interaction_query {
            if *interaction == Interaction::Pressed && *setting != btn_setting.0 {
                *prev_btn_color = NORMAL_BTN.into();
                cmds.entity(prev_btn).remove::<SelectedOption>();
                cmds.entity(entity).insert(SelectedOption);
                *setting = btn_setting.0;
            }
        }
    }

    fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
        menu_state.set(MenuState::Main);
    }

    fn create_btn(w: f32, h: f32, m: f32) -> Node {
        Node {
            width: px(w),
            height: px(h),
            margin: UiRect::all(px(m)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }

    use bevy::ecs::spawn::SpawnRelatedBundle;

    fn main_menu_setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
        fn btn_icon_node() -> Node {
            Node {
                width: px(30),
                position_type: PositionType::Absolute,
                left: px(10),
                ..default()
            }
        }

        fn btn_text_font(text: String) -> (Text, TextFont, TextColor) {
            (
                Text::new(text),
                TextFont {
                    font_size: 33.,
                    ..default()
                },
                TextColor(MAIN_TEXT_COLOR),
            )
        }

        let right_icon: Handle<Image> = asset_server.load("img/right.png");
        let gear_icon: Handle<Image> = asset_server.load("img/gear.png");
        let exit_icon: Handle<Image> = asset_server.load("img/exit.png");

        fn create_menu_bt(
            action: MenuButtonAction,
            icon: Handle<Image>,
            text: String,
        ) -> (
            Button,
            Node,
            BackgroundColor,
            MenuButtonAction,
            SpawnRelatedBundle<
                ChildOf,
                (Spawn<(ImageNode, Node)>, Spawn<(Text, TextFont, TextColor)>),
            >,
        ) {
            (
                Button,
                create_btn(300., 65., 25.),
                BackgroundColor(NORMAL_BTN),
                action,
                children![(ImageNode::new(icon), btn_icon_node()), btn_text_font(text)],
            )
        }

        cmds.spawn((
            DespawnOnExit(MenuState::Main),
            create_screen_node(),
            OnMainMenuScreen,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(BLUE_VIOLET.into()),
                children![
                    (
                        Text::new("<GAME TITLE>"),
                        TextFont {
                            font_size: 66.,
                            ..default()
                        },
                        TextColor(MAIN_TEXT_COLOR),
                        Node {
                            margin: UiRect::all(px(50)),
                            ..default()
                        },
                    ),
                    create_menu_bt(MenuButtonAction::Play, right_icon, "Play".into(),),
                    create_menu_bt(MenuButtonAction::Settings, gear_icon, "Settings".into(),),
                    create_menu_bt(MenuButtonAction::Quit, exit_icon, "Quit".into(),),
                ]
            )],
        ));
    }

    fn settings_menu_setup(mut cmds: Commands) {
        let btn_node = create_btn(200., 65., 20.);

        let btn_text_style = (
            TextFont {
                font_size: 33.,
                ..default()
            },
            TextColor(MAIN_TEXT_COLOR),
        );

        cmds.spawn((
            DespawnOnExit(MenuState::Settings),
            create_screen_node(),
            OnSettingsMenuScreen,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(BLUE_VIOLET.into()),
                Children::spawn(SpawnIter(
                    [
                        (MenuButtonAction::SettingsDisplay, "Display"),
                        (MenuButtonAction::SettingsSound, "Sound"),
                        (MenuButtonAction::BackToMainMenu, "Back"),
                    ]
                    .into_iter()
                    .map(move |(action, text)| {
                        (
                            Button,
                            btn_node.clone(),
                            BackgroundColor(NORMAL_BTN),
                            action,
                            children![(Text::new(text), btn_text_style.clone())],
                        )
                    })
                ))
            )],
        ));
    }

    fn display_settings_menu_setup(mut cmds: Commands, display_quality: Res<DisplayQuality>) {
        fn btn_text_style() -> impl Bundle {
            (
                TextFont {
                    font_size: 33.,
                    ..default()
                },
                TextColor(MAIN_TEXT_COLOR),
            )
        }

        let display_quality = *display_quality;

        cmds.spawn((
            DespawnOnExit(MenuState::SettingsDisplay),
            create_screen_node(),
            OnDisplaySettingsMenuScreen,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(BLUE_VIOLET.into()),
                children![
                    (
                        Node {
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(BLUE_VIOLET.into()),
                        Children::spawn((
                            Spawn((Text::new("Display Quality"), btn_text_style())),
                            SpawnWith(move |parent: &mut ChildSpawner| {
                                for quality_settings in [
                                    DisplayQuality::Low,
                                    DisplayQuality::Medium,
                                    DisplayQuality::High,
                                ] {
                                    let mut entity = parent.spawn((
                                        Button,
                                        create_btn(150., 65., 0.),
                                        BackgroundColor(NORMAL_BTN),
                                        Setting(quality_settings),
                                        children![(
                                            Text::new(format!("{quality_settings:?}")),
                                            btn_text_style(),
                                        )],
                                    ));

                                    if display_quality == quality_settings {
                                        entity.insert(SelectedOption);
                                    }
                                }
                            })
                        ))
                    ),
                    (
                        Button,
                        create_btn(200., 65., 20.),
                        BackgroundColor(NORMAL_BTN),
                        MenuButtonAction::BackToSettings,
                        children![(Text::new("Back"), btn_text_style())]
                    )
                ]
            )],
        ));
    }

    fn sound_settings_menu_setup(mut cmds: Commands, volume: Res<Volume>) {
        let btn_node = create_btn(200., 65., 20.);

        let btn_text_style = (
            TextFont {
                font_size: 33.,
                ..default()
            },
            TextColor(MAIN_TEXT_COLOR),
        );

        let vol = *volume;
        let btn_node_clone = btn_node.clone();
        cmds.spawn((
            DespawnOnExit(MenuState::SettingsSound),
            create_screen_node(),
            OnSoundSettingsMenuScreen,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(BLUE_VIOLET.into()),
                children![
                    (
                        Node {
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(BLUE_VIOLET.into()),
                        Children::spawn((
                            Spawn((Text::new("Volume"), btn_text_style.clone())),
                            SpawnWith(move |parent: &mut ChildSpawner| {
                                for vol_setting in 0..10 {
                                    let mut entity = parent.spawn((
                                        Button,
                                        Node {
                                            width: px(30),
                                            height: px(65),
                                            ..btn_node_clone.clone()
                                        },
                                        BackgroundColor(NORMAL_BTN),
                                        Setting(Volume(vol_setting)),
                                    ));

                                    if vol == Volume(vol_setting) {
                                        entity.insert(SelectedOption);
                                    }
                                }
                            })
                        ))
                    ),
                    (
                        Button,
                        btn_node,
                        BackgroundColor(NORMAL_BTN),
                        MenuButtonAction::BackToSettings,
                        children![(Text::new("Back"), btn_text_style)]
                    )
                ]
            )],
        ));
    }

    fn menu_action(
        interaction_query: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_writer: MessageWriter<AppExit>,
        mut menu_state: ResMut<NextState<MenuState>>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        for (interaction, menu_btn_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_btn_action {
                    MenuButtonAction::Quit => {
                        app_exit_writer.write(AppExit::Success);
                    }
                    MenuButtonAction::Play => {
                        game_state.set(GameState::Game);
                        menu_state.set(MenuState::Disabled);
                    }
                    MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                    MenuButtonAction::SettingsDisplay => {
                        menu_state.set(MenuState::SettingsDisplay);
                    }
                    MenuButtonAction::SettingsSound => {
                        menu_state.set(MenuState::SettingsSound);
                    }
                    MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                    MenuButtonAction::BackToSettings => {
                        menu_state.set(MenuState::Settings);
                    }
                }
            }
        }
    }
}
