use pelican_ui::{Component, Context, Plugins, Plugin, maverick_start, start, Application, PelicanEngine, MaverickOS};
use pelican_ui::drawable::{Drawable, Component};
use pelican_ui_std::{Stack, Interface, AppPage};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::events::{Event, OnEvent, TickEvent};
use pelican_ui::runtime::{Services, ServiceList};
use profiles::plugin::ProfilePlugin;
use profiles::service::{Name, ProfileService};
use profiles::components::AvatarContentProfiles;
use messages::plugin::MessagesPlugin;
use std::rc::Rc;
use std::cell::RefCell;

use bitcoin::service::BDKService;
use messages::service::{RoomsService, Rooms};

use bitcoin::pages::*;
use bitcoin::components::IconButtonBitcoin;
use messages::pages::*;
use messages::components::IconButtonMessages;
use profiles::pages::*;
use profiles::components::IconButtonProfiles;

// mod bdk;
// use bdk::BDKPlugin;
mod msg;
// use msg::MSGPlugin;
// use ucp_rust::UCPPlugin;

pub struct MyApp;
impl Services for MyApp {
    fn services() -> ServiceList {
        let mut services = ServiceList::default();
        services.insert::<ProfileService>();
        services.insert::<BDKService>();
        services.insert::<RoomsService>();
        services
    }
}

impl Plugins for MyApp {
    fn plugins(ctx: &mut Context) -> Vec<Box<dyn Plugin>> {
        vec![Box::new(ProfilePlugin::new(ctx)), Box::new(MessagesPlugin::new(ctx))]
    }
}

impl Application for MyApp {
    async fn new(ctx: &mut Context) -> Box<dyn Drawable> { App::new(ctx) }
}

start!(MyApp);

#[derive(Debug, Component)]
pub struct App(Stack, Interface);

impl App {
    pub fn new(ctx: &mut Context) -> Box<Self> {
        let account_actions = Rc::new(RefCell::new(vec![IconButtonBitcoin::new(ctx), IconButtonMessages::new(ctx), IconButtonProfiles::block(ctx)]));
        let messages_actions = account_actions.clone();
        let navigation = vec![
            ("wallet", "Bitcoin".to_string(), None, Some(Box::new(|ctx: &mut Context| Box::new(BitcoinHome::new(ctx)) as Box<dyn AppPage>) as Box<dyn FnMut(&mut Context) -> Box<dyn AppPage>>)),
            ("messages", "Messages".to_string(), None, Some(Box::new(move |ctx: &mut Context| Box::new(MessagesHome::new(ctx, messages_actions.clone())) as Box<dyn AppPage>) as Box<dyn FnMut(&mut Context) -> Box<dyn AppPage>>)),
            // ("door", "Rooms".to_string(), None, Some(Box::new(move |ctx: &mut Context| Box::new(RoomsHome::new(ctx, account_actions.clone())) as Box<dyn AppPage>) as Box<dyn FnMut(&mut Context) -> Box<dyn AppPage>>)),
        ];

        let navigation_b = vec![
            ("profile", "My Account".to_string(), Some(AvatarContentProfiles::default()), Some(Box::new(|ctx: &mut Context| Box::new(Account::new(ctx)) as Box<dyn AppPage>) as Box<dyn FnMut(&mut Context) -> Box<dyn AppPage>>))
        ];

        let home = BitcoinHome::new(ctx);
        let interface = Interface::new(ctx, Box::new(home), Some((0_usize, navigation, navigation_b)), None);
        Box::new(App(Stack::default(), interface))
    }
}

impl OnEvent for App {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            let rooms = &ctx.state().get_or_default::<Rooms>().0;
            let any_unread = rooms.iter().any(|r| r.1.2.iter().any(|m| !m.is_read()));
            if let Some(mobile) = self.1.mobile() {
                if let Some(n) = mobile.navigator().as_mut() { n.inner().buttons()[1].show_flair(any_unread); }
            } else if let Some(desktop) = self.1.desktop() {
                if let Some(n) = desktop.navigator().as_mut() { n.buttons()[1].show_flair_left(any_unread); }
            }

            if ctx.state().get::<Name>().is_some() {
                self.1.desktop().as_mut().map(|d| d.navigator().as_mut().map(|nav| {
                    let me = ProfilePlugin::me(ctx).0;
                    nav.update_avatar(AvatarContentProfiles::from_orange_name(ctx, &me));

                    // let username = ProfilePlugin::username(ctx);
                    // let username = NameGenerator::display_name(username);
                    // nav.update_username(username)
                }));
            }
        }
        true
    }
}