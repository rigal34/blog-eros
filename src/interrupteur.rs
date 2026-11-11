// L'interrupteur chromé en haut à droite : un clic éteint le site
// (fond noir, texte blanc), un autre clic le rallume.

use gloo_timers::callback::Timeout;
use yew::prelude::*;

// Un peu plus long que le changement de couleurs dans style.css (0,6 s)
const DUREE_CHANGEMENT: u32 = 700;

// Au départ, le site suit le réglage clair ou sombre de l'ordinateur du visiteur.
fn sombre_au_depart() -> bool {
    web_sys::window()
        .and_then(|f| f.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .map(|m| m.matches())
        .unwrap_or(false)
}

// Change les couleurs de tout le site, en douceur
fn appliquer(sombre: bool) {
    let Some(html) = web_sys::window()
        .and_then(|f| f.document())
        .and_then(|d| d.document_element())
    else {
        return;
    };
    let _ = html.set_attribute("data-theme", if sombre { "dark" } else { "light" });
    let _ = html.class_list().add_1("changement-theme");
    Timeout::new(DUREE_CHANGEMENT, move || {
        let _ = html.class_list().remove_1("changement-theme");
    })
    .forget();
}

#[component]
pub fn Interrupteur() -> Html {
    let sombre = use_state(sombre_au_depart);

    let onclick = {
        let sombre = sombre.clone();
        Callback::from(move |_: MouseEvent| {
            let nouveau = !*sombre;
            appliquer(nouveau);
            sombre.set(nouveau);
        })
    };

    let eteint = *sombre;
    html! {
        <button
            type="button"
            class={classes!("interrupteur", eteint.then_some("interrupteur-eteint"))}
            aria-label="Mode sombre"
            aria-pressed={if eteint { "true" } else { "false" }}
            title={if eteint { "Rallumer le site" } else { "Éteindre le site" }}
            {onclick}
        >
            // Le petit cadre chromé, avec le creux noir
            <img class="interrupteur-cadre" src="images/interrupteur-cadre.webp" alt="" width="123" height="210" />
            // Le bouton du milieu, dans ses deux positions : « I » enfoncé (allumé) ou « O » enfoncé (éteint)
            <img class="levier levier-allume" src="images/interrupteur-allume.webp" alt="" width="123" height="210" />
            <img class="levier levier-eteint" src="images/interrupteur-eteint.webp" alt="" width="123" height="210" />
        </button>
    }
}
