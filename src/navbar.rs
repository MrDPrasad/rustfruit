use yew::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <nav class="navbar">
            <div class="navbar-container">
                <a href="/" class="navbar-logo">
                    <svg class="logo-svg" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M12 2L2 7L12 12L22 7L12 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                        <path d="M2 17L12 22L22 17" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                        <path d="M2 12L12 17L22 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                    <span class="logo-text">{"LOGO"}</span>
                </a>
                <input type="checkbox" id="nav-toggle" class="nav-toggle" />
                <label for="nav-toggle" class="nav-toggle-label">
                    <span></span>
                </label>
                <ul class="nav-menu">
                    <li class="nav-item"><a href="/" class="nav-link">{"Home"}</a></li>
                    <li class="nav-item"><a href="/about" class="nav-link">{"About"}</a></li>
                    <li class="nav-item"><a href="/services" class="nav-link">{"Services"}</a></li>
                    <li class="nav-item"><a href="/contact" class="nav-link">{"Contact"}</a></li>
                </ul>
            </div>
        </nav>
    }
}