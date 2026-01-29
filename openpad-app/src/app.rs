use makepad_widgets::*;

live_design! {
    use link::widgets::*;

    App = {{App}} {
        ui: <Root> {
            <Window> {
                body = <View> {}
            }
        }
    }
}

#[derive(Live, LiveHook)]
struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
