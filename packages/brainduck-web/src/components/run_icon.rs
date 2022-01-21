use yew::{function_component, html};

#[function_component(RunIcon)]
pub fn run_icon() -> Html {
    // From FontAwesome 6.0
    html! {
        <svg aria-hidden="true" focusable="false" data-prefix="fas" data-icon="play" class="svg-inline--fa fa-play fill-current" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M176 480C148.6 480 128 457.6 128 432v-352c0-25.38 20.4-47.98 48.01-47.98c8.686 0 17.35 2.352 25.02 7.031l288 176C503.3 223.8 512 239.3 512 256s-8.703 32.23-22.97 40.95l-288 176C193.4 477.6 184.7 480 176 480z"></path></svg>
    }
}
