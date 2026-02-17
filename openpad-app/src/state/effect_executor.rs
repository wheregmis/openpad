use crate::state::actions::AppAction;
use crate::state::effects::StateEffect;
use makepad_widgets::Cx;

pub fn execute_state_effects(cx: &mut Cx, effects: Vec<StateEffect>) {
    for effect in effects {
        match effect {
            StateEffect::RequestSessionDiff {
                session_id,
                message_id,
            } => {
                cx.action(AppAction::RequestSessionDiff {
                    session_id,
                    message_id,
                });
            }
        }
    }
}
